//! PostgreSQL identifier types.
//!
//! This module provides types for PostgreSQL identifier values (table names, schema names, etc.).
//!
//! **Important:** These types represent identifier *values*, not SQL syntax. They do not parse
//! or produce quoted identifier syntax. For example, a table named `my table` (with a space)
//! is represented as the string `my table`, not as `"my table"`.
//!
//! Validation rules:
//! - Cannot be empty
//! - Maximum length of 63 bytes (NAMEDATALEN - 1)
//! - Cannot contain NUL bytes

use core::fmt::{Display, Formatter};
use core::str::FromStr;

/// Maximum length of a PostgreSQL identifier in bytes.
pub const MAX_LENGTH: usize = 63;

/// Const-compatible validation that returns an optional error.
const fn validate(input: &str) -> Option<ParseError> {
    if input.is_empty() {
        return Some(ParseError::Empty);
    }

    if input.len() > MAX_LENGTH {
        return Some(ParseError::TooLong);
    }

    let bytes = input.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == 0 {
            return Some(ParseError::ContainsNul);
        }
        index += 1;
    }

    None
}

/// A validated PostgreSQL identifier value.
///
/// This represents the actual identifier value, not SQL syntax. Identifiers can contain
/// spaces and special characters (which would require quoting in SQL).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Identifier<'a>(crate::protocol::String<'a>);

impl serde::Serialize for Identifier<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl TryFrom<String> for Identifier<'_> {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validated(value)
    }
}

impl Identifier<'static> {
    /// Creates a new identifier from a static string.
    ///
    /// # Panics
    ///
    /// Panics if the input is empty, exceeds [`MAX_LENGTH`], or contains NUL bytes.
    #[must_use]
    const fn from_static_or_panic(input: &'static str) -> Self {
        match validate(input) {
            Some(error) => panic!("{}", error.message()),
            None => Self(crate::protocol::String::from_static_or_panic(input)),
        }
    }
}

impl<'a> Identifier<'a> {
    fn validated(input: impl Into<std::borrow::Cow<'a, str>>) -> Result<Self, ParseError> {
        let input = input.into();

        if let Some(error) = validate(&input) {
            return Err(error);
        }

        crate::protocol::String::new(input)
            .map(Self)
            .map_err(|crate::protocol::StringError::ContainsNul { .. }| ParseError::ContainsNul)
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    const fn as_protocol_string(&self) -> &crate::protocol::String<'a> {
        &self.0
    }
}

impl Display for Identifier<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl AsRef<str> for Identifier<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for Identifier<'_> {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::validated(input.to_owned())
    }
}

/// Error parsing a PostgreSQL identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    /// Identifier cannot be empty.
    Empty,

    /// Identifier exceeds maximum length.
    TooLong,

    /// Identifier contains a NUL byte.
    ContainsNul,
}

impl ParseError {
    /// Returns the error message.
    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self {
            Self::Empty => "identifier cannot be empty",
            Self::TooLong => "identifier exceeds maximum length",
            Self::ContainsNul => "identifier cannot contain NUL bytes",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{}", self.message())
    }
}

impl std::error::Error for ParseError {}

/// The bytes `pg_clean_ascii` leaves alone; a server replaces every other byte
/// with `\xNN`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
struct CleanAscii(String);

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum CleanAsciiError {
    #[error("value is empty")]
    Empty,
    #[error("value is {length} bytes, more than {max}")]
    TooLong { length: usize, max: usize },
    #[error("byte {byte:#04x} at {index} is not printable ASCII")]
    NotPrintable { index: usize, byte: u8 },
}

impl CleanAscii {
    fn as_str(&self) -> &str {
        &self.0
    }

    fn reported(value: &str) -> Result<Self, CleanAsciiError> {
        clean_ascii(value, MAX_LENGTH * 4).map(|()| Self(value.to_owned()))
    }
}

impl FromStr for CleanAscii {
    type Err = CleanAsciiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        clean_ascii(value, MAX_LENGTH).map(|()| Self(value.to_owned()))
    }
}

