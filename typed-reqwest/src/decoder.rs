//! HTTP response decoders.
//!
//! This module provides a builder-based API for defining how to decode
//! HTTP responses based on status code and content type.

type Result<T> = std::result::Result<T, DecodeError>;

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
    /// The underlying error, if any.
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.reason)
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|value| value.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl PartialEq for DecodeError {
    fn eq(&self, other: &Self) -> bool {
        self.reason == other.reason
    }
}

/// Specific reason for a decode error.
#[derive(Debug, PartialEq)]
pub enum ErrorReason {
    /// A required header has an invalid value.
    InvalidHeaderValue {
        /// The name of the header.
        name: http::header::HeaderName,
    },
    /// Body decoding failed.
    BodyDecodeError,
    /// Response is missing a required header.
    MissingHeader {
        /// The name of the missing header.
        name: http::header::HeaderName,
    },
    /// HTTP request failed.
    RequestError,
    /// Response has an unexpected Content-Type.
    UnexpectedContentType {
        /// The actual Content-Type received.
        content_type: String,
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
            Self::InvalidHeaderValue { name } => write!(formatter, "invalid {name} header value"),
            Self::BodyDecodeError => write!(formatter, "body decode error"),
            Self::MissingHeader { name } => write!(formatter, "missing {name} header"),
            Self::RequestError => write!(formatter, "request error"),
            Self::UnexpectedContentType { content_type } => {
                write!(formatter, "unexpected Content-Type: {content_type:?}")
            }
            Self::UnexpectedStatusCode { status_code } => {
                write!(formatter, "unexpected status code: {status_code}")
            }
        }
    }
}

/// Error returned when building a response decoder.
#[derive(Debug)]
pub enum ResponseBuilderError {
    /// A content type was registered more than once for the same status code.
    DuplicateContentType(mime::Mime),
}

impl std::fmt::Display for ResponseBuilderError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateContentType(mime) => {
                write!(formatter, "duplicate content type: {mime}")
            }
        }
    }
}

