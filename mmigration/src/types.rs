use nom::{
    Finish, IResult, Parser,
    bytes::complete::take_while1,
    character::complete::char,
    combinator::{all_consuming, recognize},
    error::context,
    multi::many0_count,
    sequence::{pair, preceded},
};
use nom_language::error::VerboseError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Index(u32);

impl Index {
    /// Return successor of index
    ///
    /// # Example
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let a: Index = 0_u32.into();
    /// let b: Index = 1_u32.into();
    ///
    /// assert_eq!(a.succ(), b);
    /// ```
    #[must_use]
    pub fn succ(&self) -> Index {
        Self(self.0.checked_add(1).unwrap())
    }

    /// Test if index is initial one
    ///
    /// # Example
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let a: Index = 0_u32.into();
    /// let b: Index = 1_u32.into();
    ///
    /// assert_eq!(a.is_initial(), true);
    /// assert_eq!(b.is_initial(), false);
    /// ```
    #[must_use]
    pub fn is_initial(&self) -> bool {
        self == &Self::initial()
    }

    /// Return initial index
    ///
    /// # Example
    /// ```
    /// # use mmigration::*;
    ///
    /// let index: Index = 0.into();
    ///
    /// assert_eq!(index, Index::initial());
    /// ```
    #[must_use]
    pub fn initial() -> Index {
        Self(0)
    }

    /// Test if other is successor of self
    ///
    /// # Example
    ///
    /// ```
    /// # use mmigration::*;
    ///
    /// let a: Index = 0_u32.into();
    /// let b: Index = 1_u32.into();
    /// let c: Index = 2_u32.into();
    ///
    /// assert_eq!(a.is_succ_of(a), false);
    /// assert_eq!(a.is_succ_of(b), false);
    /// assert_eq!(b.is_succ_of(a), true);
    /// assert_eq!(c.is_succ_of(a), false);
    /// assert_eq!(c.is_succ_of(b), true);
    /// ```
    #[must_use]
    pub fn is_succ_of(&self, other: Self) -> bool {
        *self == other.succ()
    }
}

impl std::fmt::Display for Index {
    /// Format index
    ///
    /// ```
    /// # use mmigration::*;
    /// assert_eq!("0", format!("{}", Index::initial()));
    /// ```
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
    }
}

impl From<u32> for Index {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl std::str::FromStr for Index {
    type Err = std::num::ParseIntError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        <u32 as std::str::FromStr>::from_str(value).map(Self)
    }
}

impl sqlx::Decode<'_, sqlx::Postgres> for Index {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'_>,
    ) -> Result<Index, Box<dyn std::error::Error + 'static + Send + Sync>> {
        <i64 as sqlx::Decode<sqlx::Postgres>>::decode(value).and_then(|value| {
            match value.try_into() {
                Ok(valid) => Ok(Self(valid)),
                Err(_) => Err("out of u32 range".into()),
            }
        })
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for Index {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <i64 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0.into(), buf)
    }
}

