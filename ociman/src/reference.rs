//! OCI image reference parsing and types.
//!
//! This module provides types for working with OCI image references following
//! the [distribution/reference](https://github.com/distribution/reference) specification.
//!
//! # Grammar
//!
//! ```text
//! reference                       := name [ ":" tag ] [ "@" digest ]
//! name                            := [domain '/'] remote-name
//! domain                          := host [':' port-number]
//! host                            := domain-name | IPv4address | [ IPv6address ]
//! domain-name                     := domain-component ['.' domain-component]*
//! domain-component                := /([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9])/
//! port-number                     := /[0-9]+/
//! path-component                  := alpha-numeric [separator alpha-numeric]*
//! path (remote-name)              := path-component ['/' path-component]*
//! alpha-numeric                   := /[a-z0-9]+/
//! separator                       := /[_.]|__|[-]*/
//! tag                             := /[\w][\w.-]{0,127}/
//! digest                          := digest-algorithm ":" digest-hex
//! digest-algorithm                := digest-algorithm-component [ digest-algorithm-separator digest-algorithm-component ]*
//! digest-algorithm-separator      := /[+.-_]/
//! digest-algorithm-component      := /[A-Za-z][A-Za-z0-9]*/
//! digest-hex                      := /[0-9a-fA-F]{32,}/
//! ```
//!
//! # Examples
//!
//! ```
//! use ociman::reference::Reference;
//!
//! // Simple image name
//! let reference: Reference = "alpine".parse().unwrap();
//! assert_eq!(reference.to_string(), "alpine");
//!
//! // With tag
//! let reference: Reference = "alpine:latest".parse().unwrap();
//! assert_eq!(reference.tag.as_ref().map(|t| t.as_str()), Some("latest"));
//!
//! // With domain and path
//! let reference: Reference = "docker.io/library/alpine:latest".parse().unwrap();
//! assert!(reference.name.domain.is_some());
//! assert_eq!(reference.name.path.to_string(), "library/alpine");
//! ```

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{take_while, take_while_m_n, take_while1},
    character::complete::{char, digit1, u16},
    combinator::{all_consuming, opt, recognize, verify},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded},
};

pub(crate) trait Parse: Sized {
    fn parse(input: &str) -> IResult<&str, Self>;
}

fn write_interspersed<I, T>(
    formatter: &mut std::fmt::Formatter<'_>,
    items: I,
    separator: &str,
) -> std::fmt::Result
where
    I: IntoIterator<Item = T>,
    T: AsRef<str>,
{
    let mut iter = items.into_iter();
    if let Some(first) = iter.next() {
        formatter.write_str(first.as_ref())?;
        for item in iter {
            formatter.write_str(separator)?;
            formatter.write_str(item.as_ref())?;
        }
    }
    Ok(())
}

macro_rules! impl_from_str {
    ($($type:ty),* $(,)?) => {
        $(
            impl std::str::FromStr for $type {
                type Err = String;

                fn from_str(string: &str) -> Result<Self, Self::Err> {
                    match all_consuming(<Self as Parse>::parse).parse(string) {
                        Ok((_remaining, value)) => Ok(value),
                        Err(error) => Err(format!("parse error: {error}")),
                    }
                }
            }
        )*
    };
}

impl_from_str!(
    DomainComponent,
    DomainName,
    PortNumber,
    Host,
    Domain,
    PathComponent,
    Path,
    Name,
    Tag,
    DigestAlgorithm,
    Digest,
    Reference,
);

/// A domain component: alphanumeric, optionally with hyphens between alphanumerics.
///
/// Pattern: `([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]*[a-zA-Z0-9])`
///
/// # Examples
///
/// ```
/// use ociman::reference::DomainComponent;
///
/// let component: DomainComponent = "example".parse().unwrap();
/// assert_eq!(component.as_str(), "example");
///
/// let component: DomainComponent = "my-registry".parse().unwrap();
/// assert_eq!(component.as_str(), "my-registry");
///
/// // Multiple consecutive hyphens allowed between alphanumerics
/// let component: DomainComponent = "my--registry".parse().unwrap();
/// assert_eq!(component.as_str(), "my--registry");
///
/// // Rejects empty input
/// assert_eq!(
///     "".parse::<DomainComponent>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"\", code: TakeWhile1 }"
/// );
///
/// // Rejects leading hyphen
/// assert_eq!(
///     "-example".parse::<DomainComponent>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"-example\", code: TakeWhile1 }"
/// );
///
/// // Rejects trailing hyphen
/// assert_eq!(
///     "example-".parse::<DomainComponent>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"-\", code: Eof }"
/// );
///
/// // Rejects special characters
/// assert_eq!(
///     "_invalid".parse::<DomainComponent>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"_invalid\", code: TakeWhile1 }"
/// );
/// assert_eq!(
///     "@invalid".parse::<DomainComponent>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"@invalid\", code: TakeWhile1 }"
/// );
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DomainComponent(String);

