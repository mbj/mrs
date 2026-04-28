//! OCI label types for containers and images.
//!
//! Labels are key-value metadata attached to images and containers. This module
//! models the two sides of the pipeline with distinct type families:
//!
//! - **Write side (Definition / BuildDefinition)**: [`Key`], [`Value`], [`Map`].
//!   Validated and bounded. Errors from constructing them are reported via
//!   [`Error`].
//! - **Read side (inspect)**: [`ReadKey`], [`ReadValue`], [`ReadLabels`],
//!   [`ReadError`] — all parametrised by a [`Scope`] marker so container and
//!   image sides have distinct type identity while sharing their
//!   implementation. Type aliases [`ContainerKey`], [`ImageKey`], etc. provide
//!   the ergonomic names. Constructible only within this crate.
//!
//! The two families support cross-family `PartialEq`, so callers can compare a
//! validated [`Key`] against a [`ReadKey`] of any scope without explicit
//! conversions.

use std::borrow::Cow;
use std::marker::PhantomData;

use cmd_proc::Command;

use crate::Apply;

/// Maximum permitted length of a [`Key`], in bytes.
pub const MAX_KEY_LEN: usize = 256;

/// Maximum permitted length of a [`Value`], in bytes.
pub const MAX_VALUE_LEN: usize = 65_536;

pub mod scope {
    //! Zero-size markers distinguishing read-side label scopes.
    //!
    //! The [`Scope`] trait is sealed — only [`Container`] and [`Image`] implement
    //! it, and external crates cannot add new scopes.

    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub struct Container;

    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub struct Image;

    mod sealed {
        pub trait Sealed {}
        impl Sealed for super::Container {}
        impl Sealed for super::Image {}
    }

    pub trait Scope:
        sealed::Sealed + Clone + Copy + Eq + std::fmt::Debug + Ord + PartialEq + PartialOrd
    {
    }
    impl Scope for Container {}
    impl Scope for Image {}
}

pub use scope::Scope;