fn clean_ascii(value: &str, max: usize) -> Result<(), CleanAsciiError> {
    if value.is_empty() {
        return Err(CleanAsciiError::Empty);
    }

    if value.len() > max {
        return Err(CleanAsciiError::TooLong {
            length: value.len(),
            max,
        });
    }

    match value
        .bytes()
        .enumerate()
        .find(|(_, byte)| !(b' '..=b'~').contains(byte))
    {
        Some((index, byte)) => Err(CleanAsciiError::NotPrintable { index, byte }),
        None => Ok(()),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApplicationName(crate::parameter::Value);

impl ApplicationName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[must_use]
    pub fn into_value(self) -> crate::parameter::Value {
        self.0
    }
}

impl FromStr for ApplicationName {
    type Err = CleanAsciiError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        clean_ascii(value, MAX_LENGTH)?;

        crate::parameter::Value::try_from(value.to_owned())
            .map(Self)
            .map_err(|error| match error {
                crate::parameter::ValueError::ContainsNul { index } => {
                    CleanAsciiError::NotPrintable { index, byte: 0 }
                }
                crate::parameter::ValueError::TooLong { length, max } => {
                    CleanAsciiError::TooLong { length, max }
                }
            })
    }
}

/// Not necessarily one any client sent: SQL, role and database defaults, or a
/// pooler can set it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EffectiveApplicationName(CleanAscii);

impl EffectiveApplicationName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub(crate) fn reported(value: &str) -> Result<Self, CleanAsciiError> {
        CleanAscii::reported(value).map(Self)
    }
}

/// Macro to define identifier-backed newtypes.
macro_rules! define_identifier_type {
    ($(#[$meta:meta])* $name:ident, $test_mod:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
        #[serde(try_from = "String")]
        pub struct $name<'a>(Identifier<'a>);

        impl TryFrom<String> for $name<'_> {
            type Error = ParseError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Identifier::try_from(value).map(Self)
            }
        }

        impl $name<'static> {
            /// Creates a new value from a static string.
            ///
            /// # Panics
            ///
            /// Panics if the input is empty, exceeds [`MAX_LENGTH`], or contains NUL bytes.
            #[must_use]
            pub const fn from_static_or_panic(input: &'static str) -> Self {
                Self(Identifier::from_static_or_panic(input))
            }
        }

        impl<'a> $name<'a> {
            pub fn new(input: &'a str) -> Result<Self, ParseError> {
                Identifier::validated(input).map(Self)
            }

            /// Returns the value as a string slice.
            #[must_use]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            #[must_use]
            pub const fn as_protocol_string(&self) -> &crate::protocol::String<'a> {
                self.0.as_protocol_string()
            }
        }

        impl Display for $name<'_> {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "{}", self.0)
            }
        }

        impl AsRef<str> for $name<'_> {
            fn as_ref(&self) -> &str {
                self.0.as_ref()
            }
        }

        impl FromStr for $name<'_> {
            type Err = ParseError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Identifier::from_str(input).map(Self)
            }
        }

        impl crate::value::Decode<'_> for $name<'static> {
            fn accepts(column: &crate::protocol::backend::ColumnDescription<'_>) -> bool {
                crate::value::accepts_text(column)
            }

            fn decode(value: &[u8]) -> Result<Self, crate::value::DecodeError> {
                Self::try_from(String::decode(value)?).map_err(crate::value::DecodeError::InvalidName)
            }
        }

        #[cfg(test)]
        mod $test_mod {
            use super::*;

            #[test]
            fn parse_valid() {
                let value: $name = "test".parse().unwrap();
                assert_eq!(value.to_string(), "test");
            }

            #[test]
            fn parse_valid_with_space() {
                let value: $name = "test value".parse().unwrap();
                assert_eq!(value.to_string(), "test value");
            }

            #[test]
            fn parse_empty_fails() {
                let result: Result<$name, _> = "".parse();
                assert!(matches!(result, Err(ParseError::Empty)));
            }

            #[test]
            fn parse_contains_nul_fails() {
                let result: Result<$name, _> = "test\0value".parse();
                assert!(matches!(result, Err(ParseError::ContainsNul)));
            }

            #[test]
            fn parse_too_long_fails() {
                let input = "a".repeat(MAX_LENGTH + 1);
                let result: Result<$name, _> = input.parse();
                assert!(matches!(result, Err(ParseError::TooLong)));
            }
        }
    };
}

