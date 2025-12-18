//! GitHub API client.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::cli_token::Token;

const USER_AGENT: &str = concat!(
    "greenhell/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/mbj/mrs/tree/main/greenhell)"
);

/// Maximum concurrent requests to GitHub API.
const MAX_CONCURRENT_REQUESTS: usize = 100;

/// GitHub API client with concurrency limiting.
#[derive(Clone)]
pub struct Client {
    inner: tower::limit::ConcurrencyLimit<HttpClient>,
}

impl Client {
    /// Default GitHub API base URL.
    pub const DEFAULT_BASE_URL: &str = "https://api.github.com";

    /// Creates a new GitHub API client.
    #[must_use]
    pub fn new(token: Token) -> Self {
        Self {
            inner: tower::limit::ConcurrencyLimit::new(
                HttpClient::new(
                    token,
                    mhttp::BaseUrl::new(Self::DEFAULT_BASE_URL.parse().unwrap()),
                ),
                MAX_CONCURRENT_REQUESTS,
            ),
        }
    }

    /// Creates a new GitHub API client with a custom base URL.
    ///
    /// Useful for GitHub Enterprise or testing with a mock server.
    #[must_use]
    pub fn with_base_url(token: Token, base_url: mhttp::BaseUrl) -> Self {
        Self {
            inner: tower::limit::ConcurrencyLimit::new(
                HttpClient::new(token, base_url),
                MAX_CONCURRENT_REQUESTS,
            ),
        }
    }
}

/// Error returned by the GitHub API client.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP request failed.
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    /// Response decoding failed.
    #[error("decode failed: {0}")]
    Decode(#[from] mhttp::decoder::DecodeError),
}

impl<R: mhttp::Request<Client> + Send + 'static> tower_service::Service<R> for Client {
    type Response = <R as mhttp::Request<Client>>::Response;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        tower_service::Service::<R>::poll_ready(&mut self.inner, context)
    }

    fn call(&mut self, request: R) -> Self::Future {
        use tower::ServiceExt;
        let inner = self.inner.clone();
        Box::pin(async move { inner.oneshot(request).await })
    }
}

/// Inner HTTP client that performs actual requests.
#[derive(Clone)]
struct HttpClient {
    base_url: mhttp::BaseUrl,
    http: reqwest::Client,
    token: Token,
}

impl HttpClient {
    fn new(token: Token, base_url: mhttp::BaseUrl) -> Self {
        Self {
            base_url,
            http: Self::build_http_client(),
            token,
        }
    }

    fn build_http_client() -> reqwest::Client {
        reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .default_headers({
                let mut headers = http::HeaderMap::new();
                headers.insert(
                    http::header::ACCEPT,
                    http::header::HeaderValue::from_static("application/vnd.github+json"),
                );
                headers.insert(
                    http::HeaderName::from_static("x-github-api-version"),
                    http::header::HeaderValue::from_static("2022-11-28"),
                );
                headers
            })
            .build()
            .unwrap()
    }
}

impl<R: mhttp::Request<Client> + 'static> tower_service::Service<R> for HttpClient {
    type Response = <R as mhttp::Request<Client>>::Response;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: R) -> Self::Future {
        let http_request = request
            .request_builder(&self.http, &self.base_url)
            .bearer_auth(self.token.as_str())
            .build()
            .unwrap();

        let http = self.http.clone();

        Box::pin(async move {
            log::info!("{} {}", http_request.method(), http_request.url());

            let response = http.execute(http_request).await.map_err(Error::Request)?;

            let decoder = R::DECODER;
            decoder.decode(response).await.map_err(Error::Decode)
        })
    }
}
