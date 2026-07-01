//! HTTP response decoders.
//!
//! This module provides a builder-based API for defining how to decode
//! HTTP responses based on status code and content type.

use tracing::Instrument as _;

type Result<T> = std::result::Result<T, DecodeError>;

type HeaderValue = http::header::HeaderValue;
type StatusCode = http::StatusCode;

/// A body decoder with header data already extracted.
type BodyDecoderFn<T> = Box<dyn FnOnce(&[u8]) -> Result<T> + Send>;

/// Extracts header data and returns a body decoder that captures it.
type HeaderDecoderFn<T> = dyn Fn(&http::HeaderMap) -> Result<BodyDecoderFn<T>> + Send + Sync;

/// Default maximum size, in bytes, of a buffered response body.
///
/// Defaults to 10 MiB to match the response payload ceiling of common API
/// gateways: AWS API Gateway (REST and HTTP APIs) and Apigee both cap at ~10 MB,
/// while AWS Lambda (6 MB) and ALB (1 MB) sit below it. A response that
/// legitimately traversed such a gateway is therefore never rejected by this
/// default. Override via [`ResponseBuilder::buffered_body_max_size`].
const DEFAULT_BUFFERED_BODY_MAX_SIZE: usize = 10 * 1024 * 1024;

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
    /// The response body exceeded the effective size limit while it was being
    /// read.
    ///
    /// The effective limit is the size the response advertised, when it
    /// advertised one within the maximum, otherwise the configured maximum.
    BufferedBodyTooLarge {
        /// The configured maximum buffered size, in bytes.
        max_size: usize,
        /// The body size the response advertised, in bytes, if it advertised one.
        content_length: Option<u64>,
        /// The limit actually enforced while reading, in bytes.
        effective_limit: usize,
    },
    /// The response advertised a body size that exceeds the configured maximum
    /// buffered size, rejected before any of the body was read.
    DeclaredBodyTooLarge {
        /// The configured maximum buffered size, in bytes.
        max_size: usize,
        /// The body size the response advertised, in bytes.
        content_length: u64,
    },
    /// A required header has an invalid value.
    InvalidHeaderValue {
        /// The name of the header.
        name: http::header::HeaderName,
    },
    /// JSON deserialization failed.
    JsonDecodeError,
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
            Self::BufferedBodyTooLarge {
                max_size,
                content_length: Some(content_length),
                ..
            } => write!(
                formatter,
                "response body exceeded the {content_length} bytes it advertised \
                 (configured maximum {max_size} bytes)"
            ),
            Self::BufferedBodyTooLarge {
                max_size,
                content_length: None,
                ..
            } => write!(
                formatter,
                "response body exceeded the configured maximum of {max_size} bytes \
                 (response advertised no size)"
            ),
            Self::DeclaredBodyTooLarge {
                max_size,
                content_length,
            } => write!(
                formatter,
                "response advertised a body of {content_length} bytes, \
                 exceeding the configured maximum of {max_size} bytes"
            ),
            Self::InvalidHeaderValue { name } => write!(formatter, "invalid {name} header value"),
            Self::JsonDecodeError => write!(formatter, "JSON decode error"),
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

type StatusCodeMap<T> = std::collections::BTreeMap<StatusCode, ContentTypes<T>>;

/// A response decoder that maps status codes and content types to body decoders.
pub struct Response<T> {
    pub(crate) map: StatusCodeMap<T>,
    buffered_body_max_size: usize,
}

