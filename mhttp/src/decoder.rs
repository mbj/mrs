//! HTTP response decoders.
//!
//! This module provides a builder-based API for defining how to decode
//! HTTP responses based on status code and content type.

type Result<T> = std::result::Result<T, DecodeError>;

type HeaderValue = http::header::HeaderValue;
type StatusCode = http::StatusCode;

/// A body decoder with header data already extracted.
type BodyDecoderFn<T> = Box<dyn FnOnce(&[u8]) -> Result<T> + Send>;

/// Extracts header data and returns a body decoder that captures it.
type HeaderDecoderFn<T> = dyn Fn(&http::HeaderMap) -> Result<BodyDecoderFn<T>> + Send + Sync;

/// Error when decoding an HTTP response.
#[derive(Debug)]
pub struct DecodeError {
    /// The reason for the decode failure.
    pub reason: ErrorReason,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.reason)
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.reason.source()
    }
}

impl PartialEq for DecodeError {
    fn eq(&self, other: &Self) -> bool {
        self.reason == other.reason
    }
}

/// Specific reason for a decode error.
#[derive(Debug)]
pub enum ErrorReason {
    /// Link header parsing failed.
    InvalidLinkHeader(crate::link::ParseError),
    /// JSON deserialization failed.
    JsonDecodeError(serde_json::Error),
    /// Response is missing the Content-Type header.
    MissingContentTypeHeader,
    /// HTTP request failed.
    RequestError {
        /// The underlying reqwest error.
        error: reqwest::Error,
    },
    /// Response has an unexpected Content-Type.
    UnexpectedContentType {
        /// The actual Content-Type received.
        content_type: http::header::HeaderValue,
    },
    /// Response has an unexpected status code.
    UnexpectedStatusCode {
        /// The actual status code received.
        status_code: http::StatusCode,
    },
}

impl std::fmt::Display for ErrorReason {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLinkHeader(error) => write!(formatter, "invalid link header: {error}"),
            Self::JsonDecodeError(error) => write!(formatter, "JSON decode error: {error}"),
            Self::MissingContentTypeHeader => write!(formatter, "missing Content-Type header"),
            Self::RequestError { error } => write!(formatter, "request error: {error}"),
            Self::UnexpectedContentType { content_type } => {
                write!(formatter, "unexpected Content-Type: {content_type:?}")
            }
            Self::UnexpectedStatusCode { status_code } => {
                write!(formatter, "unexpected status code: {status_code}")
            }
        }
    }
}

impl ErrorReason {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidLinkHeader(error) => Some(error),
            Self::JsonDecodeError(error) => Some(error),
            Self::RequestError { error } => Some(error),
            Self::MissingContentTypeHeader
            | Self::UnexpectedContentType { .. }
            | Self::UnexpectedStatusCode { .. } => None,
        }
    }
}

impl PartialEq for ErrorReason {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::InvalidLinkHeader(left), Self::InvalidLinkHeader(right)) => left == right,
            (Self::JsonDecodeError(left), Self::JsonDecodeError(right)) => {
                left.to_string() == right.to_string()
            }
            (Self::MissingContentTypeHeader, Self::MissingContentTypeHeader) => true,
            (Self::RequestError { error: left }, Self::RequestError { error: right }) => {
                left.to_string() == right.to_string()
            }
            (
                Self::UnexpectedContentType { content_type: left },
                Self::UnexpectedContentType {
                    content_type: right,
                },
            ) => left == right,
            (
                Self::UnexpectedStatusCode { status_code: left },
                Self::UnexpectedStatusCode { status_code: right },
            ) => left == right,
            _ => false,
        }
    }
}

type StatusCodeMap<T> = std::collections::BTreeMap<StatusCode, ContentTypes<T>>;

/// A response decoder that maps status codes and content types to body decoders.
pub struct Response<T> {
    pub(crate) map: StatusCodeMap<T>,
}

