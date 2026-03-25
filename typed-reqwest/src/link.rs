//! HTTP Link header types and parsing (RFC 8288).

use nom::{
    IResult, Parser,
    bytes::complete::{take_while, take_while1},
    character::complete::char,
    combinator::value,
    multi::separated_list0,
    sequence::{delimited, preceded},
};
use nom_language::error::VerboseError;

/// A type that can be parsed from a string using nom.
trait Parse: Sized {
    /// Parses the type from a string.
    fn parse(remaining: &str) -> IResult<&str, Self, VerboseError<&str>>;
}

/// Reason for a Link header parse error.
#[derive(Debug, PartialEq)]
pub enum ParseErrorReason {
    /// Duplicate rel value in Link header.
    DuplicateRel(Rel),
    /// Invalid Link header format.
    Format(String),
    /// Link header contains invalid UTF-8.
    InvalidUtf8,
}

/// Relation type for a link.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rel {
    /// Next page.
    Next,
    /// Previous page.
    Prev,
    /// First page.
    First,
    /// Last page.
    Last,
}

/// Error when parsing a Link header.
#[derive(Debug, PartialEq)]
pub struct ParseError(pub ParseErrorReason);

impl std::fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            ParseErrorReason::DuplicateRel(rel) => {
                write!(formatter, "duplicate rel={rel:?} in Link header")
            }
            ParseErrorReason::Format(msg) => {
                write!(formatter, "invalid Link header format: {msg}")
            }
            ParseErrorReason::InvalidUtf8 => {
                write!(formatter, "invalid UTF-8 in Link header")
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// Pagination links extracted from a Link header.
#[derive(Clone, Debug, PartialEq)]
pub struct Links {
    /// URL for the next page.
    pub next: Option<url::Url>,
    /// URL for the previous page.
    pub prev: Option<url::Url>,
    /// URL for the first page.
    pub first: Option<url::Url>,
    /// URL for the last page.
    pub last: Option<url::Url>,
}

/// A paginated response containing data and pagination links.
#[derive(Clone, Debug, PartialEq)]
pub struct Paginated<T> {
    /// The response data.
    pub data: T,
    /// Pagination links from the Link header, if present.
    pub links: Option<Links>,
}

/// A request for fetching a pagination URL.
///
/// Extracts the path and query from a Link header URL and applies them
/// to the base URL, ensuring requests stay within the same domain.
pub struct PaginationRequest<R> {
    path_segments: Vec<String>,
    query: Option<String>,
    _marker: std::marker::PhantomData<R>,
}

impl<API, R: crate::Request<API>> crate::Request<API> for PaginationRequest<R> {
    type Response = R::Response;

    const DECODER: std::sync::LazyLock<crate::decoder::Response<Self::Response>> = R::DECODER;

    fn request_builder(
        &self,
        client: &reqwest::Client,
        base_url: &crate::BaseUrl,
    ) -> reqwest::RequestBuilder {
        let segment_refs: Vec<&str> = self.path_segments.iter().map(String::as_str).collect();
        let mut url = base_url.set_path_segments(&segment_refs);
        url.set_query(self.query.as_deref());
        client.get(url)
    }
}

/// A request type that supports pagination.
///
/// Implement this trait for request types that return [`Paginated`] responses.
pub trait PaginatedRequest: Sized {
    /// Creates a request for fetching a pagination URL.
    ///
    /// Extracts the path and query from the URL to be applied to the base URL.
    fn for_url(url: url::Url) -> PaginationRequest<Self> {
        let mut path_segments = url
            .path_segments()
            .map(|segments| segments.map(str::to_string).collect::<Vec<_>>())
            .unwrap_or_default();
        if url.path().ends_with('/')
            && path_segments
                .last()
                .is_none_or(|segment| !segment.is_empty())
        {
            path_segments.push(String::new());
        }
        PaginationRequest {
            path_segments,
            query: url.query().map(str::to_owned),
            _marker: std::marker::PhantomData,
        }
    }
}

/// Error from pagination.
#[derive(Debug)]
pub enum PaginateError<E> {
    /// Service error.
    Service(E),
}

impl<E: std::fmt::Display> std::fmt::Display for PaginateError<E> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Service(error) => write!(formatter, "pagination service error: {error}"),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for PaginateError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Service(error) => Some(error),
        }
    }
}