impl<T: 'static> Response<T> {
    /// Creates a new response decoder from a status code map.
    ///
    /// Buffered bodies are capped at 10 MiB; use
    /// [`ResponseBuilder::buffered_body_max_size`] to configure a different cap.
    #[must_use]
    pub const fn new(map: StatusCodeMap<T>) -> Self {
        Self {
            map,
            buffered_body_max_size: DEFAULT_BUFFERED_BODY_MAX_SIZE,
        }
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
    #[tracing::instrument(level = "info", skip_all, fields(status = response.status().as_u16()))]
    pub async fn decode(&self, response: reqwest::Response) -> Result<T> {
        let status_code = response.status();

        match self.map.get(&status_code) {
            Some(content_map) => {
                Self::decode_content_type(content_map, response, self.buffered_body_max_size).await
            }
            None => Err(DecodeError {
                reason: ErrorReason::UnexpectedStatusCode { status_code },
                source: None,
            }),
        }
    }

    async fn decode_content_type(
        content_types: &ContentTypes<T>,
        response: reqwest::Response,
        max_bytes: usize,
    ) -> Result<T> {
        match response.headers().get(http::header::CONTENT_TYPE) {
            None => match content_types.default() {
                Some(body_decoder) => Self::decode_body(body_decoder, response, max_bytes).await,
                None => Err(DecodeError {
                    reason: ErrorReason::MissingHeader {
                        name: http::header::CONTENT_TYPE,
                    },
                    source: None,
                }),
            },
            Some(content_type) => match content_types.get(content_type) {
                Some(body_decoder) => Self::decode_body(body_decoder, response, max_bytes).await,
                None => Err(DecodeError {
                    reason: ErrorReason::UnexpectedContentType {
                        content_type: content_type.clone(),
                    },
                    source: None,
                }),
            },
        }
    }

    async fn decode_body(
        body_decoder: &BodyDecoder<T>,
        response: reqwest::Response,
        max_bytes: usize,
    ) -> Result<T> {
        // Extract header data before consuming the response body.
        let decode_body = body_decoder.decode_headers(response.headers())?;

        match body_decoder.consumption {
            Consumption::Discard => {
                drain_body(response).await?;
                decode_body(&[])
            }
            Consumption::Buffered => {
                let body = buffer_body(response, max_bytes)
                    .instrument(tracing::debug_span!("buffer"))
                    .await?;

                tracing::debug!(
                    "Response body:\n{}",
                    std::str::from_utf8(&body).unwrap_or("<undecodable utf-8>")
                );

                tracing::debug_span!("deserialize", bytes = body.len())
                    .in_scope(|| decode_body(&body))
            }
        }
    }
}

/// Builder for constructing a `Response` decoder.
pub struct ResponseBuilder<T> {
    map: StatusCodeMap<T>,
    buffered_body_max_size: usize,
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
            buffered_body_max_size: DEFAULT_BUFFERED_BODY_MAX_SIZE,
        }
    }

    /// Sets the maximum size, in bytes, of a buffered response body.
    ///
    /// Bodies exceeding this size are rejected with
    /// [`ErrorReason::DeclaredBodyTooLarge`] (when the size is advertised up
    /// front) or [`ErrorReason::BufferedBodyTooLarge`] (when discovered while
    /// reading). Defaults to 10 MiB. Decoders that
    /// ignore the body (such as constant decoders) are unaffected.
    #[must_use]
    pub const fn buffered_body_max_size(mut self, size: usize) -> Self {
        self.buffered_body_max_size = size;
        self
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
        Response {
            map: self.map,
            buffered_body_max_size: self.buffered_body_max_size,
        }
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
        let buffered_body_max_size = self.buffered_body_max_size;

        let paginated_map = self
            .map
            .into_iter()
            .map(|(status_code, content_types)| (status_code, content_types.paginated()))
            .collect();

        Response {
            map: paginated_map,
            buffered_body_max_size,
        }
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

/// Matches a specific value of `T`, or anything (`*`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Match<T> {
    /// Matches a specific value.
    Exact(T),
    /// Matches anything (`*`).
    Any,
}

impl<T> From<T> for Match<T> {
    fn from(value: T) -> Self {
        Self::Exact(value)
    }
}

impl<T: AsRef<str>> Match<T> {
    /// Returns whether `value` matches, comparing case-insensitively.
    fn matches(&self, value: &str) -> bool {
        match self {
            Self::Exact(expected) => expected.as_ref().eq_ignore_ascii_case(value),
            Self::Any => true,
        }
    }
}

/// Error returned when a string is not a valid media type token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidToken {
    value: String,
}

impl std::fmt::Display for InvalidToken {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "invalid media type token: {:?}", self.value)
    }
}

