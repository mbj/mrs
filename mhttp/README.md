# mhttp

Turn any type into a typed HTTP request with declarative response decoding.

## Overview

mhttp provides a trait-based abstraction for HTTP clients that separates:

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

mhttp solves these by making requests data:

- **Introspectable** - Request structs can derive `Debug`, `PartialEq`, `Clone`
- **Declarative decoding** - Response handling is defined once per request type
- **Type-safe** - `Request::Response` associates the response type at compile time
- **Testable** - Assert on request construction without HTTP, mock at the service layer

## Example

```rust
use mhttp::{BaseUrl, Request, decoder};
use mhttp::link::{Paginated, PaginatedRequest};

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
        client.get(base_url.set_path(&format!("/users/{}", self.username)))
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
        client.get(base_url.set_path(&format!("/users/{}/repos", self.username)))
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
            base_url: BaseUrl::new("https://api.github.com".parse().unwrap()),
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
                reason: decoder::ErrorReason::RequestError { error },
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

## Testing

Enable the `test-utils` feature for request assertion helpers:

```rust
# use mhttp::{BaseUrl, Request, decoder};
# #[derive(Debug, serde::Deserialize)]
# struct User { id: u64, login: String }
# struct GitHubApi;
# struct GetUser { username: String }
# impl Request<GitHubApi> for GetUser {
#     type Response = User;
#     decoder!(decoder::Response::build().status_code_json(http::StatusCode::OK).finish());
#     fn request_builder(&self, client: &reqwest::Client, base_url: &BaseUrl) -> reqwest::RequestBuilder {
#         client.get(base_url.set_path(&format!("/users/{}", self.username)))
#     }
# }
use mhttp::testing::TestRequest;

// Assert request construction without making HTTP calls
let base_url = BaseUrl::new("https://api.github.com".parse().unwrap());

TestRequest {
    method: reqwest::Method::GET,
    url: "https://api.github.com/users/octocat".parse().unwrap(),
    headers: http::HeaderMap::new(),
    body: None,
}.assert(&GetUser { username: "octocat".to_string() }, &base_url);
```

## License

MIT