impl std::error::Error for ResponseBuilderError {}

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

        match self.map.get(&status_code) {
            Some(content_map) => Self::decode_content_type(content_map, response).await,
            None => Err(DecodeError {
                reason: ErrorReason::UnexpectedStatusCode { status_code },
                source: None,
            }),
        }
    }

    async fn decode_content_type(
        content_types: &ContentTypes<T>,
        response: reqwest::Response,
    ) -> Result<T> {
        let header_value = match response.headers().get(http::header::CONTENT_TYPE) {
            Some(value) => value,
            None => {
                return match content_types.default() {
                    Some(body_decoder) => Self::decode_body(body_decoder, response).await,
                    None => Err(DecodeError {
                        reason: ErrorReason::MissingHeader {
                            name: http::header::CONTENT_TYPE,
                        },
                        source: None,
                    }),
                };
            }
        };

        let content_type_str = header_value.to_str().map_err(|error| DecodeError {
            reason: ErrorReason::InvalidHeaderValue {
                name: http::header::CONTENT_TYPE,
            },
            source: Some(Box::new(error)),
        })?;

        let mime: mime::Mime = content_type_str.parse().map_err(|error| DecodeError {
            reason: ErrorReason::InvalidHeaderValue {
                name: http::header::CONTENT_TYPE,
            },
            source: Some(Box::new(error)),
        })?;

        match content_types.get(&mime) {
            Some(body_decoder) => Self::decode_body(body_decoder, response).await,
            None => Err(DecodeError {
                reason: ErrorReason::UnexpectedContentType {
                    content_type: content_type_str.to_owned(),
                },
                source: None,
            }),
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
                reason: ErrorReason::RequestError,
                source: Some(Box::new(error)),
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
    pub fn status_code(
        mut self,
        status_code: http::StatusCode,
        mut action: impl FnMut(&mut ContentTypes<T>) -> std::result::Result<(), ResponseBuilderError>,
    ) -> std::result::Result<Self, ResponseBuilderError> {
        let mut content_types = ContentTypes::new();

        action(&mut content_types)?;

        self.map.insert(status_code, content_types);

        Ok(self)
    }

    /// Adds a status code handler that deserializes JSON and maps it through a function.
    pub fn status_code_json_map<U: for<'de> serde::Deserialize<'de> + 'static>(
        self,
        status_code: http::StatusCode,
        function: fn(U) -> T,
    ) -> std::result::Result<Self, ResponseBuilderError> {
        self.status_code(status_code, |content_types| {
            content_types.json_map(function)
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
    pub fn status_code_json(
        self,
        status_code: http::StatusCode,
    ) -> std::result::Result<Self, ResponseBuilderError> {
        self.status_code(status_code, |content_types| content_types.json())
    }
}

impl<T: Clone + Send + Sync + 'static> ResponseBuilder<T> {
    /// Adds a status code handler that returns a constant value, ignoring the response body.
    ///
    /// Useful for testing request building without needing actual response parsing.
    pub fn status_code_constant(
        self,
        status_code: http::StatusCode,
        value: T,
    ) -> std::result::Result<Self, ResponseBuilderError> {
        self.status_code(status_code, |content_types| {
            content_types.constant(value.clone());
            Ok(())
        })
    }
}

/// Content type handlers for a specific status code.
pub struct ContentTypes<T> {
    default: Option<BodyDecoder<T>>,
    entries: Vec<(mime::Mime, BodyDecoder<T>)>,
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
            entries: Vec::new(),
        }
    }

    /// Adds a handler for a specific content type that only decodes the body.
    ///
    /// Matching is exact: `application/json` and `application/json; charset=utf-8`
    /// are distinct content types requiring separate registrations. This is
    /// deliberate so that body decoding can be specific to the registered
    /// content type variant (e.g. different charset handling).
    ///
    /// # Errors
    ///
    /// Returns `DuplicateContentType` if a handler is already registered for this content type.
    pub fn add(
        &mut self,
        content_type: mime::Mime,
        body_fn: impl Fn(&[u8]) -> Result<T> + Send + Sync + Copy + 'static,
    ) -> std::result::Result<(), ResponseBuilderError> {
        self.check_duplicate(&content_type)?;
        self.entries
            .push((content_type, BodyDecoder::body_only(body_fn)));
        Ok(())
    }

    /// Adds a handler with header extraction for a specific content type.
    ///
    /// Matching is exact: `application/json` and `application/json; charset=utf-8`
    /// are distinct content types requiring separate registrations. This is
    /// deliberate so that body decoding can be specific to the registered
    /// content type variant (e.g. different charset handling).
    ///
    /// # Errors
    ///
    /// Returns `DuplicateContentType` if a handler is already registered for this content type.
    pub fn add_with_headers<H: Send + 'static>(
        &mut self,
        content_type: mime::Mime,
        header_fn: impl Fn(&http::HeaderMap) -> Result<H> + Send + Sync + 'static,
        body_fn: impl Fn(H, &[u8]) -> Result<T> + Send + Sync + Copy + 'static,
    ) -> std::result::Result<(), ResponseBuilderError> {
        self.check_duplicate(&content_type)?;
        self.entries
            .push((content_type, BodyDecoder::new(header_fn, body_fn)));
        Ok(())
    }

    fn check_duplicate(
        &self,
        content_type: &mime::Mime,
    ) -> std::result::Result<(), ResponseBuilderError> {
        if self.entries.iter().any(|(mime, _)| mime == content_type) {
            Err(ResponseBuilderError::DuplicateContentType(
                content_type.clone(),
            ))
        } else {
            Ok(())
        }
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
    #[must_use]
    pub fn get(&self, content_type: &mime::Mime) -> Option<&BodyDecoder<T>> {
        self.entries
            .iter()
            .find(|(mime, _)| mime == content_type)
            .map(|(_, decoder)| decoder)
            .or(self.default.as_ref())
    }

    /// Gets the default body decoder for absent Content-Type header.
    #[must_use]
    pub fn default(&self) -> Option<&BodyDecoder<T>> {
        self.default.as_ref()
    }

    /// Adds a JSON handler that deserializes and maps through a function.
    ///
    /// # Errors
    ///
    /// Returns `DuplicateContentType` if a JSON handler is already registered.
    pub fn json_map<U: for<'de> serde::Deserialize<'de> + 'static>(
        &mut self,
        function: impl Fn(U) -> T + Copy + Send + Sync + 'static,
    ) -> std::result::Result<(), ResponseBuilderError> {
        self.add(mime::APPLICATION_JSON, move |body| json(body).map(function))
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
            entries: self
                .entries
                .into_iter()
                .map(|(content_type, decoder)| (content_type, decoder.paginated()))
                .collect(),
        }
    }
}

