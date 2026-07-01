# typed-reqwest - Typed HTTP Requests

> **Status**: Pre-1.0, API may change. Feedback welcome.

Turn any type into a typed HTTP [`reqwest`](https://docs.rs/reqwest) request with link header pagination.

## Overview

typed-reqwest provides a trait-based abstraction for HTTP clients that separates:

- **Request construction** - The `Request` trait turns any type into an HTTP request
- **Response decoding** - Declarative decoders based on status code and content-type
- **HTTP transport** - Left to the caller (uses reqwest for request building)

## Why does this exist?

Using reqwest directly leads to imperative code scattered across call sites:

```rust,ignore
let response = client.get(url).send().await?;
if response.status() == StatusCode::OK {
    let user: User = response.json().await?;
    // ...
} else if response.status() == StatusCode::NOT_FOUND {
    // ...
} else {
    panic!("unexpected status: {}", response.status());
}
```

Problems with this approach:

- **Requests are opaque** - You can't inspect, compare, or log a request before sending it
- **Response handling is imperative** - Status code checks are scattered and error-prone
- **No type connection** - Nothing ties a request to its expected response type
- **Hard to test** - Mocking requires intercepting HTTP calls

typed-reqwest solves these by making requests data:

- **Introspectable** - Request structs can derive `Debug`, `PartialEq`, `Clone`
- **Declarative decoding** - Response handling is defined once per request type
- **Type-safe** - `Request::Response` associates the response type at compile time
- **Testable** - Assert on request construction without HTTP, mock at the service layer

## Safer defaults than raw reqwest

`reqwest`'s convenience methods have several silent footguns. Decoding a response
through typed-reqwest addresses each of them by default:

| reqwest default (silent) | typed-reqwest default |
| --- | --- |
| **Any status is treated as success.** `send()` returns `Ok` for 4xx/5xx; turning a bad status into an error requires remembering `error_for_status()`. So `response.json()` on a `500` happily parses the error page into your success type. | You declare which status codes are valid and how each one decodes. An unexpected status fails with `ErrorReason::UnexpectedStatusCode`. |
| **`.json()` ignores `Content-Type`.** It runs `serde_json::from_slice` on whatever bytes arrive — an HTML error page, `text/plain`, anything. | Decoding negotiates on `Content-Type`; an unexpected or missing media type fails with `ErrorReason::UnexpectedContentType` / `ErrorReason::MissingHeader`. |
| **Bodies are buffered without limit.** `bytes()` / `json()` / `text()` collect the entire body into memory with no cap, so a hostile or buggy peer can OOM the process. | Buffered bodies are capped (default 10 MiB, configurable via `ResponseBuilder::buffered_body_max_size`) and enforced while streaming, so chunked and auto-decompressed bodies are bounded too. Overruns fail with `ErrorReason::BufferedBodyTooLarge`. |
| **The advertised size is not a bound.** `content_length()` is only a size hint; reqwest still reads whatever the connection streams. | When the response advertises a body size, that size becomes the read limit (capped at the maximum); a size advertised over the maximum is rejected before reading with `ErrorReason::DeclaredBodyTooLarge`. |
| **Rejected and ignored bodies are still fully downloaded.** | On a size violation, reading stops and the response is dropped — the rest of the oversized body is never downloaded. Decoders that ignore the body (such as constant decoders) drain it one chunk at a time without buffering. |

The default body cap is **10 MiB**, chosen to match the response payload ceiling
of common API gateways: AWS API Gateway (REST and HTTP APIs) and Apigee both cap
responses at ~10 MB, while AWS Lambda (6 MB) and ALB-to-Lambda (1 MB) sit below
it. A response that legitimately traversed such a gateway therefore never trips
the default, while still bounding the pathological case. Raise or lower it per
decoder with `ResponseBuilder::buffered_body_max_size`.

On any decode error — unexpected status, unexpected content type, or an oversized
body — the response is dropped rather than drained, so its connection is not
reused. This is intentional: a connection that produced a response we could not
accept is not trusted for reuse, and draining an oversized or hostile body would
mean downloading the very bytes we rejected. (A response whose body is read in
full but fails deserialization has already been consumed, so its connection
remains reusable.)

## Example

```rust
use typed_reqwest::{BaseUrl, Request, decoder};
use typed_reqwest::link::{Paginated, PaginatedRequest};

// API marker type - distinguishes requests for different APIs
struct GitHubApi;

// Response types - what the API returns
#[derive(Debug, serde::Deserialize)]
struct User {
    id: u64,
    login: String,
}

#[derive(Debug, serde::Deserialize)]
struct Repository {
    id: u64,
    name: String,
}

// Any type can become a request
struct GetUser {
    username: String,
}

// Implement Request to define how GetUser maps to HTTP
impl Request<GitHubApi> for GetUser {
    type Response = User;

    // Declarative decoder: expect 200 OK with JSON body
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
        client.get(base_url.set_path_segments(&["users", &self.username]))
    }
}

// Paginated request - returns data with Link header navigation
struct ListRepos {
    username: String,
}

impl Request<GitHubApi> for ListRepos {
    // Paginated wraps the response with parsed Link headers
    type Response = Paginated<Vec<Repository>>;

    // .paginated() wraps the decoder to parse Link headers
    decoder!(
        decoder::Response::build()
            .status_code_json(http::StatusCode::OK)
            .paginated()
    );

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &BaseUrl,
    ) -> reqwest::RequestBuilder {
        client.get(base_url.set_path_segments(&["users", &self.username, "repos"]))
    }
}

// Mark as supporting pagination to enable paginate() stream
impl PaginatedRequest for ListRepos {}

// Client wraps HTTP transport with authentication
struct Client {
    http: reqwest::Client,
    base_url: BaseUrl,
    token: String,
}

impl Client {
    pub fn new(token: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: BaseUrl::https(url::Host::parse("api.github.com").unwrap()),
            token,
        }
    }

    // Generic over any Request<GitHubApi> - one method handles all endpoints
    pub async fn execute<R: Request<GitHubApi>>(
        &self,
        request: R,
    ) -> Result<R::Response, decoder::DecodeError> {
        let response = request
            .request_builder(&self.http, &self.base_url)
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|error| decoder::DecodeError {
                reason: decoder::ErrorReason::RequestError,
                source: Some(Box::new(error)),
            })?;

        // DECODER is defined by the decoder!() macro on the Request impl
        R::DECODER.decode(response).await
    }
}

async fn example() -> Result<(), decoder::DecodeError> {
    let client = Client::new("ghp_xxxx".to_string());

    // Type-safe: GetUser returns User
    let user: User = client.execute(GetUser {
        username: "octocat".to_string(),
    }).await?;

    // Type-safe: ListRepos returns Paginated<Vec<Repository>>
    let repos: Paginated<Vec<Repository>> = client.execute(ListRepos {
        username: "octocat".to_string(),
    }).await?;

    // Access pagination links from response headers
    if let Some(links) = repos.links {
        if let Some(next_url) = links.next {
            println!("Next page: {next_url}");
        }
    }

    Ok(())
}
```

To preserve a trailing slash, include an empty segment. For example,
`set_path_segments(&["foo", ""])` yields `/foo/`.

```rust
# use typed_reqwest::BaseUrl;
# fn base_url() -> BaseUrl {
#     BaseUrl::https(url::Host::parse("api.example.com").unwrap())
# }
assert_eq!(
    base_url().set_path_segments(&["foo", ""]).as_str(),
    "https://api.example.com/foo/"
);
```

## Testing

Enable the `test-utils` feature for request assertion helpers:

```rust
# use typed_reqwest::{BaseUrl, Request, decoder};
# #[derive(Debug, serde::Deserialize)]
# struct User { id: u64, login: String }
# struct GitHubApi;
# struct GetUser { username: String }
# impl Request<GitHubApi> for GetUser {
#     type Response = User;
#     decoder!(decoder::Response::build().status_code_json(http::StatusCode::OK).finish());
#     fn request_builder(&self, client: &reqwest::Client, base_url: &BaseUrl) -> reqwest::RequestBuilder {
#         client.get(base_url.set_path_segments(&["users", &self.username]))
#     }
# }
use typed_reqwest::testing::TestRequest;

// Assert request construction without making HTTP calls
let base_url = BaseUrl::https(url::Host::parse("api.github.com").unwrap());

TestRequest {
    method: reqwest::Method::GET,
    url: "https://api.github.com/users/octocat".parse().unwrap(),
    headers: http::HeaderMap::new(),
    body: None,
}.assert(&GetUser { username: "octocat".to_string() }, &base_url);
```

## Tracing

Decoding emits `tracing` spans so you can see where decode time goes:

- `decode` (INFO) — the whole decode call, with a `status` field
- `buffer` (DEBUG) — reading the response body off the wire
- `deserialize` (DEBUG) — turning the bytes into your type, with a `bytes` field

typed-reqwest only performs the decode, so these nest under whatever span is
active when you call `decode`. Wrapping the request and the send in your own
spans yields the full tree:

```text
http_request        (yours)
├─ send             (yours, around .send().await)
└─ decode           (typed-reqwest, INFO)
   ├─ buffer        (DEBUG)
   └─ deserialize   (DEBUG)
```

`decode` is INFO so total decode time shows in a production trace alongside
`send`; enable `typed_reqwest=debug` to break it down into buffering versus
deserialization.