impl sqlx::Type<sqlx::Postgres> for Index {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <i64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl sqlx::postgres::PgHasArrayType for Index {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <i64 as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawSql(String);

impl AsRef<[u8]> for RawSql {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<str> for RawSql {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<String> for RawSql {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for RawSql {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl sqlx::SqlSafeStr for &RawSql {
    fn into_sql_str(self) -> sqlx::SqlStr {
        sqlx::AssertSqlSafe(self.0.clone()).into_sql_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PendingMigration {
    pub index: Index,
    pub name: MigrationName,
    pub raw_sql: RawSql,
}

impl PendingMigration {
    #[must_use]
    pub fn digest(&self) -> [u8; 32] {
        <sha2::Sha256 as sha2::Digest>::digest(<RawSql as AsRef<[u8]>>::as_ref(&self.raw_sql))
            .into()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Schema(String);

impl AsRef<[u8]> for Schema {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<str> for Schema {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Schema {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for Schema {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationName(String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseMigrationNameError {
    Nom {
        report: String,
        error: VerboseError<String>,
    },
    TooLong {
        max: usize,
        actual: usize,
    },
}

impl std::fmt::Display for ParseMigrationNameError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nom { report, .. } => write!(formatter, "{report}"),
            Self::TooLong { max, actual } => write!(
                formatter,
                "migration name cannot consist of more than {max} characters (got {actual})"
            ),
        }
    }
}

impl std::error::Error for ParseMigrationNameError {}

type ParseResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;

impl AsRef<str> for MigrationName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl sqlx::Decode<'_, sqlx::Postgres> for MigrationName {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'_>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        <String as sqlx::Decode<sqlx::Postgres>>::decode(value).and_then(|value| {
            match <Self as std::str::FromStr>::from_str(&value) {
                Ok(valid) => Ok(valid),
                Err(message) => Err(message.into()),
            }
        })
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for MigrationName {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <&str as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&self.0.as_str(), buf)
    }
}

impl sqlx::Type<sqlx::Postgres> for MigrationName {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl std::fmt::Display for MigrationName {
    /// Format migration name
    ///
    /// ```
    /// # use mmigration::*;
    /// # use std::str::FromStr;
    /// assert_eq!(
    ///     "example",
    ///     format!("{}", MigrationName::from_str("example").unwrap())
    /// );
    /// ```
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
    }
}

impl std::str::FromStr for MigrationName {
    type Err = ParseMigrationNameError;

    /// Parse string into migration name
    ///
    /// ### Examples
    ///
    /// ```
    /// # use mmigration::*;
    /// # use std::str::FromStr;
    /// assert!(MigrationName::from_str("bar").is_ok());
    /// assert!(MigrationName::from_str("foo_bar").is_ok());
    /// assert!(MigrationName::from_str("foo_bar_baz").is_ok());
    ///
    /// assert_eq!(
    ///     "migration name cannot consist of more than 128 characters (got 129)",
    ///     MigrationName::from_str(&("a".repeat(129))).unwrap_err().to_string()
    /// );
    /// ```
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match all_consuming(migration_name_parser).parse(value).finish() {
            Ok((_, parsed_name)) => {
                let actual = parsed_name.chars().count();
                if actual > 128 {
                    Err(ParseMigrationNameError::TooLong { max: 128, actual })
                } else {
                    Ok(Self::from_validated(parsed_name))
                }
            }
            Err(error) => Err(ParseMigrationNameError::Nom {
                report: nom_language::error::convert_error(value, error.clone()),
                error: error.into(),
            }),
        }
    }
}

fn is_migration_name_char(character: char) -> bool {
    character.is_ascii_lowercase() || character.is_ascii_digit()
}

pub(crate) fn migration_name_parser(input: &str) -> ParseResult<'_, &str> {
    context(
        "migration name (expected: alphanumeric lower snake case)",
        recognize(pair(
            take_while1(is_migration_name_char),
            many0_count(preceded(char('_'), take_while1(is_migration_name_char))),
        )),
    )
    .parse(input)
}

impl MigrationName {
    pub(crate) fn from_validated(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[macro_export]
macro_rules! migration_name {
    ($string: literal) => {
        <mmigration::MigrationName as std::str::FromStr>::from_str($string).unwrap()
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::str::FromStr;

    #[test]
    fn invalid_leading_underscore_error_exact() {
        let expected = indoc! {"
            0: at line 1, in TakeWhile1:
            _bar
            ^

            1: at line 1, in migration name (expected: alphanumeric lower snake case):
            _bar
            ^

        "};

        assert_eq!(
            expected,
            MigrationName::from_str("_bar").unwrap_err().to_string()
        );
    }

    #[test]
    fn invalid_trailing_underscore_error_exact() {
        let expected = indoc! {"
            0: at line 1, in Eof:
            bar_
               ^

        "};

        assert_eq!(
            expected,
            MigrationName::from_str("bar_").unwrap_err().to_string()
        );
    }

    #[test]
    fn invalid_symbol_error_exact() {
        let expected = indoc! {"
            0: at line 1, in TakeWhile1:
            ###
            ^

            1: at line 1, in migration name (expected: alphanumeric lower snake case):
            ###
            ^

        "};

        assert_eq!(
            expected,
            MigrationName::from_str("###").unwrap_err().to_string()
        );
    }

    #[test]
    fn too_long_error_exact() {
        assert_eq!(
            "migration name cannot consist of more than 128 characters (got 129)",
            MigrationName::from_str(&"a".repeat(129))
                .unwrap_err()
                .to_string()
        );
    }
}
