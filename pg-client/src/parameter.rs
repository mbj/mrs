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
pub struct Name(crate::protocol::String<'static>);

impl From<Name> for String {
    fn from(name: Name) -> Self {
        name.0.into_string()
    }
}

impl Name {
    /// Returns the parameter name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[must_use]
    pub const fn as_protocol_string(&self) -> &crate::protocol::String<'static> {
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
            Err(NameError::Reserved) => {
                panic!("PostgreSQL parameter name is a startup keyword, not a setting");
            }
        }
        Self(crate::protocol::String::from_static_or_panic(name))
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum NameError {
    #[error("PostgreSQL parameter name cannot be empty")]
    Empty,
    #[error("PostgreSQL parameter name must start with a letter or underscore")]
    InvalidStartCharacter,
    #[error("PostgreSQL parameter name contains an invalid character")]
    InvalidCharacter,
    #[error("PostgreSQL parameter name is a startup keyword, not a setting")]
    Reserved,
}

impl std::str::FromStr for Name {
    type Err = NameError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Self::try_from(name.to_owned())
    }
}

impl TryFrom<String> for Name {
    type Error = NameError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        validate_name(&name)?;

        crate::protocol::String::new(name)
            .map(Self)
            .map_err(|crate::protocol::StringError::ContainsNul { .. }| NameError::InvalidCharacter)
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
    if equals_ignoring_case(bytes, b"user")
        || equals_ignoring_case(bytes, b"database")
        || equals_ignoring_case(bytes, b"replication")
        || equals_ignoring_case(bytes, b"options")
    {
        return Err(NameError::Reserved);
    }
    Ok(())
}

const fn equals_ignoring_case(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut index = 0;
    while index < left.len() {
        if left[index].to_ascii_lowercase() != right[index] {
            return false;
        }
        index += 1;
    }
    true
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
pub struct Value(crate::protocol::String<'static>);

impl From<Value> for String {
    fn from(value: Value) -> Self {
        value.0.into_string()
    }
}

impl Value {
    /// Returns the parameter value as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[must_use]
    pub const fn as_protocol_string(&self) -> &crate::protocol::String<'static> {
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
        Self(crate::protocol::String::from_static_or_panic(value))
    }
}

impl AsRef<str> for Value {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ValueError {
    #[error("PostgreSQL parameter value contains NUL byte at index {index}")]
    ContainsNul { index: usize },
    #[error("PostgreSQL parameter value length {length} exceeds maximum {max}")]
    TooLong { length: usize, max: usize },
}

impl std::str::FromStr for Value {
    type Err = ValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value.to_owned())
    }
}

impl TryFrom<String> for Value {
    type Error = ValueError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > VALUE_MAX_LEN {
            return Err(ValueError::TooLong {
                length: value.len(),
                max: VALUE_MAX_LEN,
            });
        }

        crate::protocol::String::new(value).map(Self).map_err(
            |crate::protocol::StringError::ContainsNul { index }| ValueError::ContainsNul { index },
        )
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

/// A `NAME=VALUE` parameter assignment.
///
/// This is PG's own textual assignment syntax, shared by the `-c` command-line
/// flag, `postgresql.conf` entries, and `PGOPTIONS`. [`Name`] rejects `=`, so
/// the first `=` is unambiguously the separator and everything after it —
/// including further `=` characters — is the value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assignment {
    pub name: Name,
    pub value: Value,
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}={}", self.name, self.value)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AssignmentError {
    #[error("PostgreSQL parameter assignment expects NAME=VALUE, got: {input}")]
    MissingSeparator { input: String },
    #[error(transparent)]
    Name(#[from] NameError),
    #[error(transparent)]
    Value(#[from] ValueError),
}

impl std::str::FromStr for Assignment {
    type Err = AssignmentError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (name, value) =
            input
                .split_once('=')
                .ok_or_else(|| AssignmentError::MissingSeparator {
                    input: input.to_string(),
                })?;

        Ok(Self {
            name: name.parse()?,
            value: value.parse()?,
        })
    }
}