impl DomainComponent {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for DomainComponent {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Parse for DomainComponent {
    fn parse(input: &str) -> IResult<&str, Self> {
        recognize(pair(
            take_while1(|character: char| character.is_ascii_alphanumeric()),
            many0(pair(
                take_while1(|character: char| character == '-'),
                take_while1(|character: char| character.is_ascii_alphanumeric()),
            )),
        ))
        .map(|string: &str| Self(string.to_string()))
        .parse(input)
    }
}

/// A domain name: one or more domain components separated by dots.
///
/// Pattern: `domain-component ['.' domain-component]*`
///
/// # Examples
///
/// ```
/// use ociman::reference::DomainName;
///
/// let name: DomainName = "docker.io".parse().unwrap();
/// assert_eq!(name.to_string(), "docker.io");
/// assert_eq!(name.components().len(), 2);
///
/// let name: DomainName = "ghcr.io".parse().unwrap();
/// assert_eq!(name.to_string(), "ghcr.io");
///
/// // Rejects empty input
/// assert_eq!(
///     "".parse::<DomainName>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"\", code: TakeWhile1 }"
/// );
///
/// // Rejects leading dot
/// assert_eq!(
///     ".example.com".parse::<DomainName>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \".example.com\", code: TakeWhile1 }"
/// );
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DomainName(Vec<DomainComponent>);

impl DomainName {
    #[must_use]
    pub fn components(&self) -> &[DomainComponent] {
        &self.0
    }
}

impl Parse for DomainName {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(char('.'), DomainComponent::parse)
            .map(Self)
            .parse(input)
    }
}

impl std::fmt::Display for DomainName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_interspersed(formatter, &self.0, ".")
    }
}

/// A port number.
///
/// # Examples
///
/// ```
/// use ociman::reference::PortNumber;
///
/// let port: PortNumber = "5000".parse().unwrap();
/// assert_eq!(port.value(), 5000);
///
/// let port: PortNumber = "8080".parse().unwrap();
/// assert_eq!(port.value(), 8080);
///
/// // Rejects empty input
/// assert_eq!(
///     "".parse::<PortNumber>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"\", code: Digit }"
/// );
///
/// // Rejects non-numeric
/// assert_eq!(
///     "abc".parse::<PortNumber>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"abc\", code: Digit }"
/// );
///
/// // Rejects values > 65535 (u16::MAX)
/// assert_eq!(
///     "123456".parse::<PortNumber>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"123456\", code: Digit }"
/// );
/// ```
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PortNumber(u16);

impl PortNumber {
    #[must_use]
    pub fn value(&self) -> u16 {
        self.0
    }
}

impl Parse for PortNumber {
    fn parse(input: &str) -> IResult<&str, Self> {
        u16.map(Self).parse(input)
    }
}

impl std::fmt::Display for PortNumber {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A host: either a domain name, IPv4 address, or bracketed IPv6 address.
///
/// # Examples
///
/// ```
/// use ociman::reference::Host;
///
/// let host: Host = "docker.io".parse().unwrap();
/// assert!(matches!(host, Host::DomainName(_)));
/// assert_eq!(host.to_string(), "docker.io");
///
/// let host: Host = "192.168.1.1".parse().unwrap();
/// assert!(matches!(host, Host::Ipv4(_)));
///
/// let host: Host = "[::1]".parse().unwrap();
/// assert!(matches!(host, Host::Ipv6(_)));
///
/// // Rejects empty input
/// assert_eq!(
///     "".parse::<Host>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"\", code: TakeWhile1 }"
/// );
///
/// // Rejects unclosed IPv6 bracket
/// assert_eq!(
///     "[::1".parse::<Host>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"[::1\", code: TakeWhile1 }"
/// );
///
/// // Invalid IPv4 parses as domain name
/// let host: Host = "999.999.999.999".parse().unwrap();
/// assert!(matches!(host, Host::DomainName(_)));
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Host {
    DomainName(DomainName),
    Ipv4(std::net::Ipv4Addr),
    Ipv6(std::net::Ipv6Addr),
}

impl Host {
    fn parse_ipv4(input: &str) -> IResult<&str, std::net::Ipv4Addr> {
        recognize((
            digit1,
            char('.'),
            digit1,
            char('.'),
            digit1,
            char('.'),
            digit1,
        ))
        .map_res(|s: &str| s.parse())
        .parse(input)
    }