/// Returns a stream that yields each page's data by following pagination links.
///
/// # Example
///
/// ```
/// use futures_util::StreamExt;
/// use typed_reqwest::link::{paginate, Paginated, PaginatedRequest, PaginationRequest};
/// # use typed_reqwest::{BaseUrl, Request, decoder};
/// # use std::pin::pin;
/// # struct Api;
/// # #[derive(Clone, Debug, serde::Deserialize)]
/// # struct Pull { title: String }
/// # struct ListPulls;
/// # impl Request<Api> for ListPulls {
/// #     type Response = Paginated<Vec<Pull>>;
/// #     decoder!(decoder::Response::build().status_code_constant(http::StatusCode::OK, Vec::new()).paginated());
/// #     fn request_builder(&self, client: &reqwest::Client, base_url: &BaseUrl) -> reqwest::RequestBuilder {
/// #         client.get(base_url.set_path_segments(&["pulls"]))
/// #     }
/// # }
/// # impl PaginatedRequest for ListPulls {}
/// # use std::convert::Infallible;
/// # use std::task::{Context, Poll};
/// # struct MockService;
/// # impl tower_service::Service<ListPulls> for MockService {
/// #     type Response = Paginated<Vec<Pull>>;
/// #     type Error = Infallible;
/// #     type Future = std::future::Ready<Result<Self::Response, Self::Error>>;
/// #     fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> { Poll::Ready(Ok(())) }
/// #     fn call(&mut self, _: ListPulls) -> Self::Future { std::future::ready(Ok(Paginated { data: Vec::new(), links: None })) }
/// # }
/// # impl tower_service::Service<PaginationRequest<ListPulls>> for MockService {
/// #     type Response = Paginated<Vec<Pull>>;
/// #     type Error = Infallible;
/// #     type Future = std::future::Ready<Result<Self::Response, Self::Error>>;
/// #     fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> { Poll::Ready(Ok(())) }
/// #     fn call(&mut self, _: PaginationRequest<ListPulls>) -> Self::Future { std::future::ready(Ok(Paginated { data: Vec::new(), links: None })) }
/// # }
/// # async fn example() {
/// # let service = MockService;
///
/// let mut pages = pin!(paginate(service, ListPulls));
///
/// while let Some(result) = pages.next().await {
///     let pulls: Vec<Pull> = result.unwrap();
///     for pull in pulls {
///         println!("{}", pull.title);
///     }
/// }
/// # }
/// ```
pub fn paginate<API, S, R, T, E>(
    service: S,
    request: R,
) -> impl futures_util::Stream<Item = Result<T, PaginateError<E>>>
where
    R: crate::Request<API, Response = Paginated<T>> + PaginatedRequest,
    S: tower_service::Service<R, Response = Paginated<T>, Error = E>
        + tower_service::Service<PaginationRequest<R>, Response = Paginated<T>, Error = E>,
{
    enum State<S, R> {
        Initial {
            service: S,
            request: R,
        },
        Next {
            service: S,
            request: PaginationRequest<R>,
        },
        Done,
    }

    futures_util::stream::unfold(State::Initial { service, request }, |state| async move {
        let (service, page) = match state {
            State::Done => return None,
            State::Initial {
                mut service,
                request,
            } => {
                let page = service.call(request).await;
                (service, page)
            }
            State::Next {
                mut service,
                request,
            } => {
                let page = service.call(request).await;
                (service, page)
            }
        };

        let page = match page {
            Ok(page) => page,
            Err(error) => return Some((Err(PaginateError::Service(error)), State::Done)),
        };

        let next_state = match page.links.as_ref().and_then(|links| links.next.clone()) {
            Some(url) => State::Next {
                service,
                request: R::for_url(url),
            },
            None => State::Done,
        };

        Some((Ok(page.data), next_state))
    })
}

impl Links {
    /// Parses pagination links from an HTTP header value.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the header contains invalid UTF-8 or has an invalid format.
    pub fn from_header_value(value: &http::header::HeaderValue) -> Result<Self, ParseError> {
        let header_string = value
            .to_str()
            .map_err(|_| ParseError(ParseErrorReason::InvalidUtf8))?;

        Self::from_header(header_string)
    }

