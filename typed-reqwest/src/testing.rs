//! Test utilities for `typed_reqwest::Request` implementations.

use crate::{BaseUrl, Request};

/// A test representation of an HTTP request for assertion purposes.
#[derive(Debug, PartialEq)]
pub struct TestRequest {
    pub method: reqwest::Method,
    pub url: url::Url,
    pub headers: http::HeaderMap,
    pub body: Option<Vec<u8>>,
}

impl TestRequest {
    /// Builds a `TestRequest` from a `Request` implementation.
    pub fn from_request<API, R: Request<API>>(request: &R, base_url: &BaseUrl) -> Self {
        let client = reqwest::Client::new();
        let built = request.request_builder(&client, base_url).build().unwrap();

        Self {
            method: built.method().clone(),
            url: built.url().clone(),
            headers: built.headers().clone(),
            body: built.body().and_then(|b| b.as_bytes().map(|b| b.to_vec())),
        }
    }

    /// Asserts that a `Request` implementation produces the expected `TestRequest`.
    #[track_caller]
    pub fn assert<API, R: Request<API>>(self, request: &R, base_url: &BaseUrl) {
        assert_eq!(Self::from_request(request, base_url), self);
    }
}