    fn parse_ipv6(input: &str) -> IResult<&str, std::net::Ipv6Addr> {
        delimited(
            char('['),
            take_while1(|character: char| character.is_ascii_hexdigit() || character == ':'),
            char(']'),
        )
        .map_res(|s: &str| s.parse())
        .parse(input)
    }
}

impl Parse for Host {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            Self::parse_ipv6.map(Self::Ipv6),
            Self::parse_ipv4.map(Self::Ipv4),
            DomainName::parse.map(Self::DomainName),
        ))
        .parse(input)
    }
}

impl std::fmt::Display for Host {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DomainName(name) => write!(formatter, "{name}"),
            Self::Ipv4(address) => write!(formatter, "{address}"),
            Self::Ipv6(address) => write!(formatter, "[{address}]"),
        }
    }
}

/// A domain: host with optional port number.
///
/// Pattern: `host [':' port-number]`
///
/// # Examples
///
/// ```
/// use ociman::reference::Domain;
///
/// let domain: Domain = "docker.io".parse().unwrap();
/// assert_eq!(domain.to_string(), "docker.io");
/// assert!(domain.port.is_none());
///
/// let domain: Domain = "localhost:5000".parse().unwrap();
/// assert_eq!(domain.to_string(), "localhost:5000");
/// assert_eq!(domain.port.map(|p| p.value()), Some(5000));
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Domain {
    pub host: Host,
    pub port: Option<PortNumber>,
}

impl Domain {
    /// Returns true if this domain looks like a registry endpoint.
    ///
    /// Following the distribution/reference convention (used by both Docker and
    /// Podman) on top of the OCI spec: a domain is a registry if it has a port,
    /// is an IP address, is the literal "localhost", or is a multi-component
    /// domain name (contains dots). A bare single-component name like
    /// "myproject" is treated as a path component, not a domain.
    #[must_use]
    pub fn is_registry(&self) -> bool {
        if self.port.is_some() {
            return true;
        }

        match &self.host {
            Host::Ipv4(_) | Host::Ipv6(_) => true,
            Host::DomainName(name) => {
                name.components().len() > 1 || name.to_string() == "localhost"
            }
        }
    }
}

impl Parse for Domain {
    fn parse(input: &str) -> IResult<&str, Self> {
        pair(Host::parse, opt(preceded(char(':'), PortNumber::parse)))
            .map(|(host, port)| Self { host, port })
            .parse(input)
    }
}

impl std::fmt::Display for Domain {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.host)?;
        if let Some(port) = &self.port {
            write!(formatter, ":{port}")?;
        }
        Ok(())
    }
}

