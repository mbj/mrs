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

## Usage

Define a request type and implement the `Request` trait:

```rust
use mhttp::{BaseUrl, Request, decoder};

struct GetUser {
    id: u64,
}

#[derive(Debug, serde::Deserialize)]
struct User {
    id: u64,
    name: String,
}

struct MyApi;

impl Request<MyApi> for GetUser {
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
```

## Declarative Response Decoding

Decoders are built using a builder pattern that maps status codes to content-type handlers:

```rust
# use mhttp::decoder;
# #[derive(Debug, serde::Deserialize)]
# struct User { id: u64 }
# #[derive(Debug, serde::Deserialize)]
# struct Created<T>(T);
# impl<T> Created<T> { fn into_inner(self) -> T { self.0 } }
// Simple JSON response
let _: decoder::Response<User> = decoder::Response::build()
    .status_code_json(http::StatusCode::OK)
    .finish();

// Multiple status codes with different handling
let _: decoder::Response<User> = decoder::Response::build()
    .status_code_json(http::StatusCode::OK)
    .status_code_json_map(http::StatusCode::CREATED, Created::into_inner)
    .finish();

// Custom content-type handling
let _: decoder::Response<String> = decoder::Response::build()
    .status_code(http::StatusCode::OK, |content_types| {
        content_types.add("text/plain", |body| {
            Ok(String::from_utf8_lossy(body).into_owned())
        });
    })
    .finish();
```

## Pagination

Built-in support for RFC 8288 Link header pagination:

```rust,ignore
use mhttp::link::{paginate, Paginated, PaginatedRequest};

struct ListUsers;

impl Request<MyApi> for ListUsers {
    type Response = Paginated<Vec<User>>;

    decoder!(
        decoder::Response::build()
            .status_code_json(http::StatusCode::OK)
            .paginated()
    );

    // ...
}

impl PaginatedRequest for ListUsers {}

// Stream through all pages
let mut pages = paginate(service, ListUsers);
while let Some(users) = pages.next().await {
    // process users
}
```

## Testing

Enable the `test-utils` feature for request assertion helpers:

```rust
# use mhttp::{BaseUrl, Request, decoder};
# #[derive(Debug, serde::Deserialize)]
# struct User { id: u64 }
# struct MyApi;
# struct GetUser { id: u64 }
# impl Request<MyApi> for GetUser {
#     type Response = User;
#     decoder!(decoder::Response::build().status_code_json(http::StatusCode::OK).finish());
#     fn request_builder(&self, client: &reqwest::Client, base_url: &BaseUrl) -> reqwest::RequestBuilder {
#         client.get(base_url.set_path(&format!("/users/{}", self.id)))
#     }
# }
# let base_url = BaseUrl::new("https://api.example.com".parse().unwrap());
use mhttp::testing::TestRequest;

TestRequest {
    method: reqwest::Method::GET,
    url: "https://api.example.com/users/123".parse().unwrap(),
    headers: http::HeaderMap::new(),
    body: None,
}.assert(&GetUser { id: 123 }, &base_url);
```

## License

MIT