impl<T: for<'de> serde::Deserialize<'de> + 'static> ContentTypes<T> {
    /// Adds a handler for JSON content type that deserializes directly to `T`.
    ///
    /// # Errors
    ///
    /// Returns `DuplicateContentType` if a JSON handler is already registered.
    pub fn json(&mut self) -> std::result::Result<(), ResponseBuilderError> {
        self.add(mime::APPLICATION_JSON, json::<T>)
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
                reason: ErrorReason::InvalidHeaderValue {
                    name: http::header::LINK,
                },
                source: Some(Box::new(error)),
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
        reason: ErrorReason::BodyDecodeError,
        source: Some(Box::new(error)),
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

        assert_eq!(error.reason, ErrorReason::BodyDecodeError);
        assert!(error.source.is_some());
    }

    #[test]
    fn json_error_on_malformed() {
        let body = b"not json at all";
        let error = json::<User>(body).unwrap_err();

        assert_eq!(error.reason, ErrorReason::BodyDecodeError);
        assert!(error.source.is_some());
    }

    #[test]
    fn content_types_json_registers() {
        let mut content_types = ContentTypes::<User>::new();
        content_types.json().unwrap();

        assert!(content_types.get(&mime::APPLICATION_JSON).is_some());
    }

    #[test]
    fn content_types_json_does_not_match_with_charset() {
        let mut content_types = ContentTypes::<User>::new();
        content_types.json().unwrap();

        let with_charset: mime::Mime = "application/json; charset=utf-8".parse().unwrap();
        assert!(content_types.get(&with_charset).is_none());
    }

    #[test]
    fn content_types_duplicate_errors() {
        let mut content_types = ContentTypes::<User>::new();
        content_types.json().unwrap();

        assert!(matches!(
            content_types.json().unwrap_err(),
            ResponseBuilderError::DuplicateContentType(mime) if mime == mime::APPLICATION_JSON
        ));
    }

    #[test]
    fn content_types_fallback_to_default() {
        let mut content_types = ContentTypes::<String>::new();
        content_types.any(|body| Ok(String::from_utf8_lossy(body).to_string()));

        let decoder = content_types.get(&mime::TEXT_PLAIN).unwrap();
        let headers = http::HeaderMap::new();
        let body_decoder = decoder.decode_headers(&headers).unwrap();
        assert_eq!(body_decoder(b"hello").unwrap(), "hello");
    }

    #[test]
    fn content_types_no_match_without_default() {
        let content_types = ContentTypes::<User>::new();
        assert!(content_types.get(&mime::TEXT_PLAIN).is_none());
    }

    #[test]
    fn response_builder_creates_decoder() {
        let decoder: Response<User> = Response::build()
            .status_code_json(http::StatusCode::OK)
            .unwrap()
            .finish();

        assert!(decoder.map.contains_key(&http::StatusCode::OK));
    }

    #[tokio::test]
    async fn decode_missing_content_type_with_default() {
        let decoder: Response<String> = Response::build()
            .status_code(http::StatusCode::NO_CONTENT, |content_types| {
                content_types.constant("empty".to_string());
                Ok(())
            })
            .unwrap()
            .finish();

        let response: reqwest::Response = http::Response::builder()
            .status(http::StatusCode::NO_CONTENT)
            .body("")
            .unwrap()
            .into();

        let result = decoder.decode(response).await.unwrap();
        assert_eq!(result, "empty");
    }

    #[tokio::test]
    async fn decode_missing_content_type_without_default_errors() {
        let decoder: Response<User> = Response::build()
            .status_code_json(http::StatusCode::OK)
            .unwrap()
            .finish();

        let response: reqwest::Response = http::Response::builder()
            .status(http::StatusCode::OK)
            .body("")
            .unwrap()
            .into();

        let error = decoder.decode(response).await.unwrap_err();
        assert_eq!(
            error.reason,
            ErrorReason::MissingHeader {
                name: http::header::CONTENT_TYPE
            }
        );
    }
}