define_identifier_type!(
    /// A PostgreSQL table name.
    Table,
    table
);

define_identifier_type!(
    /// A PostgreSQL schema name.
    Schema,
    schema
);

define_identifier_type!(Channel, channel);

define_identifier_type!(Statement, statement);

define_identifier_type!(Portal, portal);

impl Schema<'static> {
    /// The default `public` schema.
    pub const PUBLIC: Self = Self::from_static_or_panic("public");
}

/// A schema-qualified PostgreSQL table name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct QualifiedTable {
    /// The schema name.
    pub schema: Schema<'static>,
    /// The table name.
    pub table: Table<'static>,
}

impl Display for QualifiedTable {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{}.{}", self.schema, self.table)
    }
}

define_identifier_type!(
    /// A PostgreSQL column name.
    Column,
    column
);

define_identifier_type!(
    /// A PostgreSQL index name.
    Index,
    index
);

define_identifier_type!(
    /// A PostgreSQL constraint name.
    ///
    /// Includes PRIMARY KEY, FOREIGN KEY, CHECK, UNIQUE, and EXCLUSION constraints.
    Constraint,
    constraint
);

define_identifier_type!(
    /// A PostgreSQL extension name.
    Extension,
    extension
);

define_identifier_type!(
    /// A PostgreSQL sequence name.
    Sequence,
    sequence
);

define_identifier_type!(
    /// A PostgreSQL function or procedure name.
    Function,
    function
);

define_identifier_type!(
    /// A PostgreSQL trigger name.
    Trigger,
    trigger
);

define_identifier_type!(
    /// A PostgreSQL domain name.
    Domain,
    domain
);

define_identifier_type!(
    /// A PostgreSQL type name.
    ///
    /// Includes custom types, enums, and composite types.
    Type,
    r#type
);

define_identifier_type!(
    /// A PostgreSQL view name.
    View,
    view
);

define_identifier_type!(
    /// A PostgreSQL relation name.
    ///
    /// A relation is either a table or a view. Use this type when an operation
    /// accepts both tables and views (e.g., SELECT queries).
    Relation,
    relation
);

impl<'a> From<Table<'a>> for Relation<'a> {
    fn from(table: Table<'a>) -> Self {
        Self(table.0)
    }
}

impl<'a> From<View<'a>> for Relation<'a> {
    fn from(view: View<'a>) -> Self {
        Self(view.0)
    }
}

define_identifier_type!(
    /// A PostgreSQL materialized view name.
    MaterializedView,
    materialized_view
);

impl<'a> From<MaterializedView<'a>> for Relation<'a> {
    fn from(materialized_view: MaterializedView<'a>) -> Self {
        Self(materialized_view.0)
    }
}

define_identifier_type!(
    /// A PostgreSQL operator name.
    Operator,
    operator
);

define_identifier_type!(
    /// A PostgreSQL aggregate function name.
    Aggregate,
    aggregate
);

define_identifier_type!(
    /// A PostgreSQL collation name.
    Collation,
    collation
);

define_identifier_type!(
    /// A PostgreSQL tablespace name.
    Tablespace,
    tablespace
);

define_identifier_type!(
    /// A PostgreSQL row-level security policy name.
    Policy,
    policy
);

define_identifier_type!(
    /// A PostgreSQL rule name.
    Rule,
    rule
);

define_identifier_type!(
    /// A PostgreSQL publication name (for logical replication).
    Publication,
    publication
);

