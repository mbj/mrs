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
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while, take_while_m_n, take_while1},
    character::complete::{char, digit1},
    combinator::{all_consuming, map, opt, recognize, verify},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, pair, preceded, tuple},
};

pub(crate) trait Parse: Sized {
    fn parse(input: &str) -> IResult<&str, Self>;
}

macro_rules! impl_from_str {
    ($($type:ty),* $(,)?) => {
        $(
            impl std::str::FromStr for $type {
                type Err = String;

                fn from_str(string: &str) -> Result<Self, Self::Err> {
                    match all_consuming(<Self as Parse>::parse)(string) {
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
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DomainComponent(String);

impl DomainComponent {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Parse for DomainComponent {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            recognize(pair(
                take_while_m_n(1, 1, |character: char| character.is_ascii_alphanumeric()),
                take_while(|character: char| character.is_ascii_alphanumeric() || character == '-'),
            )),
            |string: &str| {
                let trimmed = string.trim_end_matches('-');
                Self(trimmed.to_string())
            },
        )(input)
    }
}

#[cfg(test)]
mod domain_component_tests {
    use super::*;

    #[test]
    fn rejects_empty_input() {
        assert_eq!(
            DomainComponent::parse("").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"\", code: TakeWhileMN }"
        );
    }

    #[test]
    fn rejects_leading_hyphen() {
        assert_eq!(
            DomainComponent::parse("-example").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"-example\", code: TakeWhileMN }"
        );
    }

    #[test]
    fn rejects_special_characters() {
        assert_eq!(
            DomainComponent::parse("_invalid").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"_invalid\", code: TakeWhileMN }"
        );
        assert_eq!(
            DomainComponent::parse("@invalid").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"@invalid\", code: TakeWhileMN }"
        );
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
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DomainName(Vec<DomainComponent>);

impl DomainName {
    pub fn components(&self) -> &[DomainComponent] {
        &self.0
    }
}

impl Parse for DomainName {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(char('.'), DomainComponent::parse), Self)(input)
    }
}

impl std::fmt::Display for DomainName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<&str> = self.0.iter().map(|component| component.as_str()).collect();
        write!(formatter, "{}", parts.join("."))
    }
}

#[cfg(test)]
mod domain_name_tests {
    use super::*;

    #[test]
    fn rejects_empty_input() {
        assert_eq!(
            DomainName::parse("").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"\", code: TakeWhileMN }"
        );
    }

    #[test]
    fn rejects_leading_dot() {
        assert_eq!(
            DomainName::parse(".example.com").unwrap_err().to_string(),
            "Parsing Error: Error { input: \".example.com\", code: TakeWhileMN }"
        );
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
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PortNumber(u16);

impl PortNumber {
    pub fn value(&self) -> u16 {
        self.0
    }
}

impl Parse for PortNumber {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            verify(digit1, |string: &str| string.len() <= 5),
            |string: &str| Self(string.parse().unwrap_or(0)),
        )(input)
    }
}

impl std::fmt::Display for PortNumber {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

#[cfg(test)]
mod port_number_tests {
    use super::*;

    #[test]
    fn rejects_empty_input() {
        assert_eq!(
            PortNumber::parse("").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"\", code: Digit }"
        );
    }

    #[test]
    fn rejects_non_numeric() {
        assert_eq!(
            PortNumber::parse("abc").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"abc\", code: Digit }"
        );
    }

    #[test]
    fn rejects_too_many_digits() {
        assert_eq!(
            PortNumber::parse("123456").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"123456\", code: Verify }"
        );
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
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Host {
    DomainName(DomainName),
    Ipv4(std::net::Ipv4Addr),
    Ipv6(std::net::Ipv6Addr),
}

impl Host {
    fn parse_ipv4(input: &str) -> IResult<&str, std::net::Ipv4Addr> {
        let (remaining, address_string) = recognize(tuple((
            digit1,
            char('.'),
            digit1,
            char('.'),
            digit1,
            char('.'),
            digit1,
        )))(input)?;

        match address_string.parse() {
            Ok(address) => Ok((remaining, address)),
            Err(_) => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            ))),
        }
    }

    fn parse_ipv6(input: &str) -> IResult<&str, std::net::Ipv6Addr> {
        let (remaining, address_string) = delimited(
            char('['),
            take_while1(|character: char| character.is_ascii_hexdigit() || character == ':'),
            char(']'),
        )(input)?;

        match address_string.parse() {
            Ok(address) => Ok((remaining, address)),
            Err(_) => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            ))),
        }
    }
}

