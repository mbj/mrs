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
///         client.get(base_url.set_path(&format!("/users/{}", self.id)))
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

/// Base URL for API requests.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct BaseUrl(url::Url);

impl BaseUrl {
    /// Creates a new `BaseUrl` from a `url::Url`.
    #[must_use]
    pub fn new(url: url::Url) -> Self {
        Self(url)
    }

    /// Builds a URL by replacing the path component of the base URL.
    #[must_use]
    pub fn set_path(&self, path: &str) -> url::Url {
        let mut target = self.0.clone();
        target.set_path(path);
        target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_url() -> BaseUrl {
        BaseUrl::new("https://api.example.com".parse().unwrap())
    }

    #[test]
    fn base_url_path() {
        assert_eq!(
            base_url().set_path("/users/123").as_str(),
            "https://api.example.com/users/123"
        );
    }

    #[test]
    fn base_url_path_replaces_existing() {
        let base = BaseUrl::new("https://api.example.com/v1".parse().unwrap());
        assert_eq!(
            base.set_path("/users/123").as_str(),
            "https://api.example.com/users/123"
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
            client.get(base_url.set_path(&format!("/users/{}", self.id)))
        }
    }

    #[test]
    fn decoder_macro() {
        let decoder = &*GetUser::DECODER;
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
