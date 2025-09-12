/// Postgresql images supported, references images from https://hub.docker.com/_/postgres
#[derive(Clone, Debug, PartialEq)]
pub enum Image {
    /// Official release
    OfficialRelease { major: Major, minor: Minor, os: OS },
    /// OfficialRelease candidate
    OfficialReleaseCandidate {
        major: Major,
        number: ReleaseCandidateNumber,
        os: OS,
    },
    /// Latest image on docker.com
    ///
    /// Only use that one for quickand dirty testing, its recommended to always pin
    /// specific images in config files. Also note that pg-ephemeral currently never refreshes
    /// `latest` once cached in the local registry its never refreshed.
    OfficialLatest { os: OS },
}

impl std::default::Default for Image {
    fn default() -> Self {
        Self::OfficialLatest { os: OS::Default }
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OfficialRelease { major, minor, os } => {
                write!(formatter, "{}{}{}", major, minor, os)
            }
            Self::OfficialReleaseCandidate { major, number, os } => {
                write!(formatter, "{}rc{}{}", major, number, os)
            }
            Self::OfficialLatest { os } => match os {
                OS::Default => write!(formatter, "latest"),
                OS::Explicit(value) => write!(formatter, "{value}"),
            },
        }
    }
}

impl std::str::FromStr for Image {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        fn parse_field<T, N: std::str::FromStr>(
            constructor: fn(N) -> T,
            field: &str,
            captures: &regex_lite::Captures,
        ) -> Result<T, String>
        where
            <N as std::str::FromStr>::Err: std::fmt::Display,
        {
            match captures.name(field).unwrap().as_str().parse() {
                Ok(value) => Ok(constructor(value)),
                Err(error) => Err(format!(
                    "Cannot parse image component {field}, error: {error}"
                )),
            }
        }

        if value == "latest" {
            return Ok(Self::OfficialLatest { os: OS::Default });
        }

        let os_pattern = r#"(?<os>[a-z]+(?:\d+\.\d+)?)"#;

        let os_regex = regex_lite::Regex::new(&format!(r#"\A{os_pattern}\z"#)).unwrap();

        if os_regex.is_match(value) {
            return Ok(Self::OfficialLatest {
                os: OS::Explicit(value.to_string()),
            });
        }

        let regex = regex_lite::Regex::new(&format!(
            r#"\A(?<major>[0-9]+)(?:\.(?<minor>\d+)|rc(?<number>[1-9]\d*))?(?:-{os_pattern})?\z"#
        ))
        .unwrap();

        match regex.captures(value) {
            None => Err("invalid image format".to_string()),
            Some(captures) => {
                let os = captures
                    .name("os")
                    .map_or(OS::Default, |value| OS::Explicit(value.as_str().into()));

                let major = parse_field(Major, "major", &captures)?;

                if let Some(capture) = captures.name("minor") {
                    let minor = Minor::Explicit(capture.as_str().parse().unwrap());
                    return Ok(Self::OfficialRelease { major, minor, os });
                }

                if let Some(capture) = captures.name("number") {
                    let number = ReleaseCandidateNumber(capture.as_str().parse().unwrap());
                    return Ok(Self::OfficialReleaseCandidate { major, number, os });
                }

                Ok(Self::OfficialRelease {
                    major,
                    minor: Minor::Latest,
                    os,
                })
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for Image {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::de::Deserialize<'de>>::deserialize(deserializer)
            .and_then(|value| value.parse().map_err(serde::de::Error::custom))
    }
}

use crate::cbt;

impl From<&Image> for cbt::Image {
    fn from(value: &Image) -> Self {
        cbt::Image::from(format!(
            "registry.hub.docker.com/library/postgres:{}",
            value
        ))
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

/// Postgresql images supported, references images from https://hub.docker.com/_/postgres
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_image_string() {
        assert_image("latest", &Image::OfficialLatest { os: OS::Default });

        assert_image(
            "trixie",
            &Image::OfficialLatest {
                os: OS::Explicit("trixie".to_string()),
            },
        );

        assert_image(
            "18rc1",
            &Image::OfficialReleaseCandidate {
                major: Major(18),
                number: ReleaseCandidateNumber(1u8.try_into().unwrap()),
                os: OS::Default,
            },
        );

        assert_image(
            "18rc1-trixie",
            &Image::OfficialReleaseCandidate {
                major: Major(18),
                number: ReleaseCandidateNumber(1u8.try_into().unwrap()),
                os: OS::Explicit("trixie".to_string()),
            },
        );

        assert_image(
            "18rc1-bookworm",
            &Image::OfficialReleaseCandidate {
                major: Major(18),
                number: ReleaseCandidateNumber(1u8.try_into().unwrap()),
                os: OS::Explicit("bookworm".to_string()),
            },
        );

        assert_image(
            "18rc1-alpine3.22",
            &Image::OfficialReleaseCandidate {
                major: Major(18),
                number: ReleaseCandidateNumber(1u8.try_into().unwrap()),
                os: OS::Explicit("alpine3.22".to_string()),
            },
        );

        assert_image(
            "18rc1-alpine3.21",
            &Image::OfficialReleaseCandidate {
                major: Major(18),
                number: ReleaseCandidateNumber(1u8.try_into().unwrap()),
                os: OS::Explicit("alpine3.21".to_string()),
            },
        );

        assert_image(
            "18rc1-alpine",
            &Image::OfficialReleaseCandidate {
                major: Major(18),
                number: ReleaseCandidateNumber(1u8.try_into().unwrap()),
                os: OS::Explicit("alpine".to_string()),
            },
        );

        assert_image(
            "17",
            &Image::OfficialRelease {
                major: Major(17),
                minor: Minor::Latest,
                os: OS::Default,
            },
        );

        assert_image(
            "17-trixie",
            &Image::OfficialRelease {
                major: Major(17),
                minor: Minor::Latest,
                os: OS::Explicit("trixie".to_string()),
            },
        );

        assert_image(
            "17.6",
            &Image::OfficialRelease {
                major: Major(17),
                minor: Minor::Explicit(6),
                os: OS::Default,
            },
        );

        assert_image(
            "17.6-trixie",
            &Image::OfficialRelease {
                major: Major(17),
                minor: Minor::Explicit(6),
                os: OS::Explicit("trixie".to_string()),
            },
        );
    }

    fn assert_image(syntax: &str, expected: &Image) {
        assert_eq!(syntax.parse().as_ref(), Ok(expected), "parses: {syntax:#?}");
        assert_eq!(format!("{expected}"), syntax, "generates: {syntax:#?}");
    }
}
