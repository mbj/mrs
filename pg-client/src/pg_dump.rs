use std::borrow::Cow;

use core::fmt::{Display, Formatter};
use core::str::FromStr;

use crate::identifier::{QualifiedTable, Schema};

/// Alphanumeric key for the `\restrict` / `\unrestrict` psql meta-commands
/// emitted by `pg_dump` (CVE-2025-8714).
///
/// When set, `pg_dump` uses this fixed key instead of generating a random one,
/// making plain-text dump output deterministic across invocations.
///
/// Constraints: non-empty, alphanumeric (`a-zA-Z0-9`), max 63 bytes
/// (matching the auto-generated key length).
///
/// See: <https://www.postgresql.org/docs/current/app-pgdump.html>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RestrictKey(Cow<'static, str>);

/// Maximum length of a restrict key in bytes.
const MAX_LENGTH: usize = 63;

/// Const-compatible validation that returns an optional error.
const fn validate_restrict_key(input: &str) -> Option<RestrictKeyParseError> {
    if input.is_empty() {
        return Some(RestrictKeyParseError::Empty);
    }

    if input.len() > MAX_LENGTH {
        return Some(RestrictKeyParseError::TooLong);
    }

    let bytes = input.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        if !bytes[index].is_ascii_alphanumeric() {
            return Some(RestrictKeyParseError::NotAlphanumeric);
        }
        index += 1;
    }

    None
}

impl RestrictKey {
    /// Creates a new restrict key from a static string.
    ///
    /// # Panics
    ///
    /// Panics if the input is empty, exceeds 63 bytes, or contains non-alphanumeric bytes.
    #[must_use]
    pub const fn from_static_or_panic(input: &'static str) -> Self {
        match validate_restrict_key(input) {
            Some(error) => panic!("{}", error.message()),
            None => Self(Cow::Borrowed(input)),
        }
    }

    /// Returns the value as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for RestrictKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for RestrictKey {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for RestrictKey {
    type Err = RestrictKeyParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match validate_restrict_key(input) {
            Some(error) => Err(error),
            None => Ok(Self(Cow::Owned(input.to_owned()))),
        }
    }
}

/// Error parsing a restrict key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestrictKeyParseError {
    /// Key cannot be empty.
    Empty,
    /// Key exceeds maximum length.
    TooLong,
    /// Key contains non-alphanumeric bytes.
    NotAlphanumeric,
}

impl RestrictKeyParseError {
    /// Returns the error message.
    #[must_use]
    pub const fn message(&self) -> &'static str {
        match self {
            Self::Empty => "restrict key cannot be empty",
            Self::TooLong => "restrict key exceeds 63 byte max length",
            Self::NotAlphanumeric => "restrict key must be alphanumeric",
        }
    }
}

impl Display for RestrictKeyParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(self.message())
    }
}

impl std::error::Error for RestrictKeyParseError {}

#[must_use]
pub struct PgSchemaDump {
    exclude_schemas: Vec<Schema>,
    exclude_tables: Vec<QualifiedTable>,
    no_comments: bool,
    no_owner: bool,
    no_privileges: bool,
    no_tablespaces: bool,
    restrict_key: Option<RestrictKey>,
    schemas: Vec<Schema>,
    tables: Vec<QualifiedTable>,
    verbose: bool,
}

impl Default for PgSchemaDump {
    fn default() -> Self {
        Self::new()
    }
}

impl PgSchemaDump {
    pub fn new() -> Self {
        Self {
            exclude_schemas: Vec::new(),
            exclude_tables: Vec::new(),
            no_comments: false,
            no_owner: false,
            no_privileges: false,
            no_tablespaces: false,
            restrict_key: None,
            schemas: Vec::new(),
            tables: Vec::new(),
            verbose: false,
        }
    }

    pub fn exclude_schema(mut self, schema: Schema) -> Self {
        self.exclude_schemas.push(schema);
        self
    }

    pub fn exclude_table(mut self, table: QualifiedTable) -> Self {
        self.exclude_tables.push(table);
        self
    }

    pub fn no_comments(mut self) -> Self {
        self.no_comments = true;
        self
    }

    pub fn no_owner(mut self) -> Self {
        self.no_owner = true;
        self
    }

    pub fn no_privileges(mut self) -> Self {
        self.no_privileges = true;
        self
    }

    pub fn no_tablespaces(mut self) -> Self {
        self.no_tablespaces = true;
        self
    }

    pub fn restrict_key(mut self, restrict_key: RestrictKey) -> Self {
        self.restrict_key = Some(restrict_key);
        self
    }

    pub fn schema(mut self, schema: Schema) -> Self {
        self.schemas.push(schema);
        self
    }