    /// Parses pagination links from a Link header value.
    ///
    /// Format: `<url>; rel="next", <url>; rel="prev", ...`
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the header format is invalid or contains duplicate rel values.
    pub fn from_header(header: &str) -> Result<Self, ParseError> {
        let (_, entries) = nom::combinator::all_consuming(nom::sequence::delimited(
            whitespace,
            separated_list0(
                (whitespace, char(','), whitespace),
                <LinkEntry as Parse>::parse,
            ),
            whitespace,
        ))
        .parse(header)
        .map_err(|error| ParseError(ParseErrorReason::Format(format!("{error}"))))?;

        let mut links = Self {
            next: None,
            prev: None,
            first: None,
            last: None,
        };

        for entry in entries {
            if let Some(rel) = entry.rel {
                let slot = match rel {
                    Rel::Next => &mut links.next,
                    Rel::Prev => &mut links.prev,
                    Rel::First => &mut links.first,
                    Rel::Last => &mut links.last,
                };

                if slot.is_some() {
                    return Err(ParseError(ParseErrorReason::DuplicateRel(rel)));
                }

                *slot = Some(entry.url);
            }
        }

        Ok(links)
    }
}

impl Parse for Rel {
    fn parse(remaining: &str) -> IResult<&str, Self, VerboseError<&str>> {
        nom::branch::alt((
            nom::combinator::value(Self::Next, nom::bytes::complete::tag("next")),
            nom::combinator::value(Self::Prev, nom::bytes::complete::tag("prev")),
            nom::combinator::value(Self::First, nom::bytes::complete::tag("first")),
            nom::combinator::value(Self::Last, nom::bytes::complete::tag("last")),
        ))
        .parse(remaining)
    }
}

/// A parsed link entry.
struct LinkEntry {
    url: url::Url,
    rel: Option<Rel>,
}

impl Parse for LinkEntry {
    fn parse(remaining: &str) -> IResult<&str, Self, VerboseError<&str>> {
        let (remaining, (url_str, params)) = (parse_url, parse_params).parse(remaining)?;

        let url = url_str.parse::<url::Url>().map_err(|_| {
            nom::Err::Error(VerboseError {
                errors: vec![(
                    remaining,
                    nom_language::error::VerboseErrorKind::Context("invalid URL"),
                )],
            })
        })?;

        let rel = params.into_iter().find_map(|param| param);

        Ok((remaining, Self { url, rel }))
    }
}

fn parse_url(remaining: &str) -> IResult<&str, &str, VerboseError<&str>> {
    delimited(
        (whitespace, char('<')),
        take_while(|character| character != '>'),
        char('>'),
    )
    .parse(remaining)
}

fn parse_params(remaining: &str) -> IResult<&str, Vec<Option<Rel>>, VerboseError<&str>> {
    nom::multi::many0(preceded((whitespace, char(';'), whitespace), parse_param)).parse(remaining)
}

fn parse_param(remaining: &str) -> IResult<&str, Option<Rel>, VerboseError<&str>> {
    let (remaining, (key, _, param_value)) = (
        take_while1(|character: char| character.is_alphanumeric() || character == '_'),
        char('='),
        parse_param_value,
    )
        .parse(remaining)?;

    if key == "rel" {
        Ok((
            remaining,
            <Rel as Parse>::parse(param_value).ok().map(|(_, rel)| rel),
        ))
    } else {
        Ok((remaining, None))
    }
}

fn parse_param_value(remaining: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let quoted = delimited(
        char('"'),
        take_while(|character| character != '"'),
        char('"'),
    );

    let unquoted = take_while1(|character: char| {
        character.is_alphanumeric() || character == '_' || character == '-'
    });

    nom::branch::alt((quoted, unquoted)).parse(remaining)
}