impl std::error::Error for InvalidToken {}

/// Marker for a media type's primary (top-level) position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Primary;

/// Marker for a media type's secondary position (subtype).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Secondary;

/// A validated media type token at a [`Primary`] or [`Secondary`] position.
///
/// Build one with [`from_static_or_panic`](Self::from_static_or_panic) for a
/// literal, or via [`FromStr`](std::str::FromStr) (`"application".parse()`) for a fallible runtime
/// value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token<Position>(
    std::borrow::Cow<'static, str>,
    std::marker::PhantomData<Position>,
);

/// The primary (top-level) media type, e.g. `application`.
pub type PrimaryType = Token<Primary>;

/// The secondary media type (subtype), e.g. `json`.
pub type SecondaryType = Token<Secondary>;

impl<Position> Token<Position> {
    /// Creates a token from a static value.
    ///
    /// # Panics
    ///
    /// Panics if `value` is not a valid media type token. As a `const fn`, an
    /// invalid literal fails at compile time (e.g. in a `const` initializer);
    /// used at runtime it panics. Use [`FromStr`](std::str::FromStr) for a
    /// fallible alternative.
    #[must_use]
    pub const fn from_static_or_panic(value: &'static str) -> Self {
        assert!(is_token(value), "invalid media type token");
        Self(std::borrow::Cow::Borrowed(value), std::marker::PhantomData)
    }
}

impl PrimaryType {
    /// The `application` primary media type.
    pub const APPLICATION: Self = Self::from_static_or_panic("application");
    /// The `text` primary media type.
    pub const TEXT: Self = Self::from_static_or_panic("text");
}

impl SecondaryType {
    /// The `json` media subtype.
    pub const JSON: Self = Self::from_static_or_panic("json");
    /// The `plain` media subtype.
    pub const PLAIN: Self = Self::from_static_or_panic("plain");
}

impl<Position> std::str::FromStr for Token<Position> {
    type Err = InvalidToken;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        if is_token(value) {
            Ok(Self(
                std::borrow::Cow::Owned(value.to_owned()),
                std::marker::PhantomData,
            ))
        } else {
            Err(InvalidToken {
                value: value.to_owned(),
            })
        }
    }
}

impl<Position> AsRef<str> for Token<Position> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Matches a response `Content-Type` against an expected media type.
#[derive(Clone, Debug)]
pub enum ContentTypeMatcher {
    /// Matches `primary`/`secondary` (either of which may be
    /// [`Any`](Match::Any)), comparing case-insensitively and ignoring
    /// parameters such as `charset`. So `application`/`json` matches
    /// `application/json`, `application/json; charset=utf-8`, and
    /// `Application/JSON`.
    MediaType {
        /// The primary (top-level) type.
        primary: Match<PrimaryType>,
        /// The secondary type (subtype).
        secondary: Match<SecondaryType>,
    },
    /// Matches the raw header value exactly, byte for byte. For exotic content
    /// types where parameters or casing are significant.
    Exact(HeaderValue),
    /// Matches using a custom predicate over the raw header value.
    Custom(fn(&HeaderValue) -> bool),
}

impl ContentTypeMatcher {
    /// Returns whether `content_type` matches.
    #[must_use]
    pub fn matches(&self, content_type: &HeaderValue) -> bool {
        match self {
            Self::MediaType { primary, secondary } => content_type.to_str().is_ok_and(|value| {
                // Drop any parameters, then split the bare `type/subtype`.
                value
                    .split(';')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .split_once('/')
                    .is_some_and(|(response_primary, response_secondary)| {
                        primary.matches(response_primary) && secondary.matches(response_secondary)
                    })
            }),
            Self::Exact(expected) => content_type == expected,
            Self::Custom(predicate) => predicate(content_type),
        }
    }
}

/// Returns whether `value` is a valid media type token: non-empty and composed
/// only of RFC 7230 `tchar`s.
const fn is_token(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    let mut index = 0;
    while index < bytes.len() {
        if !is_tchar(bytes[index]) {
            return false;
        }
        index += 1;
    }
    true
}

