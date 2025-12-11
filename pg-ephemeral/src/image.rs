/// Postgresql images supported, references images from <https://hub.docker.com/_/postgres>
#[derive(Clone, Debug, PartialEq)]
pub enum Image {
    /// Official release
    OfficialRelease {
        major: Major,
        minor: Minor,
        os: OS,
        digest: Option<Digest>,
    },
    /// OfficialRelease candidate
    OfficialReleaseCandidate {
        major: Major,
        number: ReleaseCandidateNumber,
        os: OS,
        digest: Option<Digest>,
    },
    /// Latest image on docker.com
    ///
    /// Only use that one for quick and dirty testing, it's recommended to always pin
    /// specific images in config files. Also note that pg-ephemeral currently never refreshes
    /// `latest` once cached in the local registry it's never refreshed.
    OfficialLatest { os: OS, digest: Option<Digest> },
    /// Explicit OCI image reference, bypassing the official postgres image naming
    Explicit(ociman::image::Reference),
}

impl std::default::Default for Image {
    fn default() -> Self {
        Self::OfficialLatest {
            os: OS::Default,
            digest: None,
        }
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OfficialRelease {
                major,
                minor,
                os,
                digest,
            } => {
                write!(formatter, "{major}{minor}{os}")?;
                if let Some(digest) = digest {
                    write!(formatter, "@{digest}")?;
                }
                Ok(())
            }
            Self::OfficialReleaseCandidate {
                major,
                number,
                os,
                digest,
            } => {
                write!(formatter, "{major}rc{number}{os}")?;
                if let Some(digest) = digest {
                    write!(formatter, "@{digest}")?;
                }
                Ok(())
            }
            Self::OfficialLatest { os, digest } => {
                match os {
                    OS::Default => write!(formatter, "latest")?,
                    OS::Explicit(value) => write!(formatter, "{value}")?,
                }
                if let Some(digest) = digest {
                    write!(formatter, "@{digest}")?;
                }
                Ok(())
            }
            Self::Explicit(image) => write!(formatter, "{image}"),
        }
    }
}

impl std::str::FromStr for Image {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use nom::{
            Finish, IResult, Parser,
            branch::alt,
            bytes::complete::{tag, take_while_m_n, take_while1},
            character::complete::digit1,
            combinator::{cut, opt, recognize},
            error::context,
            sequence::{pair, preceded},
        };
        use nom_language::error::VerboseError;

        type ParseResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

        fn os_name(input: &str) -> ParseResult<'_, &str> {
            context(
                "OS name",
                recognize(pair(
                    take_while_m_n(1, 1, |ch: char| ch.is_ascii_lowercase()),
                    take_while1(|ch: char| {
                        ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '.'
                    }),
                )),
            )
            .parse(input)
        }

        fn os_suffix(input: &str) -> ParseResult<'_, OS> {
            context(
                "OS suffix",
                preceded(tag("-"), os_name).map(|name: &str| OS::Explicit(name.to_string())),
            )
            .parse(input)
        }

        fn digest(input: &str) -> ParseResult<'_, Digest> {
            context(
                "digest",
                preceded(
                    tag("@sha256:"),
                    cut(take_while_m_n(64, 64, |ch: char| ch.is_ascii_hexdigit())),
                )
                .map_res(|hash: &str| {
                    hex::decode(hash)
                        .map_err(|err| format!("invalid hex: {err}"))
                        .and_then(|bytes| {
                            bytes
                                .try_into()
                                .map(Digest)
                                .map_err(|_| "hash must be exactly 32 bytes".to_string())
                        })
                }),
            )
            .parse(input)
        }

        fn latest(input: &str) -> ParseResult<'_, Image> {
            context(
                "latest image",
                (tag("latest"), opt(digest)).map(|(_, digest)| Image::OfficialLatest {
                    os: OS::Default,
                    digest,
                }),
            )
            .parse(input)
        }

        fn os_only(input: &str) -> ParseResult<'_, Image> {
            context(
                "OS-only image",
                (os_name, opt(digest)).map(|(os, digest)| Image::OfficialLatest {
                    os: OS::Explicit(os.to_string()),
                    digest,
                }),
            )
            .parse(input)
        }

        fn release_candidate(input: &str) -> ParseResult<'_, Image> {
            context(
                "release candidate image",
                (
                    digit1.map_res(|digits: &str| digits.parse::<u8>().map(Major)),
                    preceded(
                        tag("rc"),
                        digit1.map_res(|digits: &str| {
                            digits
                                .parse::<std::num::NonZero<u8>>()
                                .map(ReleaseCandidateNumber)
                        }),
                    ),
                    opt(os_suffix),
                    opt(digest),
                )
                    .map(|(major, number, os, digest)| {
                        Image::OfficialReleaseCandidate {
                            major,
                            number,
                            os: os.unwrap_or(OS::Default),
                            digest,
                        }
                    }),
            )
            .parse(input)
        }

        fn official_release(input: &str) -> ParseResult<'_, Image> {
            context(
                "official release image",
                (
                    digit1.map_res(|digits: &str| digits.parse::<u8>().map(Major)),
                    opt(preceded(
                        tag("."),
                        digit1.map_res(|digits: &str| digits.parse::<u8>().map(Minor::Explicit)),
                    )),
                    opt(os_suffix),
                    opt(digest),
                )
                    .map(|(major, minor, os, digest)| Image::OfficialRelease {
                        major,
                        minor: minor.unwrap_or(Minor::Latest),
                        os: os.unwrap_or(OS::Default),
                        digest,
                    }),
            )
            .parse(input)
        }

        fn image(input: &str) -> ParseResult<'_, Image> {
            alt((latest, release_candidate, official_release, os_only)).parse(input)
        }

        match image(value).finish() {
            Ok(("", result)) => Ok(result),
            Ok((remaining, _)) => Err(format!("unexpected trailing input: '{remaining}'")),
            Err(error) => Err(nom_language::error::convert_error(value, error)),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Image {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::de::Deserialize<'de>>::deserialize(deserializer)
            .and_then(|value| value.parse().map_err(serde::de::Error::custom))
    }
}

