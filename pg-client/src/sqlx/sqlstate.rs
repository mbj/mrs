//! Postgres SQLSTATE codes.
//!
//! sqlx exposes a SQLSTATE as `Option<Cow<'_, str>>` via
//! [`sqlx::error::DatabaseError::code`]. Comparing those strings to literals at
//! every call site is ergonomically poor. [`SqlState`] wraps the 5-byte code
//! with `const`-able named constants, and [`sqlstate`] extracts one from a
//! [`sqlx::Error`].
//!
//! sqlx's own [`sqlx::error::ErrorKind`] enum only classifies constraint
//! violations (class 23) and is `#[non_exhaustive]`; it does not type the
//! transaction-rollback codes (class 40) that retry logic typically cares
//! about. This module fills that gap.

/// A Postgres SQLSTATE — a five-character error code.
///
/// SQLSTATE codes are organised into a two-character class followed by a
/// three-character subclass. See the [Postgres errcodes appendix][1] for the
/// full list.
///
/// [1]: https://www.postgresql.org/docs/current/errcodes-appendix.html
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SqlState([u8; 5]);

impl SqlState {
    /// Class 40 — Transaction Rollback: `serialization_failure`.
    pub const SERIALIZATION_FAILURE: Self = Self(*b"40001");

    /// Class 40 — Transaction Rollback: `deadlock_detected`.
    pub const DEADLOCK_DETECTED: Self = Self(*b"40P01");

    /// Class 55 — Object Not In Prerequisite State: `lock_not_available`.
    ///
    /// Raised by `LOCK TABLE … NOWAIT` when the lock cannot be acquired immediately.
    pub const LOCK_NOT_AVAILABLE: Self = Self(*b"55P03");

    /// Construct from a fixed-size byte array.
    ///
    /// No validation is performed; callers are expected to supply ASCII
    /// alphanumeric bytes as defined by the SQLSTATE specification.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 5]) -> Self {
        Self(bytes)
    }

    /// Borrow the underlying bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 5] {
        &self.0
    }

    /// The two-byte class portion of the code (the first two characters).
    #[must_use]
    pub const fn class(&self) -> [u8; 2] {
        [self.0[0], self.0[1]]
    }
}

/// Failure to parse a `SqlState` from a string.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("SQLSTATE must be exactly 5 bytes, got {got}")]
pub struct ParseSqlStateError {
    pub got: usize,
}

impl std::str::FromStr for SqlState {
    type Err = ParseSqlStateError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value
            .as_bytes()
            .try_into()
            .map(Self)
            .map_err(|_| ParseSqlStateError { got: value.len() })
    }
}

impl std::fmt::Display for SqlState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // SAFETY: SqlState is constructed from either a 5-byte ASCII literal
        // (const constructors) or from a valid `&str` (FromStr), so the bytes
        // are always valid UTF-8.
        formatter.write_str(std::str::from_utf8(&self.0).expect("SqlState is ASCII"))
    }
}

/// Extract the SQLSTATE from a sqlx error, when it carries a database error
/// with a code that parses to five bytes.
///
/// Returns `None` for client-side errors (`Io`, `PoolTimedOut`, `Connect`, …)
/// and for database errors whose driver did not surface a code.
#[must_use]
pub fn sqlstate(error: &sqlx::Error) -> Option<SqlState> {
    error
        .as_database_error()
        .and_then(|db| db.code())
        .and_then(|code| code.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn from_str_round_trips_well_known_code() {
        let parsed: SqlState = "40001".parse().unwrap();
        assert_eq!(SqlState::SERIALIZATION_FAILURE, parsed);
    }

    #[test]
    fn from_str_round_trips_letter_subclass() {
        let parsed: SqlState = "40P01".parse().unwrap();
        assert_eq!(SqlState::DEADLOCK_DETECTED, parsed);
    }

    #[test]
    fn from_str_rejects_short_input() {
        assert_eq!(
            ParseSqlStateError { got: 3 },
            SqlState::from_str("400").unwrap_err()
        );
    }

    #[test]
    fn from_str_rejects_long_input() {
        assert_eq!(
            ParseSqlStateError { got: 6 },
            SqlState::from_str("400015").unwrap_err()
        );
    }

    #[test]
    fn from_str_rejects_empty_input() {
        assert_eq!(
            ParseSqlStateError { got: 0 },
            SqlState::from_str("").unwrap_err()
        );
    }

    #[test]
    fn display_renders_canonical_code() {
        assert_eq!("40001", SqlState::SERIALIZATION_FAILURE.to_string());
    }

    #[test]
    fn display_renders_letter_subclass() {
        assert_eq!("40P01", SqlState::DEADLOCK_DETECTED.to_string());
    }

    #[test]
    fn class_returns_first_two_bytes() {
        assert_eq!(*b"40", SqlState::SERIALIZATION_FAILURE.class());
        assert_eq!(*b"40", SqlState::DEADLOCK_DETECTED.class());
    }

    #[test]
    fn as_bytes_returns_five_byte_slice() {
        assert_eq!(b"40001", SqlState::SERIALIZATION_FAILURE.as_bytes());
    }

    #[test]
    fn parse_error_message_includes_observed_length() {
        assert_eq!(
            "SQLSTATE must be exactly 5 bytes, got 3",
            ParseSqlStateError { got: 3 }.to_string()
        );
    }
}