/// A path component: alphanumeric segments with separators.
///
/// Pattern: `alpha-numeric [separator alpha-numeric]*`
///
/// # Examples
///
/// ```
/// use ociman::reference::PathComponent;
///
/// let component: PathComponent = "alpine".parse().unwrap();
/// assert_eq!(component.as_str(), "alpine");
///
/// let component: PathComponent = "my_image-name.test".parse().unwrap();
/// assert_eq!(component.as_str(), "my_image-name.test");
///
/// // Single hyphen separator
/// let component: PathComponent = "foo-bar".parse().unwrap();
/// assert_eq!(component.as_str(), "foo-bar");
///
/// // Multiple hyphens as separator
/// let component: PathComponent = "foo--bar".parse().unwrap();
/// assert_eq!(component.as_str(), "foo--bar");
///
/// let component: PathComponent = "foo---bar".parse().unwrap();
/// assert_eq!(component.as_str(), "foo---bar");
///
/// // Double underscore separator
/// let component: PathComponent = "foo__bar".parse().unwrap();
/// assert_eq!(component.as_str(), "foo__bar");
///
/// // Dot separator
/// let component: PathComponent = "foo.bar".parse().unwrap();
/// assert_eq!(component.as_str(), "foo.bar");
///
/// // Mixed separators
/// let component: PathComponent = "a-b_c.d__e---f".parse().unwrap();
/// assert_eq!(component.as_str(), "a-b_c.d__e---f");
///
/// // Rejects empty input
/// assert_eq!(
///     "".parse::<PathComponent>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"\", code: TakeWhile1 }"
/// );
///
/// // Rejects uppercase
/// assert_eq!(
///     "ALPINE".parse::<PathComponent>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"ALPINE\", code: TakeWhile1 }"
/// );
///
/// // Rejects leading separator
/// assert_eq!(
///     "_alpine".parse::<PathComponent>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"_alpine\", code: TakeWhile1 }"
/// );
/// assert_eq!(
///     ".alpine".parse::<PathComponent>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \".alpine\", code: TakeWhile1 }"
/// );
/// assert_eq!(
///     "-alpine".parse::<PathComponent>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"-alpine\", code: TakeWhile1 }"
/// );
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PathComponent(std::borrow::Cow<'static, str>);

/// Const-friendly consumer for [`PathComponent`].
///
/// Implements the full grammar
/// `path-component := alpha-numeric (separator alpha-numeric)*`
/// where `alpha-numeric := [a-z0-9]+` and
/// `separator := [_.] | __ | [-]*`.
///
/// Returns the number of bytes consumed from the start of `input`. The
/// callers split:
/// - [`PathComponent::from_static_or_panic`] requires the full input to be
///   consumed (rejects trailing data).
/// - [`PathComponent`]'s `Parse` impl uses the consumed length as the nom
///   match boundary, leaving the remainder for the next combinator.
///
/// This is the single source of truth for the grammar — both compile-time
/// static literals and runtime nom parsing go through it.
const fn consume_path_component(input: &str) -> Result<usize, &'static str> {
    let bytes = input.as_bytes();
    let len = bytes.len();

    // Initial alpha-numeric block (mandatory, at least one byte).
    let mut pos = 0;
    while pos < len {
        let byte = bytes[pos];
        if !(byte.is_ascii_lowercase() || byte.is_ascii_digit()) {
            break;
        }
        pos += 1;
    }
    if pos == 0 {
        return Err("path component must start with [a-z0-9]");
    }

    // Loop: try to consume (separator + alpha-numeric). If the trailing
    // alpha-numeric is missing, rewind the separator and stop.
    loop {
        let saved = pos;

        // Separator: __ | _ | . | [-]*
        if pos + 1 < len && bytes[pos] == b'_' && bytes[pos + 1] == b'_' {
            pos += 2;
        } else if pos < len && (bytes[pos] == b'_' || bytes[pos] == b'.') {
            pos += 1;
        } else {
            while pos < len && bytes[pos] == b'-' {
                pos += 1;
            }
        }

        // Trailing alpha-numeric (mandatory after a non-empty separator
        // attempt; required for the iteration to succeed).
        let inner_start = pos;
        while pos < len {
            let byte = bytes[pos];
            if !(byte.is_ascii_lowercase() || byte.is_ascii_digit()) {
                break;
            }
            pos += 1;
        }

        if pos == inner_start {
            return Ok(saved);
        }
    }
}

impl PathComponent {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Create a [`PathComponent`] from a static string at compile time.
    ///
    /// # Panics
    ///
    /// Panics if `input` is not a valid path component, or if it contains
    /// trailing bytes that cannot be consumed by the grammar.
    #[must_use]
    pub const fn from_static_or_panic(input: &'static str) -> Self {
        match consume_path_component(input) {
            Ok(consumed) if consumed == input.len() => Self(std::borrow::Cow::Borrowed(input)),
            Ok(_) => panic!("path component has trailing input"),
            Err(message) => panic!("{}", message),
        }
    }
}

impl AsRef<str> for PathComponent {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Parse for PathComponent {
    fn parse(input: &str) -> IResult<&str, Self> {
        match consume_path_component(input) {
            Ok(consumed) => {
                let (matched, rest) = input.split_at(consumed);
                Ok((rest, Self(std::borrow::Cow::Owned(matched.to_string()))))
            }
            Err(_) => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::TakeWhile1,
            ))),
        }
    }
}