/// ```
/// let explicit = pg_ephemeral::Image::Explicit("my-registry.com/postgres:17".parse().unwrap());
/// let reference: ociman::image::Reference = (&explicit).into();
/// assert_eq!(reference.to_string(), "my-registry.com/postgres:17");
/// ```
impl From<&Image> for ociman::image::Reference {
    fn from(image: &Image) -> Self {
        match image {
            Image::Explicit(reference) => reference.clone(),
            Image::OfficialRelease { .. }
            | Image::OfficialReleaseCandidate { .. }
            | Image::OfficialLatest { .. } => {
                format!("registry.hub.docker.com/library/postgres:{image}")
                    .parse()
                    .unwrap()
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Major(u8);

impl std::fmt::Display for Major {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl Major {
    pub const fn new(value: u8) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Minor {
    Explicit(u8),
    Latest,
}

impl std::fmt::Display for Minor {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Explicit(number) => write!(formatter, ".{number}"),
            Self::Latest => write!(formatter, ""),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReleaseCandidateNumber(std::num::NonZero<u8>);

impl ReleaseCandidateNumber {
    pub const fn new(value: std::num::NonZero<u8>) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for ReleaseCandidateNumber {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Operating system variant for PostgreSQL Docker images
#[derive(Clone, Debug, PartialEq)]
pub enum OS {
    Default,
    Explicit(String),
}

impl std::fmt::Display for OS {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Default => write!(formatter, ""),
            Self::Explicit(value) => write!(formatter, "-{value}"),
        }
    }
}

/// Docker image digest for pinning images to specific SHA256 hashes
#[derive(Clone, Debug, PartialEq)]
pub struct Digest([u8; 32]);

impl std::fmt::Display for Digest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "sha256:{}", hex::encode(self.0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_image_string() {
        assert_image(
            "latest",
            &Image::OfficialLatest {
                os: OS::Default,
                digest: None,
            },
        );

        assert_image(
            "trixie",
            &Image::OfficialLatest {
                os: OS::Explicit("trixie".to_string()),
                digest: None,
            },
        );

        assert_image(
            "18rc1",
            &Image::OfficialReleaseCandidate {
                major: Major(18),
                number: ReleaseCandidateNumber(1u8.try_into().unwrap()),
                os: OS::Default,
                digest: None,
            },
        );

        assert_image(
            "18rc1-trixie",
            &Image::OfficialReleaseCandidate {
                major: Major(18),
                number: ReleaseCandidateNumber(1u8.try_into().unwrap()),
                os: OS::Explicit("trixie".to_string()),
                digest: None,
            },
        );

        assert_image(
            "18rc1-bookworm",
            &Image::OfficialReleaseCandidate {
                major: Major(18),
                number: ReleaseCandidateNumber(1u8.try_into().unwrap()),
                os: OS::Explicit("bookworm".to_string()),
                digest: None,
            },
        );

        assert_image(
            "18rc1-alpine3.22",
            &Image::OfficialReleaseCandidate {
                major: Major(18),
                number: ReleaseCandidateNumber(1u8.try_into().unwrap()),
                os: OS::Explicit("alpine3.22".to_string()),
                digest: None,
            },
        );

        assert_image(
            "18rc1-alpine3.21",
            &Image::OfficialReleaseCandidate {
                major: Major(18),
                number: ReleaseCandidateNumber(1u8.try_into().unwrap()),
                os: OS::Explicit("alpine3.21".to_string()),
                digest: None,
            },
        );

        assert_image(
            "18rc1-alpine",
            &Image::OfficialReleaseCandidate {
                major: Major(18),
                number: ReleaseCandidateNumber(1u8.try_into().unwrap()),
                os: OS::Explicit("alpine".to_string()),
                digest: None,
            },
        );

        assert_image(
            "17",
            &Image::OfficialRelease {
                major: Major(17),
                minor: Minor::Latest,
                os: OS::Default,
                digest: None,
            },
        );

        assert_image(
            "17-trixie",
            &Image::OfficialRelease {
                major: Major(17),
                minor: Minor::Latest,
                os: OS::Explicit("trixie".to_string()),
                digest: None,
            },
        );

        assert_image(
            "17.6",
            &Image::OfficialRelease {
                major: Major(17),
                minor: Minor::Explicit(6),
                os: OS::Default,
                digest: None,
            },
        );

        assert_image(
            "17.6-trixie",
            &Image::OfficialRelease {
                major: Major(17),
                minor: Minor::Explicit(6),
                os: OS::Explicit("trixie".to_string()),
                digest: None,
            },
        );
    }

    fn assert_image(syntax: &str, expected: &Image) {
        assert_eq!(syntax.parse().as_ref(), Ok(expected), "parses: {syntax:#?}");
        assert_eq!(format!("{expected}"), syntax, "generates: {syntax:#?}");
    }

    #[test]
    fn test_image_with_digest() {
        let hash = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let parsed_digest = Some(Digest(hex::decode(hash).unwrap().try_into().unwrap()));

        // Test OfficialRelease with digest
        assert_image(
            "17.6@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            &Image::OfficialRelease {
                major: Major(17),
                minor: Minor::Explicit(6),
                os: OS::Default,
                digest: parsed_digest.clone(),
            },
        );

        assert_image(
            "17.6-trixie@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            &Image::OfficialRelease {
                major: Major(17),
                minor: Minor::Explicit(6),
                os: OS::Explicit("trixie".to_string()),
                digest: parsed_digest.clone(),
            },
        );

        assert_image(
            "17@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            &Image::OfficialRelease {
                major: Major(17),
                minor: Minor::Latest,
                os: OS::Default,
                digest: parsed_digest.clone(),
            },
        );

        // Test OfficialReleaseCandidate with digest
        assert_image(
            "18rc1@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            &Image::OfficialReleaseCandidate {
                major: Major(18),
                number: ReleaseCandidateNumber(1u8.try_into().unwrap()),
                os: OS::Default,
                digest: parsed_digest.clone(),
            },
        );

        assert_image(
            "18rc1-alpine@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            &Image::OfficialReleaseCandidate {
                major: Major(18),
                number: ReleaseCandidateNumber(1u8.try_into().unwrap()),
                os: OS::Explicit("alpine".to_string()),
                digest: parsed_digest.clone(),
            },
        );

        // Test OfficialLatest with digest
        assert_image(
            "latest@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            &Image::OfficialLatest {
                os: OS::Default,
                digest: parsed_digest.clone(),
            },
        );

        assert_image(
            "trixie@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            &Image::OfficialLatest {
                os: OS::Explicit("trixie".to_string()),
                digest: parsed_digest.clone(),
            },
        );
    }