    pub fn table(mut self, table: QualifiedTable) -> Self {
        self.tables.push(table);
        self
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    #[must_use]
    pub fn arguments(&self) -> Vec<String> {
        let mut args = vec!["--schema-only".to_string()];

        for schema in &self.exclude_schemas {
            args.push("--exclude-schema".to_string());
            args.push(schema.to_string());
        }

        for table in &self.exclude_tables {
            args.push("--exclude-table".to_string());
            args.push(table.to_string());
        }

        if self.no_comments {
            args.push("--no-comments".to_string());
        }

        if self.no_owner {
            args.push("--no-owner".to_string());
        }

        if self.no_privileges {
            args.push("--no-privileges".to_string());
        }

        if self.no_tablespaces {
            args.push("--no-tablespaces".to_string());
        }

        if let Some(restrict_key) = &self.restrict_key {
            args.push(format!("--restrict-key={restrict_key}"));
        }

        for schema in &self.schemas {
            args.push("--schema".to_string());
            args.push(schema.to_string());
        }

        for table in &self.tables {
            args.push("--table".to_string());
            args.push(table.to_string());
        }

        if self.verbose {
            args.push("--verbose".to_string());
        }

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(PgSchemaDump::new().arguments(), vec!["--schema-only"]);
    }

    #[test]
    fn test_verbose() {
        assert_eq!(
            PgSchemaDump::new().verbose().arguments(),
            vec!["--schema-only", "--verbose"],
        );
    }

    #[test]
    fn test_exclude_schema() {
        assert_eq!(
            PgSchemaDump::new()
                .exclude_schema("internal".parse().unwrap())
                .arguments(),
            vec!["--schema-only", "--exclude-schema", "internal"],
        );
    }

    #[test]
    fn test_exclude_table() {
        assert_eq!(
            PgSchemaDump::new()
                .exclude_table(QualifiedTable {
                    schema: Schema::PUBLIC,
                    table: "some_table".parse().unwrap(),
                })
                .arguments(),
            vec!["--schema-only", "--exclude-table", "public.some_table"],
        );
    }

    #[test]
    fn test_no_comments() {
        assert_eq!(
            PgSchemaDump::new().no_comments().arguments(),
            vec!["--schema-only", "--no-comments"],
        );
    }

    #[test]
    fn test_no_owner() {
        assert_eq!(
            PgSchemaDump::new().no_owner().arguments(),
            vec!["--schema-only", "--no-owner"],
        );
    }

    #[test]
    fn test_no_privileges() {
        assert_eq!(
            PgSchemaDump::new().no_privileges().arguments(),
            vec!["--schema-only", "--no-privileges"],
        );
    }

    #[test]
    fn test_no_tablespaces() {
        assert_eq!(
            PgSchemaDump::new().no_tablespaces().arguments(),
            vec!["--schema-only", "--no-tablespaces"],
        );
    }

    #[test]
    fn test_schema() {
        assert_eq!(
            PgSchemaDump::new().schema(Schema::PUBLIC).arguments(),
            vec!["--schema-only", "--schema", "public"],
        );
    }

    #[test]
    fn test_table() {
        assert_eq!(
            PgSchemaDump::new()
                .table(QualifiedTable {
                    schema: Schema::PUBLIC,
                    table: "users".parse().unwrap(),
                })
                .arguments(),
            vec!["--schema-only", "--table", "public.users"],
        );
    }

    #[test]
    fn test_restrict_key() {
        assert_eq!(
            PgSchemaDump::new()
                .restrict_key("abc123".parse().unwrap())
                .arguments(),
            vec!["--schema-only", "--restrict-key=abc123"],
        );
    }

    #[test]
    fn test_restrict_key_empty() {
        assert_eq!("".parse::<RestrictKey>(), Err(RestrictKeyParseError::Empty),);
    }

    #[test]
    fn test_restrict_key_too_long() {
        let key: String = std::iter::repeat_n('a', 64).collect();

        assert_eq!(
            key.parse::<RestrictKey>(),
            Err(RestrictKeyParseError::TooLong),
        );
    }

    #[test]
    fn test_restrict_key_max_length() {
        let key: String = std::iter::repeat_n('a', 63).collect();

        assert!(key.parse::<RestrictKey>().is_ok());
    }

    #[test]
    fn test_restrict_key_non_alphanumeric() {
        assert_eq!(
            "abc-123".parse::<RestrictKey>(),
            Err(RestrictKeyParseError::NotAlphanumeric),
        );
    }

    #[test]
    fn test_restrict_key_const() {
        const KEY: RestrictKey = RestrictKey::from_static_or_panic("abc123");
        assert_eq!(KEY.as_str(), "abc123");
    }

    #[test]
    fn test_combined() {
        assert_eq!(
            PgSchemaDump::new()
                .exclude_schema("internal".parse().unwrap())
                .exclude_table(QualifiedTable {
                    schema: Schema::PUBLIC,
                    table: "temp".parse().unwrap(),
                })
                .no_owner()
                .no_privileges()
                .verbose()
                .arguments(),
            vec![
                "--schema-only",
                "--exclude-schema",
                "internal",
                "--exclude-table",
                "public.temp",
                "--no-owner",
                "--no-privileges",
                "--verbose",
            ],
        );
    }
}