impl<T: 'static> Response<T> {
    /// Creates a new response decoder from a status code map.
    #[must_use]
    pub const fn new(map: StatusCodeMap<T>) -> Self {
        Self { map }
    }

    /// Returns a builder for constructing a response decoder.
    #[must_use]
    pub const fn build() -> ResponseBuilder<T> {
        ResponseBuilder::<T>::new()
    }

    /// Decodes an HTTP response.
    ///
    /// # Errors
    ///
    /// Returns a `DecodeError` if:
    /// - The status code is not in the decoder's map
    /// - The content type is missing or not in the decoder's map
    /// - Body deserialization fails
    pub async fn decode(&self, response: reqwest::Response) -> Result<T> {
        let status_code = response.status();

        log::info!("Status: {status_code}");

        match self.map.get(&status_code) {
            Some(content_map) => Self::decode_content_type(content_map, response).await,
            None => Err(DecodeError {
                reason: ErrorReason::UnexpectedStatusCode { status_code },
            }),
        }
    }

    async fn decode_content_type(
        content_types: &ContentTypes<T>,
        response: reqwest::Response,
    ) -> Result<T> {
        match response.headers().get(http::header::CONTENT_TYPE) {
            None => Err(DecodeError {
                reason: ErrorReason::MissingContentTypeHeader,
            }),
            Some(content_type) => match content_types.get(content_type) {
                Some(body_decoder) => Self::decode_body(body_decoder, response).await,
                None => Err(DecodeError {
                    reason: ErrorReason::UnexpectedContentType {
                        content_type: content_type.clone(),
                    },
                }),
            },
        }
    }

    async fn decode_body(body_decoder: &BodyDecoder<T>, response: reqwest::Response) -> Result<T> {
        // Extract header data before consuming response for bytes
        let decode_body = body_decoder.decode_headers(response.headers())?;

        match response.bytes().await {
            Ok(bytes) => {
                log::debug!(
                    "Response body:\n{}",
                    std::str::from_utf8(bytes.as_ref()).unwrap_or("<undecodable utf-8>")
                );
                decode_body(bytes.as_ref())
            }
            Err(error) => Err(DecodeError {
                reason: ErrorReason::RequestError { error },
            }),
        }
    }
}

/// Builder for constructing a `Response` decoder.
pub struct ResponseBuilder<T> {
    map: StatusCodeMap<T>,
}

impl<T: 'static> Default for ResponseBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: 'static> ResponseBuilder<T> {
    /// Creates a new empty response builder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            map: StatusCodeMap::new(),
        }
    }

    /// Adds a status code handler with custom content type configuration.
    #[must_use]
    pub fn status_code(
        mut self,
        status_code: http::StatusCode,
        mut action: impl FnMut(&mut ContentTypes<T>),
    ) -> Self {
        let mut content_types = ContentTypes::new();

        action(&mut content_types);

        self.map.insert(status_code, content_types);

        self
    }

    /// Adds a status code handler that deserializes JSON and maps it through a function.
    #[must_use]
    pub fn status_code_json_map<U: for<'de> serde::Deserialize<'de> + 'static>(
        self,
        status_code: http::StatusCode,
        function: fn(U) -> T,
    ) -> Self {
        self.status_code(status_code, |content_types| {
            content_types.json_map(function);
        })
    }

    /// Finishes building and returns the response decoder.
    #[must_use]
    pub fn finish(self) -> Response<T> {
        Response::new(self.map)
    }

    /// Finishes building and returns a paginated response decoder.
    ///
    /// The resulting decoder parses the Link header to extract pagination links
    /// and wraps the response data in `Paginated<T>`.
    #[must_use]
    pub fn paginated(self) -> Response<crate::link::Paginated<T>>
    where
        T: Send + Sync,
    {
        let paginated_map = self
            .map
            .into_iter()
            .map(|(status_code, content_types)| (status_code, content_types.paginated()))
            .collect();

        Response::new(paginated_map)
    }
}

impl<T: for<'de> serde::Deserialize<'de> + 'static> ResponseBuilder<T> {
    /// Adds a status code handler that deserializes JSON directly to `T`.
    #[must_use]
    pub fn status_code_json(self, status_code: http::StatusCode) -> Self {
        self.status_code(status_code, ContentTypes::json)
    }
}

impl<T: Clone + Send + Sync + 'static> ResponseBuilder<T> {
    /// Adds a status code handler that returns a constant value, ignoring the response body.
    ///
    /// Useful for testing request building without needing actual response parsing.
    #[must_use]
    pub fn status_code_constant(self, status_code: http::StatusCode, value: T) -> Self {
        self.status_code(status_code, |content_types| {
            content_types.constant(value.clone());
        })
    }
}

/// Content type handlers for a specific status code.
pub struct ContentTypes<T> {
    default: Option<BodyDecoder<T>>,
    map: std::collections::BTreeMap<HeaderValue, BodyDecoder<T>>,
}