    #[test]
    fn test_ociman_image_conversion_with_digest() {
        let hash = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let image = Image::OfficialRelease {
            major: Major(17),
            minor: Minor::Explicit(6),
            os: OS::Default,
            digest: Some(Digest(hex::decode(hash).unwrap().try_into().unwrap())),
        };

        let reference: ociman::image::Reference = (&image).into();
        let expected = "registry.hub.docker.com/library/postgres:17.6@sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

        assert_eq!(reference.to_string(), expected);
    }

    #[test]
    fn test_parse_error_uppercase() {
        let error = "LATEST".parse::<Image>().unwrap_err();
        let expected = indoc::indoc! {"
            0: at line 1, in TakeWhileMN:
            LATEST
            ^

            1: at line 1, in OS name:
            LATEST
            ^

            2: at line 1, in OS-only image:
            LATEST
            ^

            3: at line 1, in Alt:
            LATEST
            ^

        "};
        assert_eq!(error, expected);
    }

    #[test]
    fn test_parse_error_invalid_rc() {
        let error = "17rc".parse::<Image>().unwrap_err();
        let expected = "unexpected trailing input: 'rc'";
        assert_eq!(error, expected);
    }

    #[test]
    fn test_parse_error_short_digest() {
        let error = "17@sha256:abc".parse::<Image>().unwrap_err();
        let expected = indoc::indoc! {"
            0: at line 1, in TakeWhileMN:
            17@sha256:abc
                      ^

            1: at line 1, in digest:
            17@sha256:abc
              ^

            2: at line 1, in official release image:
            17@sha256:abc
            ^

        "};
        assert_eq!(error, expected);
    }

    #[test]
    fn test_parse_error_trailing_dash() {
        let error = "17-".parse::<Image>().unwrap_err();
        let expected = "unexpected trailing input: '-'";
        assert_eq!(error, expected);
    }

    #[test]
    fn test_parse_error_trailing_content() {
        let error = "17.6.5".parse::<Image>().unwrap_err();
        let expected = "unexpected trailing input: '.5'";
        assert_eq!(error, expected);
    }

    #[test]
    fn test_parse_error_invalid_os_name() {
        let error = "17-9invalid".parse::<Image>().unwrap_err();
        let expected = "unexpected trailing input: '-9invalid'";
        assert_eq!(error, expected);
    }
}
