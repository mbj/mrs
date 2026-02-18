#![doc = include_str!("../README.md")]

pub mod decoder;
pub mod link;
#[cfg(feature = "test-utils")]
pub mod testing;

/// Macro to simplify declaring the `DECODER` constant in `Request` implementations.
///
/// # Example
///
/// ```rust
/// # use mhttp::{BaseUrl, Request, decoder};
/// # #[derive(Debug, serde::Deserialize)]
/// # struct User { id: u64 }
/// # struct MyApi;
/// # struct GetUser { id: u64 }
/// impl Request<MyApi> for GetUser {
///     type Response = User;
///
///     decoder!(
///         decoder::Response::build()
///             .status_code_json(http::StatusCode::OK)
///             .finish()
///     );
///
///     fn request_builder(&self, client: &reqwest::Client, base_url: &BaseUrl) -> reqwest::RequestBuilder {
///         let id = self.id.to_string();
///         client.get(base_url.set_path_segments(&["users", &id]))
///     }
/// }
/// ```
#[macro_export]
macro_rules! decoder {
    ($decoder:expr) => {
        const DECODER: std::sync::LazyLock<$crate::decoder::Response<Self::Response>> =
            std::sync::LazyLock::new(|| $decoder);
    };
}

/// A typed HTTP request.
///
/// Each API endpoint is represented as a struct implementing this trait.
/// The struct fields contain the request parameters, making requests
/// introspectable and comparable.
pub trait Request<API> {
    /// The response type after successful decoding.
    type Response: std::fmt::Debug + 'static;

    /// Decoder for parsing HTTP responses into the `Response` type.
    #[allow(clippy::declare_interior_mutable_const)]
    const DECODER: std::sync::LazyLock<decoder::Response<Self::Response>>;

    /// Builds a `reqwest::RequestBuilder` for this request.
    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &BaseUrl,
    ) -> reqwest::RequestBuilder;
}

/// Base URL origin for API requests (scheme + host + optional port).
#[derive(Clone, Debug)]
pub struct BaseUrl {
    origin: url::Url,
}

/// Supported URL schemes for base URLs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scheme {
    /// HTTP scheme.
    Http,
    /// HTTPS scheme.
    Https,
}

impl Scheme {
    fn as_str(self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
        }
    }
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl BaseUrl {
    /// Creates a new `BaseUrl` from the required URL components.
    ///
    /// `BaseUrl` stores only the origin; request builders always replace the path.
    #[must_use]
    pub fn new(scheme: Scheme, host: url::Host<String>, port: Option<u16>) -> Self {
        let mut origin =
            url::Url::parse(&format!("{scheme}://{host}")).expect("BaseUrl must be valid");
        if let Some(port) = port {
            origin
                .set_port(Some(port))
                .expect("BaseUrl must have a valid port");
        }
        Self { origin }
    }

    /// Creates a new HTTPS `BaseUrl`.
    #[must_use]
    pub fn https(host: url::Host<String>) -> Self {
        Self::new(Scheme::Https, host, None)
    }

    /// Creates a new HTTP `BaseUrl`.
    #[must_use]
    pub fn http(host: url::Host<String>) -> Self {
        Self::new(Scheme::Http, host, None)
    }

    /// Builds a URL by replacing the path with encoded path segments.
    ///
    /// Use an empty segment to preserve a trailing slash, for example
    /// `set_path_segments(&["foo", ""])` yields `/foo/`.
    #[must_use]
    pub fn set_path_segments(&self, segments: &[&str]) -> url::Url {
        let mut target = self.origin.clone();
        {
            let mut target_segments = target
                .path_segments_mut()
                .unwrap_or_else(|_| unreachable!("BaseUrl invariant violated"));
            target_segments.clear();
            target_segments.extend(segments.iter().copied());
        }
        target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn api_host() -> url::Host<String> {
        url::Host::parse("api.example.com").unwrap()
    }

    fn base_url() -> BaseUrl {
        BaseUrl::https(api_host())
    }

    #[test]
    fn base_url_path() {
        assert_eq!(
            base_url().set_path_segments(&["users", "123"]).as_str(),
            "https://api.example.com/users/123"
        );
    }

    #[test]
    fn base_url_path_includes_port() {
        let base = BaseUrl::new(Scheme::Https, api_host(), Some(8443));
        assert_eq!(
            base.set_path_segments(&["users", "123"]).as_str(),
            "https://api.example.com:8443/users/123"
        );
    }

    #[test]
    fn base_url_path_trailing_slash() {
        assert_eq!(
            base_url().set_path_segments(&["foo", ""]).as_str(),
            "https://api.example.com/foo/"
        );
    }

    #[test]
    fn base_url_path_encodes_segments() {
        assert_eq!(
            base_url().set_path_segments(&["foo/bar"]).as_str(),
            "https://api.example.com/foo%2Fbar"
        );
    }

    struct TestApi;

    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct User {
        id: u64,
        name: String,
    }

    struct GetUser {
        id: u64,
    }

    impl Request<TestApi> for GetUser {
        type Response = User;

        decoder!(
            decoder::Response::build()
                .status_code_json(http::StatusCode::OK)
                .finish()
        );

        fn request_builder(
            &self,
            client: &reqwest::Client,
            base_url: &BaseUrl,
        ) -> reqwest::RequestBuilder {
            let id = self.id.to_string();
            client.get(base_url.set_path_segments(&["users", &id]))
        }
    }

    #[test]
    fn decoder_macro() {
        let decoder = GetUser::DECODER;
        let decoder = &*decoder;
        let content_types = decoder
            .map
            .get(&http::StatusCode::OK)
            .expect("decoder should handle OK status");

        let body_decoder = content_types
            .get(&http::header::HeaderValue::from_static("application/json"))
            .expect("decoder should handle application/json");

        let headers = http::HeaderMap::new();
        let decode_body = body_decoder
            .decode_headers(&headers)
            .expect("header decoding should succeed");
        let user = decode_body(br#"{"id": 123, "name": "Alice"}"#)
            .expect("decoder should parse valid JSON");

        assert_eq!(
            user,
            User {
                id: 123,
                name: "Alice".to_string()
            }
        );
    }
}
