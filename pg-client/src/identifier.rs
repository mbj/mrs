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

use std::borrow::Cow;

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
struct Identifier(Cow<'static, str>);

impl Identifier {
    /// Creates a new identifier from a static string.
    ///
    /// # Panics
    ///
    /// Panics if the input is empty, exceeds [`MAX_LENGTH`], or contains NUL bytes.
    #[must_use]
    const fn from_static_or_panic(input: &'static str) -> Self {
        match validate(input) {
            Some(error) => panic!("{}", error.message()),
            None => Self(Cow::Borrowed(input)),
        }
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for Identifier {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for Identifier {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match validate(input) {
            Some(error) => Err(error),
            None => Ok(Self(Cow::Owned(input.to_owned()))),
        }
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

/// Macro to define identifier-backed newtypes.
macro_rules! define_identifier_type {
    ($(#[$meta:meta])* $name:ident, $test_mod:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
        pub struct $name(Identifier);

        impl $name {
            /// Creates a new value from a static string.
            ///
            /// # Panics
            ///
            /// Panics if the input is empty, exceeds [`MAX_LENGTH`], or contains NUL bytes.
            #[must_use]
            pub const fn from_static_or_panic(input: &'static str) -> Self {
                Self(Identifier::from_static_or_panic(input))
            }

            /// Returns the value as a string slice.
            #[must_use]
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "{}", self.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.0.as_ref()
            }
        }

        impl FromStr for $name {
            type Err = ParseError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                Identifier::from_str(input).map(Self)
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

impl Schema {
    /// The default `public` schema.
    pub const PUBLIC: Self = Self::from_static_or_panic("public");
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

impl From<Table> for Relation {
    fn from(table: Table) -> Self {
        Self(table.0)
    }
}

impl From<View> for Relation {
    fn from(view: View) -> Self {
        Self(view.0)
    }
}

define_identifier_type!(
    /// A PostgreSQL materialized view name.
    MaterializedView,
    materialized_view
);

impl From<MaterializedView> for Relation {
    fn from(materialized_view: MaterializedView) -> Self {
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
    /// A PostgreSQL database name.
    Database,
    database
);

impl Database {
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

impl Role {
    /// The default `postgres` superuser role.
    pub const POSTGRES: Self = Self::from_static_or_panic("postgres");
}

/// A PostgreSQL user (alias for [`Role`]).
///
/// A user is a role with the `LOGIN` attribute.
pub type User = Role;

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
}