/// Returns whether `byte` is an RFC 7230 `tchar`.
const fn is_tchar(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'!' | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | b'`'
                | b'|'
                | b'~'
        )
}

/// Ordered content-type matchers and their body decoders, tried first to last.
struct ContentTypeMatchers<T>(Vec<(ContentTypeMatcher, BodyDecoder<T>)>);

impl<T: 'static> ContentTypeMatchers<T> {
    const fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, matcher: ContentTypeMatcher, decoder: BodyDecoder<T>) {
        self.0.push((matcher, decoder));
    }

    fn resolve(&self, content_type: &HeaderValue) -> Option<&BodyDecoder<T>> {
        self.0
            .iter()
            .find(|(matcher, _)| matcher.matches(content_type))
            .map(|(_, decoder)| decoder)
    }
}

impl<T: 'static + Send + Sync> ContentTypeMatchers<T> {
    fn paginated(self) -> ContentTypeMatchers<crate::link::Paginated<T>> {
        ContentTypeMatchers(
            self.0
                .into_iter()
                .map(|(matcher, decoder)| (matcher, decoder.paginated()))
                .collect(),
        )
    }
}

/// Content type handlers for a specific status code.
///
/// Handlers are matched against the response `Content-Type` with a
/// [`ContentTypeMatcher`], so parameters such as `charset` and differences in
/// case or whitespace do not affect matching.
pub struct ContentTypes<T> {
    default: Option<BodyDecoder<T>>,
    matchers: ContentTypeMatchers<T>,
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
            matchers: ContentTypeMatchers::new(),
        }
    }

    /// Adds a body-only handler matched on a `primary`/`secondary` media type.
    pub fn add_match(
        &mut self,
        primary: impl Into<Match<PrimaryType>>,
        secondary: impl Into<Match<SecondaryType>>,
        body_fn: impl Fn(&[u8]) -> Result<T> + Send + Sync + Copy + 'static,
    ) {
        self.add(
            ContentTypeMatcher::MediaType {
                primary: primary.into(),
                secondary: secondary.into(),
            },
            body_fn,
        );
    }

    /// Adds a body-only handler for the given content-type matcher.
    pub fn add(
        &mut self,
        matcher: ContentTypeMatcher,
        body_fn: impl Fn(&[u8]) -> Result<T> + Send + Sync + Copy + 'static,
    ) {
        self.matchers.push(matcher, BodyDecoder::body_only(body_fn));
    }

    /// Adds a handler with header extraction for the given content-type matcher.
    pub fn add_with_headers<H: Send + 'static>(
        &mut self,
        matcher: ContentTypeMatcher,
        header_fn: impl Fn(&http::HeaderMap) -> Result<H> + Send + Sync + 'static,
        body_fn: impl Fn(H, &[u8]) -> Result<T> + Send + Sync + Copy + 'static,
    ) {
        self.matchers
            .push(matcher, BodyDecoder::new(header_fn, body_fn));
    }

    /// Adds a handler with header extraction matched on a `primary`/`secondary`
    /// media type (ignoring parameters such as `charset`).
    pub fn add_with_headers_match<H: Send + 'static>(
        &mut self,
        primary: impl Into<Match<PrimaryType>>,
        secondary: impl Into<Match<SecondaryType>>,
        header_fn: impl Fn(&http::HeaderMap) -> Result<H> + Send + Sync + 'static,
        body_fn: impl Fn(H, &[u8]) -> Result<T> + Send + Sync + Copy + 'static,
    ) {
        self.add_with_headers(
            ContentTypeMatcher::MediaType {
                primary: primary.into(),
                secondary: secondary.into(),
            },
            header_fn,
            body_fn,
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

    /// Gets the body decoder matching a content type, falling back to the
    /// default.
    pub fn get(&self, content_type: &HeaderValue) -> Option<&BodyDecoder<T>> {
        self.matchers
            .resolve(content_type)
            .or(self.default.as_ref())
    }

    /// Gets the default body decoder for absent Content-Type header.
    #[must_use]
    pub fn default(&self) -> Option<&BodyDecoder<T>> {
        self.default.as_ref()
    }

    /// Adds JSON handlers that deserialize and map through a function.
    pub fn json_map<U: for<'de> serde::Deserialize<'de> + 'static>(
        &mut self,
        function: impl Fn(U) -> T + Copy + Send + Sync + 'static,
    ) {
        self.add_match(PrimaryType::APPLICATION, SecondaryType::JSON, move |body| {
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
            matchers: self.matchers.paginated(),
        }
    }
}

impl<T: for<'de> serde::Deserialize<'de> + 'static> ContentTypes<T> {
    /// Adds handlers for JSON content types that deserialize directly to `T`.
    pub fn json(&mut self) {
        self.add_match(PrimaryType::APPLICATION, SecondaryType::JSON, json::<T>);
    }
}