impl Parse for Host {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(Self::parse_ipv6, Self::Ipv6),
            map(Self::parse_ipv4, Self::Ipv4),
            map(DomainName::parse, Self::DomainName),
        ))(input)
    }
}

impl std::fmt::Display for Host {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DomainName(name) => write!(formatter, "{}", name),
            Self::Ipv4(address) => write!(formatter, "{}", address),
            Self::Ipv6(address) => write!(formatter, "[{}]", address),
        }
    }
}

#[cfg(test)]
mod host_tests {
    use super::*;

    #[test]
    fn rejects_empty_input() {
        assert_eq!(
            Host::parse("").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"\", code: TakeWhileMN }"
        );
    }

    #[test]
    fn rejects_unclosed_ipv6_bracket() {
        assert_eq!(
            Host::parse("[::1").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"[::1\", code: TakeWhileMN }"
        );
    }

    #[test]
    fn rejects_invalid_ipv4() {
        let (remaining, host) = Host::parse("999.999.999.999").unwrap();
        // Parses as domain name, not IPv4
        assert!(matches!(host, Host::DomainName(_)));
        assert_eq!(remaining, "");
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Domain {
    pub host: Host,
    pub port: Option<PortNumber>,
}

impl Parse for Domain {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            pair(Host::parse, opt(preceded(char(':'), PortNumber::parse))),
            |(host, port)| Self { host, port },
        )(input)
    }
}

impl std::fmt::Display for Domain {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.host)?;
        if let Some(port) = &self.port {
            write!(formatter, ":{}", port)?;
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
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathComponent(String);

impl PathComponent {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Pattern: `[a-z0-9]+`
    fn parse_alpha_numeric(input: &str) -> IResult<&str, &str> {
        take_while1(|character: char| character.is_ascii_lowercase() || character.is_ascii_digit())(
            input,
        )
    }

    /// Pattern: `[_.]|__|[-]*`
    fn parse_separator(input: &str) -> IResult<&str, &str> {
        alt((tag("__"), tag("_"), tag("."), recognize(many1(char('-')))))(input)
    }
}

impl Parse for PathComponent {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            recognize(pair(
                Self::parse_alpha_numeric,
                many0(pair(Self::parse_separator, Self::parse_alpha_numeric)),
            )),
            |string: &str| Self(string.to_string()),
        )(input)
    }
}

impl std::fmt::Display for PathComponent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

#[cfg(test)]
mod path_component_tests {
    use super::*;

    #[test]
    fn rejects_empty_input() {
        assert_eq!(
            PathComponent::parse("").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"\", code: TakeWhile1 }"
        );
    }

    #[test]
    fn rejects_uppercase() {
        assert_eq!(
            PathComponent::parse("ALPINE").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"ALPINE\", code: TakeWhile1 }"
        );
    }

    #[test]
    fn rejects_leading_separator() {
        assert_eq!(
            PathComponent::parse("_alpine").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"_alpine\", code: TakeWhile1 }"
        );
        assert_eq!(
            PathComponent::parse(".alpine").unwrap_err().to_string(),
            "Parsing Error: Error { input: \".alpine\", code: TakeWhile1 }"
        );
        assert_eq!(
            PathComponent::parse("-alpine").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"-alpine\", code: TakeWhile1 }"
        );
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
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path(Vec<PathComponent>);

impl Path {
    pub fn components(&self) -> &[PathComponent] {
        &self.0
    }
}

impl Parse for Path {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(char('/'), PathComponent::parse), Self)(input)
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<&str> = self.0.iter().map(|component| component.as_str()).collect();
        write!(formatter, "{}", parts.join("/"))
    }
}