define_identifier_type!(
    /// A PostgreSQL subscription name (for logical replication).
    Subscription,
    subscription
);

define_identifier_type!(
    /// A PostgreSQL foreign server name.
    ForeignServer,
    foreign_server
);

define_identifier_type!(
    /// A PostgreSQL foreign data wrapper name.
    ForeignDataWrapper,
    foreign_data_wrapper
);

define_identifier_type!(
    /// A PostgreSQL foreign table name.
    ForeignTable,
    foreign_table
);

define_identifier_type!(
    /// A PostgreSQL event trigger name.
    EventTrigger,
    event_trigger
);

define_identifier_type!(
    /// A PostgreSQL procedural language name.
    Language,
    language
);

define_identifier_type!(
    /// A PostgreSQL text search configuration name.
    TextSearchConfiguration,
    text_search_configuration
);

define_identifier_type!(
    /// A PostgreSQL text search dictionary name.
    TextSearchDictionary,
    text_search_dictionary
);

define_identifier_type!(
    /// A PostgreSQL encoding conversion name.
    Conversion,
    conversion
);

define_identifier_type!(
    /// A PostgreSQL operator class name.
    OperatorClass,
    operator_class
);

define_identifier_type!(
    /// A PostgreSQL operator family name.
    OperatorFamily,
    operator_family
);

define_identifier_type!(
    /// A PostgreSQL access method name.
    AccessMethod,
    access_method
);

define_identifier_type!(
    /// A PostgreSQL extended statistics object name.
    StatisticsObject,
    statistics_object
);

define_identifier_type!(
    /// A PostgreSQL database name.
    Database,
    database
);

impl Database<'static> {
    /// The default `postgres` database.
    pub const POSTGRES: Self = Self::from_static_or_panic("postgres");
}

define_identifier_type!(
    /// A PostgreSQL role name.
    ///
    /// Roles with the `LOGIN` attribute are typically called users.
    Role,
    role
);

impl Role<'static> {
    /// The default `postgres` superuser role.
    pub const POSTGRES: Self = Self::from_static_or_panic("postgres");
}

/// A PostgreSQL user (alias for [`Role`]).
///
/// A user is a role with the `LOGIN` attribute.
pub type User<'a> = Role<'a>;

#[cfg(test)]
mod tests {
    use super::*;

    mod identifier {
        use super::*;

        #[test]
        fn parse_valid_simple() {
            let identifier: Identifier = "users".parse().unwrap();
            assert_eq!(identifier.to_string(), "users");
        }

        #[test]
        fn parse_valid_with_space() {
            let identifier: Identifier = "my table".parse().unwrap();
            assert_eq!(identifier.to_string(), "my table");
        }

        #[test]
        fn parse_valid_with_special_chars() {
            let identifier: Identifier = "my-table.name".parse().unwrap();
            assert_eq!(identifier.to_string(), "my-table.name");
        }

        #[test]
        fn parse_valid_starting_with_digit() {
            let identifier: Identifier = "1table".parse().unwrap();
            assert_eq!(identifier.to_string(), "1table");
        }

        #[test]
        fn parse_valid_max_length() {
            let input = "a".repeat(MAX_LENGTH);
            let identifier: Identifier = input.parse().unwrap();
            assert_eq!(identifier.to_string(), input);
        }

        #[test]
        fn parse_empty_fails() {
            let result: Result<Identifier, _> = "".parse();
            assert_eq!(result, Err(ParseError::Empty));
        }

        #[test]
        fn parse_too_long_fails() {
            let input = "a".repeat(MAX_LENGTH + 1);
            let result: Result<Identifier, _> = input.parse();
            assert_eq!(result, Err(ParseError::TooLong));
        }

        #[test]
        fn parse_contains_nul_fails() {
            let result: Result<Identifier, _> = "my\0table".parse();
            assert_eq!(result, Err(ParseError::ContainsNul));
        }
    }

    mod clean_ascii {
        use super::*;