impl std::fmt::Display for PathComponent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A path (remote-name): one or more path components separated by slashes.
///
/// Pattern: `path-component ['/' path-component]*`
///
/// # Examples
///
/// ```
/// use ociman::reference::Path;
///
/// let path: Path = "library/alpine".parse().unwrap();
/// assert_eq!(path.to_string(), "library/alpine");
/// assert_eq!(path.components().len(), 2);
///
/// let path: Path = "owner/repo/subpath".parse().unwrap();
/// assert_eq!(path.to_string(), "owner/repo/subpath");
/// assert_eq!(path.components().len(), 3);
///
/// // Rejects empty input
/// assert_eq!(
///     "".parse::<Path>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"\", code: TakeWhile1 }"
/// );
///
/// // Rejects leading slash
/// assert_eq!(
///     "/alpine".parse::<Path>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"/alpine\", code: TakeWhile1 }"
/// );
///
/// // Double slash stops parsing (leaves remaining input)
/// assert_eq!(
///     "library//alpine".parse::<Path>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"//alpine\", code: Eof }"
/// );
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Path(Vec<PathComponent>);

impl Path {
    #[must_use]
    pub fn components(&self) -> &[PathComponent] {
        &self.0
    }
}

impl Parse for Path {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(char('/'), PathComponent::parse)
            .map(Self)
            .parse(input)
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write_interspersed(formatter, &self.0, "/")
    }
}

/// A name: optional domain followed by path.
///
/// Pattern: `[domain '/'] remote-name`
///
/// # Examples
///
/// ```
/// use ociman::reference::Name;
///
/// let name: Name = "alpine".parse().unwrap();
/// assert!(name.domain.is_none());
/// assert_eq!(name.path.to_string(), "alpine");
///
/// let name: Name = "docker.io/library/alpine".parse().unwrap();
/// assert!(name.domain.is_some());
/// assert_eq!(name.domain.unwrap().to_string(), "docker.io");
/// assert_eq!(name.path.to_string(), "library/alpine");
///
/// // Single-component names before '/' are path components, not domains.
/// // This follows the distribution/reference convention (used by both Docker
/// // and Podman) on top of the OCI spec: the first segment is only treated
/// // as a domain if it contains a dot, a colon (port), is an IP address,
/// // or is the literal "localhost".
/// let name: Name = "pg-ephemeral/main".parse().unwrap();
/// assert!(name.domain.is_none());
/// assert_eq!(name.path.to_string(), "pg-ephemeral/main");
///
/// let name: Name = "localhost/pg-ephemeral/main".parse().unwrap();
/// assert!(name.domain.is_some());
/// assert_eq!(name.domain.unwrap().to_string(), "localhost");
/// assert_eq!(name.path.to_string(), "pg-ephemeral/main");
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
#[serde(try_from = "String")]
pub struct Name {
    pub domain: Option<Domain>,
    pub path: Path,
}

impl TryFrom<String> for Name {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl Parse for Name {
    /// Parse a name, applying the distribution/reference convention for domain
    /// detection on top of the OCI spec: the segment before the first '/' is
    /// only a domain if it contains a dot, a colon (port), is an IP address,
    /// or is the literal "localhost".
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            pair(
                |input| {
                    let (remaining, domain) = Domain::parse(input)?;
                    if !remaining.starts_with('/') || !domain.is_registry() {
                        return Err(nom::Err::Error(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Verify,
                        )));
                    }
                    Ok((remaining, domain))
                },
                preceded(char('/'), Path::parse),
            )
            .map(|(domain, path)| Self {
                domain: Some(domain),
                path,
            }),
            Path::parse.map(|path| Self { domain: None, path }),
        ))
        .parse(input)
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(domain) = &self.domain {
            write!(formatter, "{domain}/")?;
        }
        write!(formatter, "{}", self.path)
    }
}

