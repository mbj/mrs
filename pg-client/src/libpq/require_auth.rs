// https://github.com/postgres/postgres/blob/REL_18_STABLE/src/interfaces/libpq/fe-connect.c (require_auth)

use std::collections::BTreeSet;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, strum::IntoStaticStr, strum::EnumString,
)]
pub enum Method {
    #[strum(serialize = "password")]
    Password,
    #[strum(serialize = "md5")]
    Md5,
    #[strum(serialize = "gss")]
    Gss,
    #[strum(serialize = "sspi")]
    Sspi,
    #[strum(serialize = "scram-sha-256")]
    ScramSha256,
    #[strum(serialize = "oauth")]
    OAuth,
    #[strum(serialize = "none")]
    None,
}

impl Method {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Methods(BTreeSet<Method>);

impl Methods {
    #[must_use]
    pub fn new(methods: impl IntoIterator<Item = Method>) -> Option<Self> {
        let methods: BTreeSet<Method> = methods.into_iter().collect();

        (!methods.is_empty()).then_some(Self(methods))
    }

    #[must_use]
    pub fn contains(&self, method: Method) -> bool {
        self.0.contains(&method)
    }

    pub fn iter(&self) -> impl Iterator<Item = Method> + '_ {
        self.0.iter().copied()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Requirement {
    Allow(Methods),
    Deny(Methods),
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    #[error("require_auth names no method")]
    Empty,
    #[error("require_auth method {value:?} is not one libpq knows")]
    UnknownMethod { value: String },
    #[error("require_auth mixes negated and non-negated methods")]
    Mixed,
    #[error("require_auth names {} more than once", method.as_str())]
    Duplicate { method: Method },
}

impl Requirement {
    pub(crate) fn pg_env_value(&self) -> String {
        let (methods, prefix) = match self {
            Self::Allow(methods) => (methods, ""),
            Self::Deny(methods) => (methods, "!"),
        };

        methods
            .iter()
            .map(|method| format!("{prefix}{}", method.as_str()))
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl std::str::FromStr for Requirement {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, ParseError> {
        if value.is_empty() {
            return Err(ParseError::Empty);
        }

        let mut negated = None;
        let mut methods = BTreeSet::new();

        for part in value.split(',') {
            let (is_negated, name) = match part.strip_prefix('!') {
                Some(name) => (true, name),
                None => (false, part),
            };

            if *negated.get_or_insert(is_negated) != is_negated {
                return Err(ParseError::Mixed);
            }

            let method: Method = name.parse().map_err(|_| ParseError::UnknownMethod {
                value: name.to_owned(),
            })?;

            if !methods.insert(method) {
                return Err(ParseError::Duplicate { method });
            }
        }

        let methods = Methods::new(methods).ok_or(ParseError::Empty)?;

        Ok(if negated == Some(true) {
            Self::Deny(methods)
        } else {
            Self::Allow(methods)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    fn methods(methods: &[Method]) -> Methods {
        Methods::new(methods.iter().copied()).unwrap()
    }

    #[test]
    fn parses_an_allow_list_and_renders_it_back() {
        let requirement: Requirement = "scram-sha-256,none,password".parse().unwrap();

        assert_eq!(
            requirement,
            Requirement::Allow(methods(&[
                Method::Password,
                Method::ScramSha256,
                Method::None
            ]))
        );
        assert_eq!(requirement.pg_env_value(), "password,scram-sha-256,none");
    }

    #[test]
    fn parses_a_deny_list_and_renders_it_back() {
        let requirement: Requirement = "!md5,!password".parse().unwrap();

        assert_eq!(
            requirement,
            Requirement::Deny(methods(&[Method::Password, Method::Md5]))
        );
        assert_eq!(requirement.pg_env_value(), "!password,!md5");
    }

    #[test]
    fn refuses_what_libpq_refuses() {
        for (value, error) in [
            ("", ParseError::Empty),
            (
                "kerberos",
                ParseError::UnknownMethod {
                    value: "kerberos".to_owned(),
                },
            ),
            ("password,!md5", ParseError::Mixed),
            ("!md5,password", ParseError::Mixed),
            (
                "password,password",
                ParseError::Duplicate {
                    method: Method::Password,
                },
            ),
        ] {
            assert_eq!(value.parse::<Requirement>(), Err(error), "{value:?}");
        }
    }
}