#[cfg(test)]
mod path_tests {
    use super::*;

    #[test]
    fn rejects_empty_input() {
        assert_eq!(
            Path::parse("").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"\", code: TakeWhile1 }"
        );
    }

    #[test]
    fn rejects_leading_slash() {
        assert_eq!(
            Path::parse("/alpine").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"/alpine\", code: TakeWhile1 }"
        );
    }

    #[test]
    fn rejects_double_slash() {
        let (remaining, path) = Path::parse("library//alpine").unwrap();
        assert_eq!(path.to_string(), "library");
        assert_eq!(remaining, "//alpine");
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
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Name {
    pub domain: Option<Domain>,
    pub path: Path,
}

impl Name {
    fn looks_like_domain(string: &str) -> bool {
        string.contains('.') || string.contains(':') || string == "localhost"
    }
}

impl Parse for Name {
    fn parse(input: &str) -> IResult<&str, Self> {
        // Try to parse domain followed by '/' and path
        let domain_path_result: IResult<&str, Self> = map(
            pair(
                verify(
                    |input| {
                        let (remaining, domain) = Domain::parse(input)?;
                        if remaining.starts_with('/')
                            && Self::looks_like_domain(&domain.to_string())
                        {
                            Ok((remaining, domain))
                        } else {
                            Err(nom::Err::Error(nom::error::Error::new(
                                input,
                                nom::error::ErrorKind::Verify,
                            )))
                        }
                    },
                    |_: &Domain| true,
                ),
                preceded(char('/'), Path::parse),
            ),
            |(domain, path)| Self {
                domain: Some(domain),
                path,
            },
        )(input);

        match domain_path_result {
            Ok(result) => Ok(result),
            Err(_) => map(Path::parse, |path| Self { domain: None, path })(input),
        }
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(domain) = &self.domain {
            write!(formatter, "{}/", domain)?;
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
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tag(String);

impl Tag {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Parse for Tag {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            verify(
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
                )),
                |string: &str| string.len() <= 128,
            ),
            |string: &str| Self(string.to_string()),
        )(input)
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl From<sha2::digest::Output<sha2::Sha256>> for Tag {
    fn from(hash: sha2::digest::Output<sha2::Sha256>) -> Self {
        Self(format!("{:x}", hash))
    }
}

#[cfg(test)]
mod tag_tests {
    use super::*;

    #[test]
    fn rejects_empty_input() {
        assert_eq!(
            Tag::parse("").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"\", code: TakeWhileMN }"
        );
    }

    #[test]
    fn rejects_leading_dot() {
        assert_eq!(
            Tag::parse(".latest").unwrap_err().to_string(),
            "Parsing Error: Error { input: \".latest\", code: TakeWhileMN }"
        );
    }

    #[test]
    fn rejects_leading_hyphen() {
        assert_eq!(
            Tag::parse("-latest").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"-latest\", code: TakeWhileMN }"
        );
    }

    #[test]
    fn accepts_leading_underscore() {
        let (remaining, tag) = Tag::parse("_latest").unwrap();
        assert_eq!(tag.as_str(), "_latest");
        assert_eq!(remaining, "");
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DigestAlgorithm(String);

impl DigestAlgorithm {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Pattern: `[A-Za-z][A-Za-z0-9]*`
    fn parse_component(input: &str) -> IResult<&str, &str> {
        recognize(pair(
            take_while_m_n(1, 1, |character: char| character.is_ascii_alphabetic()),
            take_while(|character: char| character.is_ascii_alphanumeric()),
        ))(input)
    }

    /// Pattern: `[+.-_]`
    fn parse_separator(input: &str) -> IResult<&str, char> {
        alt((char('+'), char('.'), char('-'), char('_')))(input)
    }
}

impl Parse for DigestAlgorithm {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            recognize(pair(
                Self::parse_component,
                many0(pair(Self::parse_separator, Self::parse_component)),
            )),
            |string: &str| Self(string.to_string()),
        )(input)
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
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
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
        )(input)
    }
}

impl Parse for Digest {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((DigestAlgorithm::parse, char(':'), Self::parse_hex)),
            |(algorithm, _, hex)| Self {
                algorithm,
                hex: hex.to_string(),
            },
        )(input)
    }
}

impl std::fmt::Display for Digest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}:{}", self.algorithm, self.hex)
    }
}