/// A tag: version identifier.
///
/// Pattern: `[\w][\w.-]{0,127}`
///
/// # Examples
///
/// ```
/// use ociman::reference::Tag;
///
/// let tag: Tag = "latest".parse().unwrap();
/// assert_eq!(tag.as_str(), "latest");
///
/// let tag: Tag = "v1.2.3-alpine".parse().unwrap();
/// assert_eq!(tag.as_str(), "v1.2.3-alpine");
///
/// // Accepts leading underscore
/// let tag: Tag = "_latest".parse().unwrap();
/// assert_eq!(tag.as_str(), "_latest");
///
/// // Rejects empty input
/// assert_eq!(
///     "".parse::<Tag>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"\", code: TakeWhileMN }"
/// );
///
/// // Rejects leading dot
/// assert_eq!(
///     ".latest".parse::<Tag>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \".latest\", code: TakeWhileMN }"
/// );
///
/// // Rejects leading hyphen
/// assert_eq!(
///     "-latest".parse::<Tag>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"-latest\", code: TakeWhileMN }"
/// );
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Tag(String);

impl Tag {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Parse for Tag {
    fn parse(input: &str) -> IResult<&str, Self> {
        recognize(pair(
            take_while_m_n(1, 1, |character: char| {
                character.is_ascii_alphanumeric() || character == '_'
            }),
            take_while_m_n(0, 127, |character: char| {
                character.is_ascii_alphanumeric()
                    || character == '_'
                    || character == '.'
                    || character == '-'
            }),
        ))
        .map(|string: &str| Self(string.to_string()))
        .parse(input)
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl From<sha2::digest::Output<sha2::Sha256>> for Tag {
    fn from(hash: sha2::digest::Output<sha2::Sha256>) -> Self {
        Self(hex::encode(hash))
    }
}

/// A digest algorithm.
///
/// Pattern: `digest-algorithm-component [ digest-algorithm-separator digest-algorithm-component ]*`
///
/// # Examples
///
/// ```
/// use ociman::reference::DigestAlgorithm;
///
/// let algorithm: DigestAlgorithm = "sha256".parse().unwrap();
/// assert_eq!(algorithm.as_str(), "sha256");
///
/// let algorithm: DigestAlgorithm = "sha512".parse().unwrap();
/// assert_eq!(algorithm.as_str(), "sha512");
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DigestAlgorithm(String);

impl DigestAlgorithm {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Pattern: `[A-Za-z][A-Za-z0-9]*`
    fn parse_component(input: &str) -> IResult<&str, &str> {
        recognize(pair(
            take_while_m_n(1, 1, |character: char| character.is_ascii_alphabetic()),
            take_while(|character: char| character.is_ascii_alphanumeric()),
        ))
        .parse(input)
    }

    /// Pattern: `[+.-_]`
    fn parse_separator(input: &str) -> IResult<&str, char> {
        alt((char('+'), char('.'), char('-'), char('_'))).parse(input)
    }
}

impl Parse for DigestAlgorithm {
    fn parse(input: &str) -> IResult<&str, Self> {
        recognize(pair(
            Self::parse_component,
            many0(pair(Self::parse_separator, Self::parse_component)),
        ))
        .map(|string: &str| Self(string.to_string()))
        .parse(input)
    }
}

impl std::fmt::Display for DigestAlgorithm {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A digest: algorithm and hex value.
///
/// Pattern: `digest-algorithm ":" digest-hex`
///
/// # Examples
///
/// ```
/// use ociman::reference::Digest;
///
/// let digest: Digest = "sha256:0123456789abcdef0123456789abcdef".parse().unwrap();
/// assert_eq!(digest.algorithm.as_str(), "sha256");
/// assert_eq!(digest.hex, "0123456789abcdef0123456789abcdef");
/// assert_eq!(digest.to_string(), "sha256:0123456789abcdef0123456789abcdef");
///
/// // Rejects empty input
/// assert_eq!(
///     "".parse::<Digest>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"\", code: TakeWhileMN }"
/// );
///
/// // Rejects missing colon
/// assert_eq!(
///     "sha2560123456789abcdef0123456789abcdef".parse::<Digest>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"\", code: Char }"
/// );
///
/// // Rejects short hex
/// assert_eq!(
///     "sha256:0123456789abcdef".parse::<Digest>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"0123456789abcdef\", code: Verify }"
/// );
///
/// // Stops at non-hex characters (leaves remaining input, causing Eof error)
/// assert_eq!(
///     "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdefgh".parse::<Digest>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"gh\", code: Eof }"
/// );
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Digest {
    pub algorithm: DigestAlgorithm,
    pub hex: String,
}

impl Digest {
    /// Pattern: `[0-9a-fA-F]{32,}`
    fn parse_hex(input: &str) -> IResult<&str, &str> {
        verify(
            take_while1(|character: char| character.is_ascii_hexdigit()),
            |string: &str| string.len() >= 32,
        )
        .parse(input)
    }
}

impl Parse for Digest {
    fn parse(input: &str) -> IResult<&str, Self> {
        (DigestAlgorithm::parse, char(':'), Self::parse_hex)
            .map(|(algorithm, _, hex)| Self {
                algorithm,
                hex: hex.to_string(),
            })
            .parse(input)
    }
}

impl std::fmt::Display for Digest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}:{}", self.algorithm, self.hex)
    }
}

/// A complete OCI image reference.
///
/// Pattern: `name [ ":" tag ] [ "@" digest ]`
///
/// # Examples
///
/// ```
/// use ociman::reference::Reference;
///
/// // Simple name
/// let reference: Reference = "alpine".parse().unwrap();
/// assert_eq!(reference.name.path.to_string(), "alpine");
/// assert!(reference.tag.is_none());
/// assert!(reference.digest.is_none());
///
/// // Name with tag
/// let reference: Reference = "alpine:latest".parse().unwrap();
/// assert_eq!(reference.tag.as_ref().map(|tag| tag.as_str()), Some("latest"));
///
/// // Name with digest
/// let reference: Reference = "alpine@sha256:0123456789abcdef0123456789abcdef".parse().unwrap();
/// assert!(reference.digest.is_some());
///
/// // Full reference with domain, tag, and digest
/// let reference: Reference = "docker.io/library/alpine:3.18@sha256:0123456789abcdef0123456789abcdef".parse().unwrap();
/// assert!(reference.name.domain.is_some());
/// assert_eq!(reference.tag.as_ref().map(|tag| tag.as_str()), Some("3.18"));
/// assert!(reference.digest.is_some());
///
/// // Display roundtrip
/// let input = "docker.io/library/alpine:latest";
/// let reference: Reference = input.parse().unwrap();
/// assert_eq!(reference.to_string(), input);
///
/// // Rejects empty input
/// assert_eq!(
///     "".parse::<Reference>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"\", code: TakeWhile1 }"
/// );
///
/// // Rejects uppercase name
/// assert_eq!(
///     "ALPINE".parse::<Reference>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"ALPINE\", code: TakeWhile1 }"
/// );
///
/// // Rejects trailing colon
/// assert_eq!(
///     "alpine:".parse::<Reference>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \":\", code: Eof }"
/// );
///
/// // Rejects trailing @
/// assert_eq!(
///     "alpine@".parse::<Reference>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"@\", code: Eof }"
/// );
///
/// // Rejects invalid digest
/// assert_eq!(
///     "alpine@sha256:tooshort".parse::<Reference>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"@sha256:tooshort\", code: Eof }"
/// );
///
/// // Rejects special characters in name
/// assert_eq!(
///     "alpine!latest".parse::<Reference>().unwrap_err(),
///     "parse error: Parsing Error: Error { input: \"!latest\", code: Eof }"
/// );
/// ```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Reference {
    pub name: Name,
    pub tag: Option<Tag>,
    pub digest: Option<Digest>,
}