impl<T: 'static> Default for ContentTypes<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: 'static> ContentTypes<T> {
    /// Creates a new empty content types handler.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            default: None,
            map: std::collections::BTreeMap::new(),
        }
    }

    /// Adds a handler for a specific content type that only decodes the body.
    pub fn add(
        &mut self,
        content_type: &'static str,
        body_fn: impl Fn(&[u8]) -> Result<T> + Send + Sync + Copy + 'static,
    ) {
        self.map.insert(
            HeaderValue::from_static(content_type),
            BodyDecoder::body_only(body_fn),
        );
    }

    /// Adds a handler with header extraction for a specific content type.
    pub fn add_with_headers<H: Send + 'static>(
        &mut self,
        content_type: &'static str,
        header_fn: impl Fn(&http::HeaderMap) -> Result<H> + Send + Sync + 'static,
        body_fn: impl Fn(H, &[u8]) -> Result<T> + Send + Sync + Copy + 'static,
    ) {
        self.map.insert(
            HeaderValue::from_static(content_type),
            BodyDecoder::new(header_fn, body_fn),
        );
    }

    /// Sets a default handler for any content type.
    pub fn any(&mut self, body_fn: impl Fn(&[u8]) -> Result<T> + Send + Sync + Copy + 'static) {
        self.default = Some(BodyDecoder::body_only(body_fn));
    }

    /// Sets a default handler that returns a constant value for any content type.
    pub fn constant(&mut self, value: T)
    where
        T: Clone + Send + Sync,
    {
        self.default = Some(BodyDecoder::constant(value));
    }

    /// Gets the body decoder for a content type, falling back to the default.
    pub fn get(&self, header_value: &HeaderValue) -> Option<&BodyDecoder<T>> {
        self.map
            .get(header_value)
            .map_or(self.default.as_ref(), Some)
    }

    /// Adds JSON handlers that deserialize and map through a function.
    pub fn json_map<U: for<'de> serde::Deserialize<'de> + 'static>(
        &mut self,
        function: impl Fn(U) -> T + Copy + Send + Sync + 'static,
    ) {
        self.add("application/json", move |body| json(body).map(function));
        self.add("application/json; charset=utf-8", move |body| {
            json(body).map(function)
        });
    }

    /// Transforms this content type handler into a paginated version.
    ///
    /// Each body decoder is wrapped to parse the Link header and return `Paginated<T>`.
    fn paginated(self) -> ContentTypes<crate::link::Paginated<T>>
    where
        T: Send + Sync,
    {
        ContentTypes {
            default: self.default.map(BodyDecoder::paginated),
            map: self
                .map
                .into_iter()
                .map(|(content_type, decoder)| (content_type, decoder.paginated()))
                .collect(),
        }
    }
}

impl<T: for<'de> serde::Deserialize<'de> + 'static> ContentTypes<T> {
    /// Adds handlers for JSON content types that deserialize directly to `T`.
    pub fn json(&mut self) {
        self.add("application/json", json::<T>);
        self.add("application/json; charset=utf-8", json::<T>);
    }
}

/// Decodes a response body with header extraction.
///
/// The decoder first extracts data from headers, then decodes the body
/// with the extracted data captured in a closure.
pub struct BodyDecoder<T> {
    header_decoder: Box<HeaderDecoderFn<T>>,
}

impl<T: 'static> BodyDecoder<T> {
    /// Creates a body decoder from separate header and body decoding functions.
    ///
    /// The header function extracts owned data from headers, which is then
    /// passed to the body function along with the response bytes.
    pub fn new<H: Send + 'static>(
        header_fn: impl Fn(&http::HeaderMap) -> Result<H> + Send + Sync + 'static,
        body_fn: impl Fn(H, &[u8]) -> Result<T> + Send + Sync + Copy + 'static,
    ) -> Self {
        Self {
            header_decoder: Box::new(move |headers| {
                let header_data = header_fn(headers)?;
                Ok(Box::new(move |body| body_fn(header_data, body)))
            }),
        }
    }

    /// Creates a body decoder that ignores headers.
    pub fn body_only(body_fn: impl Fn(&[u8]) -> Result<T> + Send + Sync + Copy + 'static) -> Self {
        Self::new(|_headers| Ok(()), move |(), body| body_fn(body))
    }

    /// Creates a body decoder that returns a constant value.
    pub fn constant(value: T) -> Self
    where
        T: Clone + Send + Sync,
    {
        let value = std::sync::Arc::new(value);

        Self {
            header_decoder: Box::new(move |_headers| {
                let value = value.clone();
                Ok(Box::new(move |_body| Ok((*value).clone())))
            }),
        }
    }

    /// Extracts header data and returns a body decoder.
    pub(crate) fn decode_headers(&self, headers: &http::HeaderMap) -> Result<BodyDecoderFn<T>> {
        (self.header_decoder)(headers)
    }
}