        #[test]
        fn accepts_both_ends_of_printable_ascii() {
            let value: CleanAscii = " ~".parse().unwrap();
            assert_eq!(value.as_str(), " ~");
        }

        #[test]
        fn refuses_the_bytes_just_outside_printable_ascii() {
            let result: Result<CleanAscii, _> = "a\u{1f}".parse();
            assert_eq!(
                result,
                Err(CleanAsciiError::NotPrintable {
                    index: 1,
                    byte: 0x1f
                })
            );

            let result: Result<CleanAscii, _> = "a\u{7f}".parse();
            assert_eq!(
                result,
                Err(CleanAsciiError::NotPrintable {
                    index: 1,
                    byte: 0x7f
                })
            );
        }

        #[test]
        fn refuses_non_ascii_rather_than_have_it_escaped() {
            let result: Result<CleanAscii, _> = "café".parse();
            assert_eq!(
                result,
                Err(CleanAsciiError::NotPrintable {
                    index: 3,
                    byte: 0xc3
                })
            );
        }

        #[test]
        fn refuses_more_than_a_name_holds() {
            let fits = "a".repeat(MAX_LENGTH);
            let value: CleanAscii = fits.parse().unwrap();
            assert_eq!(value.as_str(), fits);

            let result: Result<CleanAscii, _> = "a".repeat(MAX_LENGTH + 1).parse();
            assert_eq!(
                result,
                Err(CleanAsciiError::TooLong {
                    length: MAX_LENGTH + 1,
                    max: MAX_LENGTH
                })
            );
        }

        #[test]
        fn refuses_empty() {
            let result: Result<CleanAscii, _> = "".parse();
            assert_eq!(result, Err(CleanAsciiError::Empty));
        }

        /// What a server reported for an `application_name` of 40 `é`: clipped
        /// first, then escaped.
        #[test]
        fn reads_a_report_longer_than_can_be_sent() {
            let reported = r"\xc3\xa9".repeat(31);
            assert_eq!(reported.len(), 248);

            let value = CleanAscii::reported(&reported).unwrap();
            assert_eq!(value.as_str(), reported);

            let result = CleanAscii::reported(&"a".repeat(MAX_LENGTH * 4 + 1));
            assert_eq!(
                result,
                Err(CleanAsciiError::TooLong {
                    length: MAX_LENGTH * 4 + 1,
                    max: MAX_LENGTH * 4
                })
            );
        }

        #[test]
        fn refuses_a_report_that_was_not_escaped() {
            let result = CleanAscii::reported("café");
            assert_eq!(
                result,
                Err(CleanAsciiError::NotPrintable {
                    index: 3,
                    byte: 0xc3
                })
            );
        }
    }

    mod application_name {
        use super::*;

        #[test]
        fn effective_holds_what_no_client_could_send() {
            let reported = r"\xc3\xa9".repeat(31);

            assert_eq!(
                EffectiveApplicationName::reported(&reported)
                    .unwrap()
                    .as_str(),
                reported
            );
            assert_eq!(
                reported.parse::<ApplicationName>(),
                Err(CleanAsciiError::TooLong {
                    length: 248,
                    max: MAX_LENGTH
                })
            );
        }

        #[test]
        fn accepts_what_the_server_keeps() {
            let name = "~".repeat(MAX_LENGTH);

            assert_eq!(name.parse::<ApplicationName>().unwrap().as_str(), name);
        }

        #[test]
        fn refuses_what_the_server_would_rewrite() {
            assert_eq!(
                "café".parse::<ApplicationName>(),
                Err(CleanAsciiError::NotPrintable {
                    index: 3,
                    byte: 0xc3
                })
            );
            assert_eq!(
                "a".repeat(MAX_LENGTH + 1).parse::<ApplicationName>(),
                Err(CleanAsciiError::TooLong {
                    length: MAX_LENGTH + 1,
                    max: MAX_LENGTH
                })
            );
            assert_eq!("".parse::<ApplicationName>(), Err(CleanAsciiError::Empty));
        }
    }
}