impl Parse for Reference {
    fn parse(input: &str) -> IResult<&str, Self> {
        (
            Name::parse,
            opt(preceded(char(':'), Tag::parse)),
            opt(preceded(char('@'), Digest::parse)),
        )
            .map(|(name, tag, digest)| Self { name, tag, digest })
            .parse(input)
    }
}

impl std::fmt::Display for Reference {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.name)?;
        if let Some(tag) = &self.tag {
            write!(formatter, ":{tag}")?;
        }
        if let Some(digest) = &self.digest {
            write!(formatter, "@{digest}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_is_registry_with_dots() {
        let domain: Domain = "docker.io".parse().unwrap();
        assert!(domain.is_registry());
    }

    #[test]
    fn test_domain_is_registry_with_port() {
        let domain: Domain = "myhost:5000".parse().unwrap();
        assert!(domain.is_registry());
    }

    #[test]
    fn test_domain_is_registry_localhost() {
        let domain: Domain = "localhost".parse().unwrap();
        assert!(domain.is_registry());
    }

    #[test]
    fn test_domain_is_registry_localhost_with_port() {
        let domain: Domain = "localhost:5000".parse().unwrap();
        assert!(domain.is_registry());
    }

    #[test]
    fn test_domain_is_registry_ipv4() {
        let domain: Domain = "192.168.1.1".parse().unwrap();
        assert!(domain.is_registry());
    }

    #[test]
    fn test_domain_is_not_registry_single_component() {
        let domain: Domain = "myproject".parse().unwrap();
        assert!(!domain.is_registry());
    }

    #[test]
    fn test_name_single_component_not_domain() {
        let name: Name = "pg-ephemeral/main".parse().unwrap();
        assert!(name.domain.is_none());
        assert_eq!(name.path.to_string(), "pg-ephemeral/main");
    }

    #[test]
    fn test_name_localhost_is_domain() {
        let name: Name = "localhost/pg-ephemeral/main".parse().unwrap();
        assert_eq!(name.domain.unwrap().to_string(), "localhost");
        assert_eq!(name.path.to_string(), "pg-ephemeral/main");
    }

    #[test]
    fn test_name_dotted_is_domain() {
        let name: Name = "docker.io/library/alpine".parse().unwrap();
        assert_eq!(name.domain.unwrap().to_string(), "docker.io");
        assert_eq!(name.path.to_string(), "library/alpine");
    }

    #[test]
    fn test_name_with_port_is_domain() {
        let name: Name = "myhost:5000/myimage".parse().unwrap();
        assert_eq!(name.domain.unwrap().to_string(), "myhost:5000");
        assert_eq!(name.path.to_string(), "myimage");
    }

    #[test]
    fn test_name_bare_path() {
        let name: Name = "alpine".parse().unwrap();
        assert!(name.domain.is_none());
        assert_eq!(name.path.to_string(), "alpine");
    }

    #[test]
    fn test_name_multi_segment_path_no_domain() {
        let name: Name = "myorg/myproject/myimage".parse().unwrap();
        assert!(name.domain.is_none());
        assert_eq!(name.path.to_string(), "myorg/myproject/myimage");
    }

    #[test]
    fn test_name_deserialize_from_string() {
        let name: Name = serde_json::from_str("\"ghcr.io/myorg\"").unwrap();
        assert_eq!(name.domain.unwrap().to_string(), "ghcr.io");
        assert_eq!(name.path.to_string(), "myorg");
    }

    #[test]
    fn test_name_deserialize_invalid_string_fails() {
        let result: Result<Name, _> = serde_json::from_str("\"bad name with spaces\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_path_component_from_static_or_panic_const_context() {
        // Exercise the const constructor in a const context — this would
        // fail at compile time if the grammar rejected the input.
        const PLAIN: PathComponent = PathComponent::from_static_or_panic("alpine");
        const WITH_DASH: PathComponent = PathComponent::from_static_or_panic("pg-ephemeral");
        const WITH_UNDERSCORE: PathComponent = PathComponent::from_static_or_panic("foo_bar");
        const WITH_DOT: PathComponent = PathComponent::from_static_or_panic("foo.bar");
        const WITH_DOUBLE_UNDERSCORE: PathComponent =
            PathComponent::from_static_or_panic("foo__bar");

        assert_eq!(PLAIN.as_str(), "alpine");
        assert_eq!(WITH_DASH.as_str(), "pg-ephemeral");
        assert_eq!(WITH_UNDERSCORE.as_str(), "foo_bar");
        assert_eq!(WITH_DOT.as_str(), "foo.bar");
        assert_eq!(WITH_DOUBLE_UNDERSCORE.as_str(), "foo__bar");
    }

    #[test]
    #[should_panic(expected = "path component must start with [a-z0-9]")]
    fn test_path_component_from_static_or_panic_rejects_leading_dash() {
        let _ = PathComponent::from_static_or_panic("-alpine");
    }

    #[test]
    #[should_panic(expected = "path component has trailing input")]
    fn test_path_component_from_static_or_panic_rejects_trailing_dash() {
        // "foo-" consumes as "foo" (grammar rewinds the orphan dash),
        // leaving a non-zero trailing remainder that `from_static_or_panic`
        // refuses.
        let _ = PathComponent::from_static_or_panic("foo-");
    }

    #[test]
    #[should_panic(expected = "path component must start with [a-z0-9]")]
    fn test_path_component_from_static_or_panic_rejects_empty() {
        let _ = PathComponent::from_static_or_panic("");
    }
}