/// `DateStyle`: how the server writes dates, and how it reads an ambiguous one.
///
/// Reported in the same normalized form whatever form it was set in:
/// `ymd, sql` is reported as `SQL, YMD`, and setting only one half keeps the
/// other.
///
/// <https://www.postgresql.org/docs/current/runtime-config-client.html#GUC-DATESTYLE>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateStyle {
    pub output: DateOutput,
    pub order: DateOrder,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateOutput {
    Iso,
    Postgres,
    Sql,
    German,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DateOrder {
    Dmy,
    Mdy,
    Ymd,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum RuntimeParameterError {
    #[error("server did not report {name}")]
    Missing { name: &'static str },
    #[error("server reported a DateStyle this client cannot read")]
    DateStyle(#[source] DateStyleError),
    #[error("server reported an IntervalStyle this client cannot read")]
    IntervalStyle(#[source] IntervalStyleError),
    #[error("server reported an unreadable standard_conforming_strings")]
    StandardConformingStrings(#[source] BooleanError),
    #[error("server reported an unreadable is_superuser")]
    IsSuperuser(#[source] BooleanError),
    #[error("server reported an unreadable default_transaction_read_only")]
    DefaultTransactionReadOnly(#[source] BooleanError),
    #[error("server reported an unreadable in_hot_standby")]
    InHotStandby(#[source] BooleanError),
    #[error("server reported scram_iterations {value:?}, not a positive count")]
    ScramIterations { value: String },
    #[error("server reported a session_authorization that is not a role name")]
    SessionAuthorization(#[source] crate::name::ParseError),
    #[error("server reported an application_name that is not printable ASCII")]
    ApplicationName(#[source] crate::name::CleanAsciiError),
    /// Once a session moves to another client_encoding, the server sends
    /// every text value in that encoding from then on.
    #[error("server reported client_encoding {value:?}, and only UTF8 can be decoded")]
    ClientEncoding { value: String },
    #[error("server reported integer_datetimes {value:?}, and only on is possible")]
    IntegerDatetimes { value: String },
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum DateStyleError {
    #[error("DateStyle {value:?} is not an output format and a field order")]
    Shape { value: String },
    #[error("DateStyle output format {output:?} is not one PostgreSQL defines")]
    Output { output: String },
    #[error("DateStyle field order {order:?} is not one PostgreSQL defines")]
    Order { order: String },
}

impl std::str::FromStr for DateStyle {
    type Err = DateStyleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (output, order) = value
            .split_once(", ")
            .ok_or_else(|| DateStyleError::Shape {
                value: value.to_string(),
            })?;

        Ok(Self {
            output: match output {
                "ISO" => DateOutput::Iso,
                "Postgres" => DateOutput::Postgres,
                "SQL" => DateOutput::Sql,
                "German" => DateOutput::German,
                _ => {
                    return Err(DateStyleError::Output {
                        output: output.to_string(),
                    });
                }
            },
            order: match order {
                "DMY" => DateOrder::Dmy,
                "MDY" => DateOrder::Mdy,
                "YMD" => DateOrder::Ymd,
                _ => {
                    return Err(DateStyleError::Order {
                        order: order.to_string(),
                    });
                }
            },
        })
    }
}

/// <https://www.postgresql.org/docs/current/runtime-config-client.html#GUC-INTERVALSTYLE>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntervalStyle {
    Postgres,
    PostgresVerbose,
    SqlStandard,
    Iso8601,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("IntervalStyle {value:?} is not one PostgreSQL defines")]
pub struct IntervalStyleError {
    value: String,
}

impl std::str::FromStr for IntervalStyle {
    type Err = IntervalStyleError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "postgres" => Ok(Self::Postgres),
            "postgres_verbose" => Ok(Self::PostgresVerbose),
            "sql_standard" => Ok(Self::SqlStandard),
            "iso_8601" => Ok(Self::Iso8601),
            _ => Err(IntervalStyleError {
                value: value.to_string(),
            }),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{value:?} is neither on nor off")]
pub struct BooleanError {
    value: String,
}

fn boolean(value: &str) -> Result<bool, BooleanError> {
    match value {
        "on" => Ok(true),
        "off" => Ok(false),
        _ => Err(BooleanError {
            value: value.to_string(),
        }),
    }
}

/// The server may report offsets and special names: `+02:00`, `localtime`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimeZone(Value);

impl TimeZone {
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// `search_path`, as written: quoted where needed, and holding `$user`, which
/// names no schema of its own.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchPath(Value);

impl SearchPath {
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Packagers append to the version: `18.6 (Debian 18.6-1.pgdg13+2)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServerVersion(Value);

impl ServerVersion {
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Fixed when the database was created.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServerEncoding(Value);

impl ServerEncoding {
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub const DATE_STYLE: &str = "DateStyle";
pub const INTERVAL_STYLE: &str = "IntervalStyle";
pub const TIME_ZONE: &str = "TimeZone";
pub const STANDARD_CONFORMING_STRINGS: &str = "standard_conforming_strings";
pub const SERVER_VERSION: &str = "server_version";
pub const SERVER_ENCODING: &str = "server_encoding";
pub const SEARCH_PATH: &str = "search_path";
pub const SESSION_AUTHORIZATION: &str = "session_authorization";
pub const APPLICATION_NAME: &str = "application_name";
pub const IS_SUPERUSER: &str = "is_superuser";
pub const DEFAULT_TRANSACTION_READ_ONLY: &str = "default_transaction_read_only";
pub const IN_HOT_STANDBY: &str = "in_hot_standby";
pub const SCRAM_ITERATIONS: &str = "scram_iterations";
pub const CLIENT_ENCODING: &str = "client_encoding";
pub const INTEGER_DATETIMES: &str = "integer_datetimes";

/// `server_version` and `server_encoding` cannot change after start-up;
/// upstream calls them pseudo-parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeParameters {
    pub date_style: DateStyle,
    pub interval_style: IntervalStyle,
    pub time_zone: TimeZone,
    pub standard_conforming_strings: bool,
    pub server_version: ServerVersion,
    pub server_encoding: ServerEncoding,
    pub search_path: Option<SearchPath>,
    pub session_authorization: crate::name::Role<'static>,
    pub application_name: Option<crate::name::EffectiveApplicationName>,
    pub is_superuser: bool,
    pub default_transaction_read_only: bool,
    pub in_hot_standby: bool,
    pub scram_iterations: Option<std::num::NonZeroU32>,
}

impl RuntimeParameters {
    pub fn apply(
        &mut self,
        status: crate::protocol::backend::ParameterStatus,
    ) -> Result<(), RuntimeParameterError> {
        let crate::protocol::backend::ParameterStatus { name, value } = status;

        match name.as_str() {
            DATE_STYLE => self.date_style = date_style(&value)?,
            INTERVAL_STYLE => self.interval_style = interval_style(&value)?,
            TIME_ZONE => self.time_zone = TimeZone(value),
            STANDARD_CONFORMING_STRINGS => {
                self.standard_conforming_strings = boolean(value.as_str())
                    .map_err(RuntimeParameterError::StandardConformingStrings)?;
            }
            SERVER_VERSION => self.server_version = ServerVersion(value),
            SERVER_ENCODING => self.server_encoding = ServerEncoding(value),
            SEARCH_PATH => self.search_path = Some(SearchPath(value)),
            SESSION_AUTHORIZATION => self.session_authorization = session_authorization(&value)?,
            APPLICATION_NAME => self.application_name = application_name(&value)?,
            IS_SUPERUSER => {
                self.is_superuser =
                    boolean(value.as_str()).map_err(RuntimeParameterError::IsSuperuser)?;
            }
            DEFAULT_TRANSACTION_READ_ONLY => {
                self.default_transaction_read_only = boolean(value.as_str())
                    .map_err(RuntimeParameterError::DefaultTransactionReadOnly)?;
            }
            IN_HOT_STANDBY => {
                self.in_hot_standby =
                    boolean(value.as_str()).map_err(RuntimeParameterError::InHotStandby)?;
            }
            SCRAM_ITERATIONS => self.scram_iterations = Some(scram_iterations(&value)?),
            CLIENT_ENCODING => client_encoding(&value)?,
            INTEGER_DATETIMES => integer_datetimes(&value)?,
            // Not one kept here, which upstream asks frontends to ignore.
            _ => (),
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct StartupParameters {
    date_style: Option<DateStyle>,
    interval_style: Option<IntervalStyle>,
    time_zone: Option<TimeZone>,
    standard_conforming_strings: Option<bool>,
    server_version: Option<ServerVersion>,
    server_encoding: Option<ServerEncoding>,
    search_path: Option<SearchPath>,
    session_authorization: Option<crate::name::Role<'static>>,
    application_name: Option<crate::name::EffectiveApplicationName>,
    is_superuser: Option<bool>,
    default_transaction_read_only: Option<bool>,
    in_hot_standby: Option<bool>,
    scram_iterations: Option<std::num::NonZeroU32>,
}

impl StartupParameters {
    pub(crate) fn apply(
        &mut self,
        status: crate::protocol::backend::ParameterStatus,
    ) -> Result<(), RuntimeParameterError> {
        let crate::protocol::backend::ParameterStatus { name, value } = status;

        match name.as_str() {
            DATE_STYLE => self.date_style = Some(date_style(&value)?),
            INTERVAL_STYLE => self.interval_style = Some(interval_style(&value)?),
            TIME_ZONE => self.time_zone = Some(TimeZone(value)),
            STANDARD_CONFORMING_STRINGS => {
                self.standard_conforming_strings = Some(
                    boolean(value.as_str())
                        .map_err(RuntimeParameterError::StandardConformingStrings)?,
                );
            }
            SERVER_VERSION => self.server_version = Some(ServerVersion(value)),
            SERVER_ENCODING => self.server_encoding = Some(ServerEncoding(value)),
            SEARCH_PATH => self.search_path = Some(SearchPath(value)),
            SESSION_AUTHORIZATION => {
                self.session_authorization = Some(session_authorization(&value)?);
            }
            APPLICATION_NAME => self.application_name = application_name(&value)?,
            IS_SUPERUSER => {
                self.is_superuser =
                    Some(boolean(value.as_str()).map_err(RuntimeParameterError::IsSuperuser)?);
            }
            DEFAULT_TRANSACTION_READ_ONLY => {
                self.default_transaction_read_only = Some(
                    boolean(value.as_str())
                        .map_err(RuntimeParameterError::DefaultTransactionReadOnly)?,
                );
            }
            IN_HOT_STANDBY => {
                self.in_hot_standby =
                    Some(boolean(value.as_str()).map_err(RuntimeParameterError::InHotStandby)?);
            }
            SCRAM_ITERATIONS => self.scram_iterations = Some(scram_iterations(&value)?),
            CLIENT_ENCODING => client_encoding(&value)?,
            INTEGER_DATETIMES => integer_datetimes(&value)?,
            _ => (),
        }

        Ok(())
    }
}

impl TryFrom<StartupParameters> for RuntimeParameters {
    type Error = RuntimeParameterError;

    fn try_from(startup: StartupParameters) -> Result<Self, Self::Error> {
        Ok(Self {
            date_style: required(startup.date_style, DATE_STYLE)?,
            interval_style: required(startup.interval_style, INTERVAL_STYLE)?,
            time_zone: required(startup.time_zone, TIME_ZONE)?,
            standard_conforming_strings: required(
                startup.standard_conforming_strings,
                STANDARD_CONFORMING_STRINGS,
            )?,
            server_version: required(startup.server_version, SERVER_VERSION)?,
            server_encoding: required(startup.server_encoding, SERVER_ENCODING)?,
            search_path: startup.search_path,
            session_authorization: required(startup.session_authorization, SESSION_AUTHORIZATION)?,
            application_name: startup.application_name,
            is_superuser: required(startup.is_superuser, IS_SUPERUSER)?,
            default_transaction_read_only: required(
                startup.default_transaction_read_only,
                DEFAULT_TRANSACTION_READ_ONLY,
            )?,
            in_hot_standby: required(startup.in_hot_standby, IN_HOT_STANDBY)?,
            scram_iterations: startup.scram_iterations,
        })
    }
}

fn required<T>(slot: Option<T>, name: &'static str) -> Result<T, RuntimeParameterError> {
    slot.ok_or(RuntimeParameterError::Missing { name })
}

fn date_style(value: &Value) -> Result<DateStyle, RuntimeParameterError> {
    value
        .as_str()
        .parse()
        .map_err(RuntimeParameterError::DateStyle)
}

fn interval_style(value: &Value) -> Result<IntervalStyle, RuntimeParameterError> {
    value
        .as_str()
        .parse()
        .map_err(RuntimeParameterError::IntervalStyle)
}

fn client_encoding(value: &Value) -> Result<(), RuntimeParameterError> {
    if value.as_str() == "UTF8" {
        Ok(())
    } else {
        Err(RuntimeParameterError::ClientEncoding {
            value: value.as_str().to_string(),
        })
    }
}

fn integer_datetimes(value: &Value) -> Result<(), RuntimeParameterError> {
    if value.as_str() == "on" {
        Ok(())
    } else {
        Err(RuntimeParameterError::IntegerDatetimes {
            value: value.as_str().to_string(),
        })
    }
}

fn application_name(
    value: &Value,
) -> Result<Option<crate::name::EffectiveApplicationName>, RuntimeParameterError> {
    let name = value.as_str();

    if name.is_empty() {
        return Ok(None);
    }

    crate::name::EffectiveApplicationName::reported(name)
        .map(Some)
        .map_err(RuntimeParameterError::ApplicationName)
}

fn session_authorization(
    value: &Value,
) -> Result<crate::name::Role<'static>, RuntimeParameterError> {
    value
        .as_str()
        .parse()
        .map_err(RuntimeParameterError::SessionAuthorization)
}

fn scram_iterations(value: &Value) -> Result<std::num::NonZeroU32, RuntimeParameterError> {
    value
        .as_str()
        .parse()
        .map_err(|_| RuntimeParameterError::ScramIterations {
            value: value.as_str().to_string(),
        })
}

#[cfg(test)]
mod name_tests {
    use super::*;

    #[test]
    fn refuses_startup_keywords() {
        for name in [
            "user",
            "database",
            "replication",
            "options",
            "User",
            "DATABASE",
        ] {
            assert_eq!(name.parse::<Name>(), Err(NameError::Reserved), "{name}");
        }
    }

    #[test]
    fn accepts_names_that_only_contain_a_startup_keyword() {
        for name in ["username", "user.id", "database_name", "myapp.options"] {
            assert_eq!(name.parse::<Name>().unwrap().as_str(), name);
        }
    }

    #[test]
    fn refuses_a_startup_keyword_as_assignment() {
        assert_eq!(
            "user=postgres"
                .parse::<Assignment>()
                .unwrap_err()
                .to_string(),
            "PostgreSQL parameter name is a startup keyword, not a setting"
        );
    }
}

#[cfg(test)]
mod assignment_tests {
    use super::*;

    fn parse(input: &str) -> Result<Assignment, AssignmentError> {
        input.parse()
    }

    #[test]
    fn parses_name_and_value() {
        assert_eq!(
            parse("work_mem=16MB").unwrap(),
            Assignment {
                name: Name::from_static_or_panic("work_mem"),
                value: Value::from_static_or_panic("16MB"),
            }
        );
    }

    #[test]
    fn splits_on_first_separator_only() {
        assert_eq!(
            parse("log_line_prefix=%m [%p] db=%d").unwrap(),
            Assignment {
                name: Name::from_static_or_panic("log_line_prefix"),
                value: Value::from_static_or_panic("%m [%p] db=%d"),
            }
        );
    }

    #[test]
    fn parses_namespaced_extension_parameter() {
        assert_eq!(
            parse("pg_stat_statements.track=all").unwrap(),
            Assignment {
                name: Name::from_static_or_panic("pg_stat_statements.track"),
                value: Value::from_static_or_panic("all"),
            }
        );
    }

    #[test]
    fn parses_empty_value() {
        assert_eq!(
            parse("shared_preload_libraries=").unwrap(),
            Assignment {
                name: Name::from_static_or_panic("shared_preload_libraries"),
                value: Value::from_static_or_panic(""),
            }
        );
    }

    #[test]
    fn rejects_missing_separator() {
        assert_eq!(
            parse("work_mem").unwrap_err().to_string(),
            "PostgreSQL parameter assignment expects NAME=VALUE, got: work_mem"
        );
    }

    #[test]
    fn rejects_invalid_name() {
        assert_eq!(
            parse("9invalid=x").unwrap_err().to_string(),
            "PostgreSQL parameter name must start with a letter or underscore"
        );
    }

    #[test]
    fn display_round_trips() {
        let input = "log_line_prefix=%m [%p] db=%d";
        assert_eq!(parse(input).unwrap().to_string(), input);
    }
}

#[cfg(test)]
mod date_style_tests {
    use super::*;

    /// As a server reported each after being set.
    #[test]
    fn reads_the_form_a_server_reports() {
        assert_eq!(
            "SQL, YMD".parse::<DateStyle>(),
            Ok(DateStyle {
                output: DateOutput::Sql,
                order: DateOrder::Ymd,
            })
        );
        assert_eq!(
            "German, DMY".parse::<DateStyle>(),
            Ok(DateStyle {
                output: DateOutput::German,
                order: DateOrder::Dmy,
            })
        );
    }
}

#[cfg(test)]
mod runtime_parameters_tests {
    use super::*;
    use crate::protocol::backend::ParameterStatus;
    use pretty_assertions::assert_eq;

    /// The start-up burst a PostgreSQL 18 server sent.
    const STARTUP: [(&str, &str); 15] = [
        ("DateStyle", "ISO, MDY"),
        ("IntervalStyle", "postgres"),
        ("TimeZone", "Etc/UTC"),
        ("application_name", ""),
        ("client_encoding", "UTF8"),
        ("default_transaction_read_only", "off"),
        ("in_hot_standby", "off"),
        ("integer_datetimes", "on"),
        ("is_superuser", "on"),
        ("scram_iterations", "4096"),
        ("search_path", "\"$user\", public"),
        ("server_encoding", "UTF8"),
        ("server_version", "18.6 (Debian 18.6-1.pgdg13+2)"),
        ("session_authorization", "postgres"),
        ("standard_conforming_strings", "on"),
    ];

    fn status(name: &'static str, value: &'static str) -> ParameterStatus {
        ParameterStatus {
            name: Name::from_static_or_panic(name),
            value: Value::from_static_or_panic(value),
        }
    }

    fn settle(
        reports: &[(&'static str, &'static str)],
    ) -> Result<RuntimeParameters, RuntimeParameterError> {
        let mut startup = StartupParameters::default();

        for (name, value) in reports {
            startup.apply(status(name, value))?;
        }

        startup.try_into()
    }

    #[test]
    fn reads_the_startup_burst_a_server_sends() {
        assert_eq!(
            settle(&STARTUP),
            Ok(RuntimeParameters {
                date_style: DateStyle {
                    output: DateOutput::Iso,
                    order: DateOrder::Mdy,
                },
                interval_style: IntervalStyle::Postgres,
                time_zone: TimeZone(Value::from_static_or_panic("Etc/UTC")),
                standard_conforming_strings: true,
                server_version: ServerVersion(Value::from_static_or_panic(
                    "18.6 (Debian 18.6-1.pgdg13+2)"
                )),
                server_encoding: ServerEncoding(Value::from_static_or_panic("UTF8")),
                search_path: Some(SearchPath(Value::from_static_or_panic("\"$user\", public"))),
                session_authorization: crate::name::Role::from_static_or_panic("postgres"),
                application_name: None,
                is_superuser: true,
                default_transaction_read_only: false,
                in_hot_standby: false,
                scram_iterations: std::num::NonZeroU32::new(4096),
            })
        );
    }

    #[test]
    fn accepts_the_startup_burst_of_postgresql_15() {
        let fifteen: Vec<_> = STARTUP
            .into_iter()
            .filter(|(name, _)| ![SEARCH_PATH, SCRAM_ITERATIONS].contains(name))
            .collect();

        let parameters = settle(&fifteen).unwrap();

        assert_eq!(
            (parameters.search_path, parameters.scram_iterations),
            (None, None)
        );
    }

    #[test]
    fn applies_a_change_the_server_reports() {
        let mut parameters = settle(&STARTUP).unwrap();

        parameters.apply(status("DateStyle", "SQL, YMD")).unwrap();

        assert_eq!(
            parameters.date_style,
            DateStyle {
                output: DateOutput::Sql,
                order: DateOrder::Ymd,
            }
        );
    }

    #[test]
    fn follows_application_name_being_set_and_cleared() {
        let mut parameters = settle(&STARTUP).unwrap();

        parameters
            .apply(status("application_name", "psql"))
            .unwrap();
        assert_eq!(
            parameters.application_name,
            crate::name::EffectiveApplicationName::reported("psql").ok()
        );

        parameters.apply(status("application_name", "")).unwrap();
        assert_eq!(parameters.application_name, None);
    }

    #[test]
    fn ignores_a_parameter_it_does_not_keep() {
        let mut parameters = settle(&STARTUP).unwrap();
        let before = parameters.clone();

        assert_eq!(parameters.apply(status("work_mem", "64MB")), Ok(()));
        assert_eq!(parameters, before);
    }

    #[test]
    fn names_a_parameter_that_never_arrived() {
        let without_time_zone: Vec<_> = STARTUP
            .into_iter()
            .filter(|(name, _)| *name != TIME_ZONE)
            .collect();

        assert_eq!(
            settle(&without_time_zone),
            Err(RuntimeParameterError::Missing { name: TIME_ZONE })
        );
    }

    #[test]
    fn refuses_a_session_moved_off_utf8() {
        let mut parameters = settle(&STARTUP).unwrap();

        assert_eq!(
            parameters.apply(status("client_encoding", "LATIN1")),
            Err(RuntimeParameterError::ClientEncoding {
                value: "LATIN1".to_owned()
            })
        );
    }
}