/// Validated OCI label key.
///
/// Enforces the constraints that `docker` and `podman` actually impose at the
/// CLI level (the `--label key=value` argument is split on the first `=`):
///
/// - non-empty
/// - no `=`
/// - no whitespace or control characters
/// - no `,`
/// - at most [`MAX_KEY_LEN`] bytes
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Key(Cow<'static, str>);

impl Key {
    /// Validated label key for `'static` inputs.
    ///
    /// # Panics
    ///
    /// Panics at compile time when used in a `const` context, or at runtime
    /// otherwise, if the input does not satisfy the key constraints.
    #[must_use]
    pub const fn from_static_or_panic(input: &'static str) -> Self {
        match validate_key(input) {
            Ok(()) => {}
            Err(Error::KeyEmpty) => panic!("Label key cannot be empty"),
            Err(Error::KeyInvalidCharacter) => {
                panic!("Label key contains '=', whitespace, control, or ','")
            }
            Err(Error::KeyTooLong) => panic!("Label key exceeds maximum length"),
            Err(_) => panic!("unreachable: validate_key only emits Key* variants"),
        }
        Self(Cow::Borrowed(input))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::str::FromStr for Key {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        validate_key(input).map(|()| Self(Cow::Owned(input.to_string())))
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Bounded label value.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Value(Cow<'static, str>);

impl Value {
    /// Validated label value for `'static` inputs.
    ///
    /// # Panics
    ///
    /// Panics at compile time when used in a `const` context, or at runtime
    /// otherwise, if the input exceeds [`MAX_VALUE_LEN`] bytes.
    #[must_use]
    pub const fn from_static_or_panic(input: &'static str) -> Self {
        match validate_value(input) {
            Ok(()) => {}
            Err(Error::ValueTooLong) => panic!("Label value exceeds maximum length"),
            Err(_) => panic!("unreachable: validate_value only emits Value* variants"),
        }
        Self(Cow::Borrowed(input))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::str::FromStr for Value {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        validate_value(input).map(|()| Self(Cow::Owned(input.to_string())))
    }
}

impl TryFrom<String> for Value {
    type Error = Error;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        validate_value(&input).map(|()| Self(Cow::Owned(input)))
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Definition-side label error covering construction of [`Key`] and [`Value`].
#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("Label key cannot be empty")]
    KeyEmpty,
    #[error("Label key may not contain '=', whitespace, control characters, or ','")]
    KeyInvalidCharacter,
    #[error("Label key exceeds maximum length of {MAX_KEY_LEN} bytes")]
    KeyTooLong,
    #[error("Label value exceeds maximum length of {MAX_VALUE_LEN} bytes")]
    ValueTooLong,
}

const fn validate_key(input: &str) -> Result<(), Error> {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return Err(Error::KeyEmpty);
    }
    if bytes.len() > MAX_KEY_LEN {
        return Err(Error::KeyTooLong);
    }

    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        if byte == b'=' || byte == b',' || byte.is_ascii_whitespace() || byte.is_ascii_control() {
            return Err(Error::KeyInvalidCharacter);
        }
        i += 1;
    }
    Ok(())
}

const fn validate_value(input: &str) -> Result<(), Error> {
    if input.len() > MAX_VALUE_LEN {
        return Err(Error::ValueTooLong);
    }
    Ok(())
}

/// Ordered, deduplicated collection of [`Key`]/[`Value`] pairs for the write path.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Map(std::collections::BTreeMap<Key, Value>);

impl Map {
    #[must_use]
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub(crate) fn insert(&mut self, key: Key, value: Value) {
        self.0.insert(key, value);
    }

    #[must_use]
    pub fn get(&self, key: &Key) -> Option<&Value> {
        self.0.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &Value)> {
        self.0.iter()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Apply for Map {
    fn apply(&self, command: Command) -> Command {
        self.0.iter().fold(command, |command, (key, value)| {
            command
                .argument("--label")
                .argument(format!("{key}={value}"))
        })
    }
}

/// Unconstrained label key as returned by inspect.
///
/// Parametrised by a [`Scope`] marker so container keys and image keys have
/// distinct type identity while sharing implementation. Only this crate can
/// construct a `ReadKey`, so values of this type only exist when produced by
/// the inspect path.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ReadKey<S: Scope>(String, PhantomData<S>);

impl<S: Scope> ReadKey<S> {
    pub(crate) fn new(raw: String) -> Self {
        Self(raw, PhantomData)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<S: Scope> std::fmt::Display for ReadKey<S> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Unconstrained label value as returned by inspect.
///
/// Parametrised by a [`Scope`] marker — see [`ReadKey`] for the rationale.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ReadValue<S: Scope>(String, PhantomData<S>);

impl<S: Scope> ReadValue<S> {
    pub(crate) fn new(raw: String) -> Self {
        Self(raw, PhantomData)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<S: Scope> std::fmt::Display for ReadValue<S> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl<S: Scope> PartialEq<ReadKey<S>> for Key {
    fn eq(&self, other: &ReadKey<S>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<S: Scope> PartialEq<Key> for ReadKey<S> {
    fn eq(&self, other: &Key) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<S: Scope> PartialEq<ReadValue<S>> for Value {
    fn eq(&self, other: &ReadValue<S>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<S: Scope> PartialEq<Value> for ReadValue<S> {
    fn eq(&self, other: &Value) -> bool {
        self.as_str() == other.as_str()
    }
}

/// Labels read from a container or image. Keys and values are preserved as-is
/// from the runtime output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadLabels<S: Scope>(std::collections::BTreeMap<ReadKey<S>, ReadValue<S>>);

// Manual `Default` because the derive generates `where S: Default`, and the
// scope marker types (`Container`, `Image`) deliberately do not implement
// `Default` — they are zero-sized type tags that should never be constructed.
impl<S: Scope> Default for ReadLabels<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Scope> ReadLabels<S> {
    #[must_use]
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub(crate) fn insert(&mut self, key: ReadKey<S>, value: ReadValue<S>) {
        self.0.insert(key, value);
    }

    /// Look up the value matching `key`.
    ///
    /// Compares via cross-family `PartialEq` — O(n) scan. Label counts are
    /// small, so this is not a performance concern.
    #[must_use]
    pub fn get(&self, key: &Key) -> Option<&ReadValue<S>> {
        self.0
            .iter()
            .find(|(read_key, _)| *read_key == key)
            .map(|(_, value)| value)
    }

    #[must_use]
    pub fn contains_key(&self, key: &Key) -> bool {
        self.0.keys().any(|read_key| read_key == key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ReadKey<S>, &ReadValue<S>)> {
        self.0.iter()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// Read-side label error. Parametrised by [`Scope`] for type identity so a
/// container-read error cannot be mixed up with an image-read error.
#[derive(Debug, thiserror::Error)]
pub enum ReadError<S: Scope> {
    #[error("inspect failed")]
    Inspect {
        #[source]
        source: crate::InspectError,
        _scope: PhantomData<S>,
    },
    #[error("inspect Config.Labels was not a JSON object")]
    NotObject(PhantomData<S>),
    #[error("inspect label {key:?} had a non-string value")]
    ValueNotString { key: String, _scope: PhantomData<S> },
}

impl<S: Scope> ReadError<S> {
    /// Construct a [`ReadError::NotObject`] without callers having to spell out
    /// the `PhantomData`.
    #[must_use]
    pub fn not_object() -> Self {
        Self::NotObject(PhantomData)
    }

    /// Construct a [`ReadError::ValueNotString`] without callers having to
    /// spell out the `PhantomData`.
    #[must_use]
    pub fn value_not_string(key: String) -> Self {
        Self::ValueNotString {
            key,
            _scope: PhantomData,
        }
    }
}

/// Decode a `docker inspect` / `podman inspect` JSON payload's
/// `[0].Config.Labels` into a [`ReadLabels`] of the requested scope.
///
/// Shared between container and image inspect paths — the JSON shape is
/// identical. Missing or null `Labels` is treated as an empty map (legitimate
/// "no labels set" state from the backend).
pub(crate) fn decode_labels<S: Scope>(
    value: &serde_json::Value,
) -> Result<ReadLabels<S>, ReadError<S>> {
    let labels_value = value
        .get(0)
        .and_then(|entry| entry.get("Config"))
        .and_then(|config| config.get("Labels"));

    let mut labels = ReadLabels::<S>::new();

    let Some(labels_value) = labels_value else {
        return Ok(labels);
    };

    if labels_value.is_null() {
        return Ok(labels);
    }

    let object = labels_value
        .as_object()
        .ok_or_else(ReadError::<S>::not_object)?;

    for (key, value) in object {
        let value_str = value
            .as_str()
            .ok_or_else(|| ReadError::<S>::value_not_string(key.clone()))?;
        labels.insert(
            ReadKey::new(key.clone()),
            ReadValue::new(value_str.to_string()),
        );
    }

    Ok(labels)
}

impl<S: Scope> From<crate::InspectError> for ReadError<S> {
    fn from(source: crate::InspectError) -> Self {
        Self::Inspect {
            source,
            _scope: PhantomData,
        }
    }
}

pub type ContainerKey = ReadKey<scope::Container>;
pub type ContainerValue = ReadValue<scope::Container>;
pub type ContainerLabels = ReadLabels<scope::Container>;
pub type ContainerError = ReadError<scope::Container>;

pub type ImageKey = ReadKey<scope::Image>;
pub type ImageValue = ReadValue<scope::Image>;
pub type ImageLabels = ReadLabels<scope::Image>;
pub type ImageError = ReadError<scope::Image>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_accepts_reverse_dns() {
        assert_eq!(
            "pg-ephemeral.session".parse::<Key>().unwrap().as_str(),
            "pg-ephemeral.session"
        );
    }

    #[test]
    fn key_accepts_legacy_and_foreign_styles() {
        assert!("maintainer".parse::<Key>().is_ok());
        assert!("com.example.MixedCase".parse::<Key>().is_ok());
        assert!("foo_bar".parse::<Key>().is_ok());
        assert!(".leading".parse::<Key>().is_ok());
        assert!("trailing-".parse::<Key>().is_ok());
    }

    #[test]
    fn key_rejects_empty() {
        assert!(matches!("".parse::<Key>(), Err(Error::KeyEmpty)));
    }

    #[test]
    fn key_rejects_equals() {
        assert!(matches!(
            "foo=bar".parse::<Key>(),
            Err(Error::KeyInvalidCharacter)
        ));
    }

    #[test]
    fn key_rejects_whitespace() {
        assert!(matches!(
            "foo bar".parse::<Key>(),
            Err(Error::KeyInvalidCharacter)
        ));
        assert!(matches!(
            "foo\tbar".parse::<Key>(),
            Err(Error::KeyInvalidCharacter)
        ));
    }

    #[test]
    fn key_rejects_comma() {
        assert!(matches!(
            "foo,bar".parse::<Key>(),
            Err(Error::KeyInvalidCharacter)
        ));
    }

    #[test]
    fn key_rejects_too_long() {
        let input = "a".repeat(MAX_KEY_LEN + 1);
        assert!(matches!(input.parse::<Key>(), Err(Error::KeyTooLong)));
    }

    #[test]
    fn key_accepts_at_max_length() {
        let input = "a".repeat(MAX_KEY_LEN);
        assert!(input.parse::<Key>().is_ok());
    }

    #[test]
    fn key_from_static_or_panic_is_const() {
        const KEY: Key = Key::from_static_or_panic("pg-ephemeral.managed");
        assert_eq!(KEY.as_str(), "pg-ephemeral.managed");
    }

    #[test]
    fn value_accepts_at_max_length() {
        let input = "x".repeat(MAX_VALUE_LEN);
        assert!(input.parse::<Value>().is_ok());
    }

    #[test]
    fn value_rejects_too_long() {
        let input = "x".repeat(MAX_VALUE_LEN + 1);
        assert!(matches!(input.parse::<Value>(), Err(Error::ValueTooLong)));
    }

    #[test]
    fn value_from_static_or_panic_is_const() {
        const VALUE: Value = Value::from_static_or_panic("main");
        assert_eq!(VALUE.as_str(), "main");
    }

    #[test]
    fn cross_family_key_equality_container() {
        let typed = Key::from_static_or_panic("pg-ephemeral.session");
        let raw: ContainerKey = ContainerKey::new("pg-ephemeral.session".into());
        assert_eq!(typed, raw);
        assert_eq!(raw, typed);
    }

    #[test]
    fn cross_family_key_equality_image() {
        let typed = Key::from_static_or_panic("pg-ephemeral.session");
        let raw: ImageKey = ImageKey::new("pg-ephemeral.session".into());
        assert_eq!(typed, raw);
        assert_eq!(raw, typed);
    }

    #[test]
    fn cross_family_value_equality() {
        let typed = Value::from_static_or_panic("main");
        let raw: ContainerValue = ContainerValue::new("main".into());
        assert_eq!(typed, raw);
        assert_eq!(raw, typed);
    }

    #[test]
    fn read_labels_get_by_typed_key() {
        let mut labels: ContainerLabels = ContainerLabels::new();
        labels.insert(
            ContainerKey::new("pg-ephemeral.session".into()),
            ContainerValue::new("main".into()),
        );

        let key = Key::from_static_or_panic("pg-ephemeral.session");
        let value = Value::from_static_or_panic("main");
        assert_eq!(labels.get(&key).unwrap(), &value);
        assert!(labels.contains_key(&key));

        let missing = Key::from_static_or_panic("other.key");
        assert!(labels.get(&missing).is_none());
    }

    #[test]
    fn map_preserves_ordering() {
        let mut map = Map::new();
        map.insert(
            Key::from_static_or_panic("b.key"),
            Value::from_static_or_panic("second"),
        );
        map.insert(
            Key::from_static_or_panic("a.key"),
            Value::from_static_or_panic("first"),
        );

        let collected: Vec<_> = map.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
        assert_eq!(collected, vec![("a.key", "first"), ("b.key", "second")]);
    }
}
