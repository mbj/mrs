//! Validated PostgreSQL configuration parameter names and values.
//!
//! Recommended import style:
//!
//! ```
//! use pg_client::parameter;
//!
//! let name = parameter::Name::from_static_or_panic("synchronous_commit");
//! let value = parameter::Value::from_static_or_panic("off");
//! ```
//!
//! These types model PG's GUC (Grand Unified Configuration) parameters in
//! every context where they apply: `postgresql.conf`, `-c` command-line flags,
//! `ALTER SYSTEM SET`, `ALTER DATABASE … SET`, `ALTER ROLE … SET`, `SET`,
//! `SET LOCAL`, and `set_config(...)`. The `pg_settings.context` column on the
//! PG side determines which of those targets a given parameter can be set
//! through, but the name/value validity rules themselves are uniform.
//!
//! See [PG 18: Setting Parameters](https://www.postgresql.org/docs/current/config-setting.html)
//! for upstream documentation. PG itself does not impose a documented byte
//! length on string parameter values; the cap here is a sanity-check against
//! programmer error, not a platform constraint.

use std::borrow::Cow;

/// Validated PostgreSQL configuration parameter name.
///
/// Parameter names follow the GUC identifier rules:
/// - Non-empty.
/// - First byte: ASCII letter or underscore.
/// - Subsequent bytes: ASCII alphanumeric, underscore, or dot.
///
/// The dot enables namespaced extension parameters such as
/// `pg_stat_statements.track`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct Name(Cow<'static, str>);

impl From<Name> for String {
    fn from(name: Name) -> Self {
        name.0.into()
    }
}

impl Name {
    /// Returns the parameter name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validated parameter name for `'static` inputs.
    ///
    /// # Panics
    ///
    /// Panics at compile time when used in a `const` context, or at runtime
    /// otherwise, if the name is empty, starts with an invalid character, or
    /// contains any character outside the allowed set.
    #[must_use]
    pub const fn from_static_or_panic(name: &'static str) -> Self {
        match validate_name(name) {
            Ok(()) => {}
            Err(NameError::Empty) => {
                panic!("PostgreSQL parameter name cannot be empty");
            }
            Err(NameError::InvalidStartCharacter) => {
                panic!("PostgreSQL parameter name must start with a letter or underscore");
            }
            Err(NameError::InvalidCharacter) => {
                panic!("PostgreSQL parameter name contains an invalid character");
            }
        }
        Self(Cow::Borrowed(name))
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NameError {
    #[error("PostgreSQL parameter name cannot be empty")]
    Empty,
    #[error("PostgreSQL parameter name must start with a letter or underscore")]
    InvalidStartCharacter,
    #[error("PostgreSQL parameter name contains an invalid character")]
    InvalidCharacter,
}

impl std::str::FromStr for Name {
    type Err = NameError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        validate_name(name).map(|()| Self(Cow::Owned(name.to_string())))
    }
}

impl TryFrom<String> for Name {
    type Error = NameError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        validate_name(&name).map(|()| Self(Cow::Owned(name)))
    }
}

impl From<&'static str> for Name {
    fn from(name: &'static str) -> Self {
        Self::from_static_or_panic(name)
    }
}

const fn validate_name(name: &str) -> Result<(), NameError> {
    let bytes = name.as_bytes();
    if bytes.is_empty() {
        return Err(NameError::Empty);
    }
    let first = bytes[0];
    if !(first.is_ascii_alphabetic() || first == b'_') {
        return Err(NameError::InvalidStartCharacter);
    }
    let mut index = 1;
    while index < bytes.len() {
        let byte = bytes[index];
        if !(byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'.') {
            return Err(NameError::InvalidCharacter);
        }
        index += 1;
    }
    Ok(())
}

/// Maximum byte length of a validated PostgreSQL parameter value.
///
/// PG itself does not impose a documented length cap; this is a sanity-check
/// safely above any realistic value (`shared_preload_libraries` with a long
/// extension list is well under 1 KB) and well below kernel argv per-string
/// limits.
///
/// Keep the panic message in [`Value::from_static_or_panic`] in sync if this
/// changes.
pub const VALUE_MAX_LEN: usize = 4096;

/// Validated PostgreSQL configuration parameter value.
///
/// Ensures the value contains no NUL bytes and does not exceed
/// [`VALUE_MAX_LEN`] bytes.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct Value(Cow<'static, str>);

impl From<Value> for String {
    fn from(value: Value) -> Self {
        value.0.into()
    }
}

impl Value {
    /// Returns the parameter value as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Validated parameter value for `'static` inputs.
    ///
    /// # Panics
    ///
    /// Panics at compile time when used in a `const` context, or at runtime
    /// otherwise, if the value contains a NUL byte or exceeds
    /// [`VALUE_MAX_LEN`] bytes.
    #[must_use]
    pub const fn from_static_or_panic(value: &'static str) -> Self {
        match validate_value(value) {
            Ok(()) => {}
            Err(ValueError::ContainsNul { .. }) => {
                panic!("PostgreSQL parameter value cannot contain NUL byte");
            }
            Err(ValueError::TooLong { .. }) => {
                panic!("PostgreSQL parameter value exceeds maximum of 4096 bytes");
            }
        }
        Self(Cow::Borrowed(value))
    }
}

impl AsRef<str> for Value {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ValueError {
    #[error("PostgreSQL parameter value contains NUL byte at index {index}")]
    ContainsNul { index: usize },
    #[error("PostgreSQL parameter value length {length} exceeds maximum {max}")]
    TooLong { length: usize, max: usize },
}

impl std::str::FromStr for Value {
    type Err = ValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        validate_value(value).map(|()| Self(Cow::Owned(value.to_string())))
    }
}

impl TryFrom<String> for Value {
    type Error = ValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_value(&value).map(|()| Self(Cow::Owned(value)))
    }
}

impl From<&'static str> for Value {
    fn from(value: &'static str) -> Self {
        Self::from_static_or_panic(value)
    }
}

const fn validate_value(value: &str) -> Result<(), ValueError> {
    let bytes = value.as_bytes();
    if bytes.len() > VALUE_MAX_LEN {
        return Err(ValueError::TooLong {
            length: bytes.len(),
            max: VALUE_MAX_LEN,
        });
    }
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == 0 {
            return Err(ValueError::ContainsNul { index });
        }
        index += 1;
    }
    Ok(())
}

/// Map of PostgreSQL configuration parameters.
///
/// `BTreeMap` for stable iteration order and dedup-on-key semantics — same
/// shape as `ociman::container::EnvironmentVariables`.
pub type Map = std::collections::BTreeMap<Name, Value>;