#[cfg(test)]
mod digest_tests {
    use super::*;

    #[test]
    fn rejects_empty_input() {
        assert_eq!(
            Digest::parse("").unwrap_err().to_string(),
            "Parsing Error: Error { input: \"\", code: TakeWhileMN }"
        );
    }

    #[test]
    fn rejects_missing_colon() {
        assert_eq!(
            Digest::parse("sha2560123456789abcdef0123456789abcdef")
                .unwrap_err()
                .to_string(),
            "Parsing Error: Error { input: \"\", code: Char }"
        );
    }

    #[test]
    fn rejects_short_hex() {
        assert_eq!(
            Digest::parse("sha256:0123456789abcdef")
                .unwrap_err()
                .to_string(),
            "Parsing Error: Error { input: \"0123456789abcdef\", code: Verify }"
        );
    }

    #[test]
    fn stops_at_non_hex_characters() {
        let (remaining, digest) = Digest::parse(
            "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdefgh",
        )
        .unwrap();
        // Stops at 'g', leaving "gh" as remaining
        assert_eq!(
            digest.hex,
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        );
        assert_eq!(remaining, "gh");
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
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reference {
    pub name: Name,
    pub tag: Option<Tag>,
    pub digest: Option<Digest>,
}

impl Parse for Reference {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                Name::parse,
                opt(preceded(char(':'), Tag::parse)),
                opt(preceded(char('@'), Digest::parse)),
            )),
            |(name, tag, digest)| Self { name, tag, digest },
        )(input)
    }
}

impl std::fmt::Display for Reference {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.name)?;
        if let Some(tag) = &self.tag {
            write!(formatter, ":{}", tag)?;
        }
        if let Some(digest) = &self.digest {
            write!(formatter, "@{}", digest)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod reference_tests {
    use super::*;

    #[test]
    fn rejects_empty_input() {
        assert_eq!(
            "".parse::<Reference>().unwrap_err(),
            "parse error: Parsing Error: Error { input: \"\", code: TakeWhile1 }"
        );
    }

    #[test]
    fn rejects_uppercase_name() {
        assert_eq!(
            "ALPINE".parse::<Reference>().unwrap_err(),
            "parse error: Parsing Error: Error { input: \"ALPINE\", code: TakeWhile1 }"
        );
    }

    #[test]
    fn rejects_trailing_colon() {
        assert_eq!(
            "alpine:".parse::<Reference>().unwrap_err(),
            "parse error: Parsing Error: Error { input: \":\", code: Eof }"
        );
    }

    #[test]
    fn rejects_trailing_at() {
        assert_eq!(
            "alpine@".parse::<Reference>().unwrap_err(),
            "parse error: Parsing Error: Error { input: \"@\", code: Eof }"
        );
    }

    #[test]
    fn rejects_invalid_digest() {
        assert_eq!(
            "alpine@sha256:tooshort".parse::<Reference>().unwrap_err(),
            "parse error: Parsing Error: Error { input: \"@sha256:tooshort\", code: Eof }"
        );
    }

    #[test]
    fn rejects_special_characters_in_name() {
        assert_eq!(
            "alpine!latest".parse::<Reference>().unwrap_err(),
            "parse error: Parsing Error: Error { input: \"!latest\", code: Eof }"
        );
    }
}