impl<T: 'static + Send + Sync> BodyDecoder<T> {
    /// Wraps this decoder to return paginated results.
    ///
    /// Parses the Link header and combines it with the decoded body.
    fn paginated(self) -> BodyDecoder<crate::link::Paginated<T>> {
        BodyDecoder {
            header_decoder: Box::new(move |headers| {
                let links = parse_link_header(headers)?;
                let body_decoder = (self.header_decoder)(headers)?;

                Ok(Box::new(move |body| {
                    let data = body_decoder(body)?;
                    Ok(crate::link::Paginated { data, links })
                }))
            }),
        }
    }
}

fn parse_link_header(headers: &http::HeaderMap) -> Result<Option<crate::link::Links>> {
    match headers.get(http::header::LINK) {
        Some(value) => match crate::link::Links::from_header_value(value) {
            Ok(links) => Ok(Some(links)),
            Err(error) => Err(DecodeError {
                reason: ErrorReason::InvalidLinkHeader(error),
            }),
        },
        None => Ok(None),
    }
}

/// Decodes a JSON response body.
///
/// # Errors
///
/// Returns a `DecodeError` if JSON deserialization fails.
pub fn json<T: for<'de> serde::Deserialize<'de>>(body: &[u8]) -> Result<T> {
    serde_json::from_slice(body).map_err(|error| DecodeError {
        reason: ErrorReason::JsonDecodeError(error),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct User {
        id: u64,
        name: String,
    }

    #[test]
    fn json_decodes_valid() {
        let body = br#"{"id": 123, "name": "Alice"}"#;
        let user: User = json(body).unwrap();
        assert_eq!(
            user,
            User {
                id: 123,
                name: "Alice".to_string()
            }
        );
    }

    #[test]
    fn json_error_on_invalid() {
        let body = br#"{"id": "not a number"}"#;
        let error = json::<User>(body).unwrap_err();

        // Parse the same invalid JSON to get the same serde error
        let expected_serde_error = serde_json::from_slice::<User>(body).unwrap_err();
        assert_eq!(
            error,
            DecodeError {
                reason: ErrorReason::JsonDecodeError(expected_serde_error)
            }
        );
    }

    #[test]
    fn json_error_on_malformed() {
        let body = b"not json at all";
        let error = json::<User>(body).unwrap_err();

        let expected_serde_error = serde_json::from_slice::<User>(body).unwrap_err();
        assert_eq!(
            error,
            DecodeError {
                reason: ErrorReason::JsonDecodeError(expected_serde_error)
            }
        );
    }

    #[test]
    fn content_types_json_registers_both_variants() {
        let mut content_types = ContentTypes::<User>::new();
        content_types.json();

        assert!(
            content_types
                .get(&HeaderValue::from_static("application/json"))
                .is_some()
        );
        assert!(
            content_types
                .get(&HeaderValue::from_static("application/json; charset=utf-8"))
                .is_some()
        );
    }

    #[test]
    fn content_types_fallback_to_default() {
        let mut content_types = ContentTypes::<String>::new();
        content_types.any(|body| Ok(String::from_utf8_lossy(body).to_string()));

        let decoder = content_types
            .get(&HeaderValue::from_static("text/plain"))
            .unwrap();
        let headers = http::HeaderMap::new();
        let body_decoder = decoder.decode_headers(&headers).unwrap();
        assert_eq!(body_decoder(b"hello").unwrap(), "hello");
    }

    #[test]
    fn content_types_no_match_without_default() {
        let content_types = ContentTypes::<User>::new();
        assert!(
            content_types
                .get(&HeaderValue::from_static("text/plain"))
                .is_none()
        );
    }

    #[test]
    fn response_builder_creates_decoder() {
        let decoder: Response<User> = Response::build()
            .status_code_json(http::StatusCode::OK)
            .finish();

        assert!(decoder.map.contains_key(&http::StatusCode::OK));
    }
}