/// How a response body is consumed before decoding.
///
/// This is owned by each [`BodyDecoder`] leaf because the consumption strategy
/// is a property of the decoder, not of the call site: a JSON decoder buffers
/// the body, while a constant decoder ignores it entirely.
#[derive(Clone, Copy)]
enum Consumption {
    /// Buffer the entire body into memory, then decode it.
    Buffered,
    /// Drain and discard the body without buffering it.
    ///
    /// Used by decoders that ignore the body but must still consume it so the
    /// underlying connection can be reused.
    Discard,
}

/// Decodes a response body with header extraction.
///
/// The decoder first extracts data from headers, then decodes the body
/// with the extracted data captured in a closure.
pub struct BodyDecoder<T> {
    consumption: Consumption,
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
            consumption: Consumption::Buffered,
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
            consumption: Consumption::Discard,
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
        let consumption = self.consumption;
        BodyDecoder {
            consumption,
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

/// Builds a `DecodeError` from a failed body read.
fn request_error(error: reqwest::Error) -> DecodeError {
    DecodeError {
        reason: ErrorReason::RequestError,
        source: Some(Box::new(error)),
    }
}

/// Buffers a response body into memory, bounded by the advertised body size when
/// known and never exceeding `max_bytes`.
///
/// When the body advertises a size (`content_length()`), that size becomes the
/// read limit: a size larger than `max_bytes` is rejected before reading, and a
/// body that streams beyond its advertised size is rejected while reading. When
/// the size is unknown (chunked or auto-decompressed), `max_bytes` is the limit.
///
/// The body is read one chunk at a time so the limit is enforced before the
/// oversized body becomes fully resident. On rejection the `response` is dropped
/// without reading the rest of the body, so the (untrusted, oversized)
/// connection is discarded rather than drained.
async fn buffer_body(mut response: reqwest::Response, max_bytes: usize) -> Result<Vec<u8>> {
    let content_length = response.content_length();

    // The size the response advertised, when present, is the read limit, capped
    // at the configured maximum. It is absent for chunked or auto-decompressed
    // bodies, in which case `max_bytes` is the only limit.
    let effective_limit = match content_length {
        Some(length) if length > max_bytes as u64 => {
            return Err(DecodeError {
                reason: ErrorReason::DeclaredBodyTooLarge {
                    max_size: max_bytes,
                    content_length: length,
                },
                source: None,
            });
        }
        // `length <= max_bytes` here, so the cast cannot truncate.
        Some(length) => length as usize,
        None => max_bytes,
    };

    // Deliberately not pre-reserved from the advertised size: it is not trusted
    // for allocation, only as a read bound, so memory grows with the bytes
    // actually received.
    let mut body = Vec::new();

    while let Some(chunk) = response.chunk().await.map_err(request_error)? {
        if body.len() + chunk.len() > effective_limit {
            return Err(DecodeError {
                reason: ErrorReason::BufferedBodyTooLarge {
                    max_size: max_bytes,
                    content_length,
                    effective_limit,
                },
                source: None,
            });
        }
        body.extend_from_slice(&chunk);
    }

    Ok(body)
}

/// Drains and discards a response body without buffering it.
///
/// Reads the body one chunk at a time and drops each chunk, keeping memory
/// usage bounded while still consuming the body so the underlying connection
/// can be reused.
async fn drain_body(mut response: reqwest::Response) -> Result<()> {
    while let Some(_chunk) = response.chunk().await.map_err(request_error)? {}

    Ok(())
}

/// Decodes a JSON response body.
///
/// # Errors
///
/// Returns a `DecodeError` if JSON deserialization fails.
pub fn json<T: for<'de> serde::Deserialize<'de>>(body: &[u8]) -> Result<T> {
    serde_json::from_slice(body).map_err(|error| DecodeError {
        reason: ErrorReason::JsonDecodeError,
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

        assert_eq!(error.reason, ErrorReason::JsonDecodeError);
        assert!(error.source.is_some());
    }

    #[test]
    fn json_error_on_malformed() {
        let body = b"not json at all";
        let error = json::<User>(body).unwrap_err();

        assert_eq!(error.reason, ErrorReason::JsonDecodeError);
        assert!(error.source.is_some());
    }

    #[test]
    fn content_types_json_matches_variants() {
        let mut content_types = ContentTypes::<User>::new();
        content_types.json();

        for value in [
            "application/json",
            "application/json; charset=utf-8",
            "application/json;charset=UTF-8",
            "Application/JSON",
        ] {
            assert!(
                content_types
                    .get(&HeaderValue::from_static(value))
                    .is_some(),
                "expected {value} to match",
            );
        }
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

    #[tokio::test]
    async fn decode_missing_content_type_with_default() {
        let decoder: Response<String> = Response::build()
            .status_code(http::StatusCode::NO_CONTENT, |content_types| {
                content_types.constant("empty".to_string());
            })
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

    #[tokio::test]
    async fn decode_buffered_decodes_body() {
        let decoder: Response<User> = Response::build()
            .status_code_json(http::StatusCode::OK)
            .finish();

        let response: reqwest::Response = http::Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(r#"{"id": 7, "name": "Bob"}"#)
            .unwrap()
            .into();

        let result = decoder.decode(response).await.unwrap();
        assert_eq!(
            result,
            User {
                id: 7,
                name: "Bob".to_string()
            }
        );
    }

    #[tokio::test]
    async fn decode_buffered_allows_body_within_limit() {
        let body = r#"{"id": 7, "name": "Bob"}"#;

        let decoder: Response<User> = Response::build()
            .buffered_body_max_size(body.len())
            .status_code_json(http::StatusCode::OK)
            .finish();

        let response: reqwest::Response = http::Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(body)
            .unwrap()
            .into();

        let result = decoder.decode(response).await.unwrap();
        assert_eq!(
            result,
            User {
                id: 7,
                name: "Bob".to_string()
            }
        );
    }

    #[tokio::test]
    async fn decode_buffered_rejects_declared_oversize_body() {
        let body = r#"{"id": 7, "name": "Bob"}"#;

        let decoder: Response<User> = Response::build()
            .buffered_body_max_size(8)
            .status_code_json(http::StatusCode::OK)
            .finish();

        let response: reqwest::Response = http::Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(body)
            .unwrap()
            .into();

        // In-memory responses report an exact body size, so the oversize body is
        // rejected up front by the declared-size check. The streaming
        // `BufferedBodyTooLarge` path requires a body of unknown size (chunked or
        // auto-decompressed) and is not reachable with an in-memory response.
        let error = decoder.decode(response).await.unwrap_err();
        assert_eq!(
            error.reason,
            ErrorReason::DeclaredBodyTooLarge {
                max_size: 8,
                content_length: body.len() as u64,
            }
        );
    }

    #[tokio::test]
    async fn decode_discard_ignores_non_empty_body() {
        let decoder: Response<String> = Response::build()
            .status_code_constant(http::StatusCode::OK, "constant".to_string())
            .finish();

        let response: reqwest::Response = http::Response::builder()
            .status(http::StatusCode::OK)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(r#"{"ignored": "payload"}"#)
            .unwrap()
            .into();

        let result = decoder.decode(response).await.unwrap();
        assert_eq!(result, "constant");
    }
}