fn whitespace(remaining: &str) -> IResult<&str, (), VerboseError<&str>> {
    value((), take_while(|character: char| character.is_whitespace())).parse(remaining)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let links = Links::from_header("").unwrap();

        assert_eq!(
            links,
            Links {
                next: None,
                prev: None,
                first: None,
                last: None,
            }
        );
    }

    #[test]
    fn parse_next_only() {
        let links =
            Links::from_header(r#"<https://api.example.com/items?page=2>; rel="next""#).unwrap();

        assert_eq!(
            links,
            Links {
                next: Some("https://api.example.com/items?page=2".parse().unwrap()),
                prev: None,
                first: None,
                last: None,
            }
        );
    }

    #[test]
    fn parse_unquoted_rel() {
        let links =
            Links::from_header(r#"<https://api.example.com/items?page=2>; rel=next"#).unwrap();

        assert_eq!(
            links,
            Links {
                next: Some("https://api.example.com/items?page=2".parse().unwrap()),
                prev: None,
                first: None,
                last: None,
            }
        );
    }

    #[test]
    fn parse_all_links() {
        let header = r#"<https://api.example.com/items?page=3>; rel="next", <https://api.example.com/items?page=1>; rel="prev", <https://api.example.com/items?page=1>; rel="first", <https://api.example.com/items?page=5>; rel="last""#;
        let links = Links::from_header(header).unwrap();

        assert_eq!(
            links,
            Links {
                next: Some("https://api.example.com/items?page=3".parse().unwrap()),
                prev: Some("https://api.example.com/items?page=1".parse().unwrap()),
                first: Some("https://api.example.com/items?page=1".parse().unwrap()),
                last: Some("https://api.example.com/items?page=5".parse().unwrap()),
            }
        );
    }

    #[test]
    fn parse_ignores_unknown_rel() {
        let links =
            Links::from_header(r#"<https://api.example.com/items?page=2>; rel="unknown""#).unwrap();

        assert_eq!(
            links,
            Links {
                next: None,
                prev: None,
                first: None,
                last: None,
            }
        );
    }

    #[test]
    fn parse_with_extra_params() {
        let links = Links::from_header(
            r#"<https://api.example.com/items?page=2>; rel="next"; title="Next Page""#,
        )
        .unwrap();

        assert_eq!(
            links,
            Links {
                next: Some("https://api.example.com/items?page=2".parse().unwrap()),
                prev: None,
                first: None,
                last: None,
            }
        );
    }

    #[test]
    fn parse_handles_whitespace() {
        let links =
            Links::from_header(r#"  <https://api.example.com/items?page=2> ;  rel="next"  "#)
                .unwrap();

        assert_eq!(
            links,
            Links {
                next: Some("https://api.example.com/items?page=2".parse().unwrap()),
                prev: None,
                first: None,
                last: None,
            }
        );
    }

    #[test]
    fn parse_error_on_invalid_url() {
        let error = Links::from_header(r#"<not a url>; rel="next""#).unwrap_err();

        assert!(matches!(error.0, ParseErrorReason::Format(_)));
    }

    #[test]
    fn parse_error_on_malformed() {
        let error = Links::from_header("this is not a link header").unwrap_err();

        assert!(matches!(error.0, ParseErrorReason::Format(_)));
    }

    #[test]
    fn parse_error_on_duplicate_next() {
        let error = Links::from_header(
            r#"<https://api.example.com/items?page=2>; rel="next", <https://api.example.com/items?page=3>; rel="next""#,
        )
        .unwrap_err();

        assert_eq!(error.0, ParseErrorReason::DuplicateRel(Rel::Next));
    }

    #[test]
    fn parse_error_on_duplicate_prev() {
        let error = Links::from_header(
            r#"<https://api.example.com/items?page=1>; rel="prev", <https://api.example.com/items?page=0>; rel="prev""#,
        )
        .unwrap_err();

        assert_eq!(error.0, ParseErrorReason::DuplicateRel(Rel::Prev));
    }

    use std::collections::VecDeque;
    use std::convert::Infallible;
    use std::future::{Ready, ready};
    use std::marker::PhantomData;
    use std::task::{Context, Poll};

    /// A mock service for testing pagination.
    ///
    /// Returns pre-configured pages in sequence and records requests for verification.
    pub struct MockPaginationService<API, T> {
        base_url: crate::BaseUrl,
        pages: VecDeque<Paginated<T>>,
        requests: Vec<crate::testing::TestRequest>,
        _api: PhantomData<API>,
    }

    impl<API, T> MockPaginationService<API, T> {
        /// Creates a new mock service with the given pages.
        pub fn new(base_url: crate::BaseUrl, pages: Vec<Paginated<T>>) -> Self {
            Self {
                base_url,
                pages: pages.into(),
                requests: Vec::new(),
                _api: PhantomData,
            }
        }

        /// Returns the requests that were made to this service.
        pub fn requests(&self) -> &[crate::testing::TestRequest] {
            &self.requests
        }
    }

    impl<API, R, T> tower_service::Service<R> for MockPaginationService<API, T>
    where
        R: crate::Request<API, Response = Paginated<T>>,
        T: Clone,
    {
        type Response = Paginated<T>;
        type Error = Infallible;
        type Future = Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, request: R) -> Self::Future {
            self.requests
                .push(crate::testing::TestRequest::from_request(
                    &request,
                    &self.base_url,
                ));
            let page = self.pages.pop_front().expect("no more pages configured");
            ready(Ok(page))
        }
    }

    struct TestApi;

    struct ListItems;

    fn api_host() -> url::Host<String> {
        url::Host::parse("api.example.com").unwrap()
    }

    impl crate::Request<TestApi> for ListItems {
        type Response = Paginated<String>;

        crate::decoder!(
            crate::decoder::Response::build()
                .status_code_constant(http::StatusCode::OK, String::new())
                .paginated()
        );

        fn request_builder(
            &self,
            client: &reqwest::Client,
            base_url: &crate::BaseUrl,
        ) -> reqwest::RequestBuilder {
            client.get(base_url.set_path_segments(&["items"]))
        }
    }

    impl PaginatedRequest for ListItems {}

    #[tokio::test]
    async fn paginate_single_page() {
        use futures_util::StreamExt;

        let base_url = crate::BaseUrl::https(api_host());
        let pages = vec![Paginated {
            data: "page1".to_string(),
            links: None,
        }];

        let service: MockPaginationService<TestApi, String> =
            MockPaginationService::new(base_url.clone(), pages);

        let items: Vec<_> = paginate(service, ListItems)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(items, vec!["page1"]);
    }

    #[tokio::test]
    async fn paginate_multiple_pages() {
        use futures_util::StreamExt;

        let base_url = crate::BaseUrl::https(api_host());
        let pages = vec![
            Paginated {
                data: "page1".to_string(),
                links: Some(Links {
                    next: Some("https://api.example.com/items?page=2".parse().unwrap()),
                    prev: None,
                    first: None,
                    last: None,
                }),
            },
            Paginated {
                data: "page2".to_string(),
                links: Some(Links {
                    next: Some("https://api.example.com/items?page=3".parse().unwrap()),
                    prev: None,
                    first: None,
                    last: None,
                }),
            },
            Paginated {
                data: "page3".to_string(),
                links: None,
            },
        ];

        let service: MockPaginationService<TestApi, String> =
            MockPaginationService::new(base_url.clone(), pages);

        let items: Vec<_> = paginate(service, ListItems)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(items, vec!["page1", "page2", "page3"]);
    }

    #[tokio::test]
    async fn paginate_verifies_requests() {
        use futures_util::StreamExt;

        let base_url = crate::BaseUrl::https(api_host());
        let pages = vec![
            Paginated {
                data: "page1".to_string(),
                links: Some(Links {
                    next: Some("https://api.example.com/items?page=2".parse().unwrap()),
                    prev: None,
                    first: None,
                    last: None,
                }),
            },
            Paginated {
                data: "page2".to_string(),
                links: None,
            },
        ];

        let mut service: MockPaginationService<TestApi, String> =
            MockPaginationService::new(base_url.clone(), pages);

        let _: Vec<_> = paginate(&mut service, ListItems).collect::<Vec<_>>().await;

        assert_eq!(service.requests().len(), 2);
        assert_eq!(
            service.requests()[0].url.as_str(),
            "https://api.example.com/items"
        );
        assert_eq!(
            service.requests()[1].url.as_str(),
            "https://api.example.com/items?page=2"
        );
    }

    #[test]
    fn paginate_preserves_trailing_slash() {
        let base_url = crate::BaseUrl::https(api_host());
        let url = "https://api.example.com/items/?page=2".parse().unwrap();
        let request = <ListItems as PaginatedRequest>::for_url(url);
        let built = crate::testing::TestRequest::from_request(&request, &base_url);

        assert_eq!(built.url.as_str(), "https://api.example.com/items/?page=2");
    }
}
