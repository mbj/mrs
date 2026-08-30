use core::fmt;

use crate::name::{Channel, Column, Constraint, ParseError, Schema, Table, Type};
use crate::parameter;
use crate::protocol::{Format, Frame, Oid, Version};

fn parse_identifier<'a, T>(
    field: FieldType,
    value: &'a str,
    new: fn(&'a str) -> Result<T, ParseError>,
) -> Result<T, DecodeError> {
    new(value).map_err(|source| DecodeError::NotAnIdentifier {
        field,
        value: value.to_owned(),
        source,
    })
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum DecodeError {
    #[error("tag {tag:?} is not a backend message")]
    UnknownTag { tag: u8 },
    #[error("body holds {remaining} bytes, needed {needed}")]
    Truncated { remaining: usize, needed: usize },
    #[error("body has {remaining} bytes past the end of the message")]
    Trailing { remaining: usize },
    #[error("authentication method {method} is not defined")]
    UnknownAuthentication { method: i32 },
    #[error("value is not NUL terminated")]
    Unterminated,
    #[error("value is not UTF-8")]
    NotUtf8,
    #[error("server reported protocol options that were never requested: {options:?}")]
    UnsupportedProtocolOptions { options: Vec<String> },
    #[error("protocol version code {code:#010x} is not one this client speaks")]
    UnsupportedProtocolVersion { code: u32 },
    #[error("message is missing its {field:?}")]
    MissingField { field: FieldType },
    #[error("severity {value:?} is not one upstream defines")]
    UnknownSeverity { value: String },
    #[error("SQLSTATE {value:?} is not five digits or upper case letters")]
    InvalidSqlState { value: String },
    #[error("{field:?} carries {value:?}, which is not a decimal integer")]
    NotAnInteger { field: FieldType, value: String },
    #[error("{field:?} carries {value:?}, which is not a valid identifier")]
    NotAnIdentifier {
        field: FieldType,
        value: String,
        #[source]
        source: ParseError,
    },
    #[error("parameter name {value:?} is not valid")]
    NotAParameterName {
        value: String,
        #[source]
        source: parameter::NameError,
    },
    #[error("parameter value {value:?} is not valid")]
    NotAParameterValue {
        value: String,
        #[source]
        source: parameter::ValueError,
    },
    #[error("cancel key is {length} bytes, which is outside 4 to 256")]
    InvalidCancelKey { length: usize },
    #[error("copy format is textual but a column is binary")]
    TextualCopyWithBinaryColumn,
    #[error("transaction status {:?} is not one upstream defines", char::from(*status))]
    UnknownTransactionStatus { status: u8 },
    #[error("command tag {tag:?} does not carry the counts its command defines")]
    MalformedCommandTag { tag: String },
    #[error("notification channel {value:?} is not a valid name")]
    InvalidChannel {
        value: String,
        #[source]
        source: ParseError,
    },
    #[error("result column name {value:?} is not a valid name")]
    InvalidColumnName {
        value: String,
        #[source]
        source: ParseError,
    },
    #[error("message declares {count} entries")]
    NegativeCount { count: i16 },
    #[error("format code {code} is neither text nor binary")]
    UnknownFormat { code: i16 },
}

struct Cursor<'a> {
    bytes: &'a [u8],
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    fn take(&mut self, needed: usize) -> Result<&'a [u8], DecodeError> {
        if self.bytes.len() < needed {
            return Err(DecodeError::Truncated {
                remaining: self.bytes.len(),
                needed,
            });
        }

        let (taken, rest) = self.bytes.split_at(needed);

        self.bytes = rest;

        Ok(taken)
    }

    fn int32(&mut self) -> Result<i32, DecodeError> {
        let bytes = self.take(4)?;

        Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn uint32(&mut self) -> Result<u32, DecodeError> {
        let bytes = self.take(4)?;

        Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn int16(&mut self) -> Result<i16, DecodeError> {
        let bytes = self.take(2)?;

        Ok(i16::from_be_bytes([bytes[0], bytes[1]]))
    }

    fn oid(&mut self) -> Result<Oid, DecodeError> {
        let bytes = self.take(4)?;

        Ok(Oid::new(u32::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
        ])))
    }

    fn byte(&mut self) -> Result<u8, DecodeError> {
        Ok(self.take(1)?[0])
    }

    fn count(&mut self) -> Result<usize, DecodeError> {
        let count = self.int16()?;

        usize::try_from(count).map_err(|_| DecodeError::NegativeCount { count })
    }

    fn string(&mut self) -> Result<&'a str, DecodeError> {
        let end = self
            .bytes
            .iter()
            .position(|byte| *byte == 0)
            .ok_or(DecodeError::Unterminated)?;

        let value = core::str::from_utf8(&self.bytes[..end]).map_err(|_| DecodeError::NotUtf8)?;

        self.bytes = &self.bytes[end + 1..];

        Ok(value)
    }

    const fn rest(&self) -> &'a [u8] {
        self.bytes
    }

    const fn end(self) -> Result<(), DecodeError> {
        if self.bytes.is_empty() {
            Ok(())
        } else {
            Err(DecodeError::Trailing {
                remaining: self.bytes.len(),
            })
        }
    }
}

/// Listed in the server's order of preference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mechanisms<'a> {
    names: &'a str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mechanism<'a> {
    Known(crate::authentication::Mechanism),
    Other(&'a str),
}

#[derive(Clone, Debug)]
pub struct MechanismIter<'a> {
    names: core::str::SplitTerminator<'a, char>,
}

impl<'a> Iterator for MechanismIter<'a> {
    type Item = Mechanism<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.names.next().map(|name| {
            crate::authentication::Mechanism::from_name(name)
                .map_or(Mechanism::Other(name), Mechanism::Known)
        })
    }
}

impl<'a> IntoIterator for Mechanisms<'a> {
    type IntoIter = MechanismIter<'a>;
    type Item = Mechanism<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MechanismIter {
            names: self.names.split_terminator('\0'),
        }
    }
}

impl<'a> Mechanisms<'a> {
    fn decode(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        let names = bytes.strip_suffix(&[0]).ok_or(DecodeError::Unterminated)?;

        Ok(Self {
            names: core::str::from_utf8(names).map_err(|_| DecodeError::NotUtf8)?,
        })
    }
}

/// The protocol carries SASL payloads without defining them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Authentication<'a> {
    /// [`AuthenticationOk`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-AUTHENTICATIONOK)
    Ok,
    /// [`AuthenticationKerberosV5`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-AUTHENTICATIONKERBEROSV5)
    KerberosV5,
    /// [`AuthenticationCleartextPassword`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-AUTHENTICATIONCLEARTEXTPASSWORD)
    CleartextPassword,
    /// [`AuthenticationMD5Password`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-AUTHENTICATIONMD5PASSWORD)
    Md5Password { salt: [u8; 4] },
    /// [`AuthenticationGSS`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-AUTHENTICATIONGSS)
    Gss,
    /// [`AuthenticationGSSContinue`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-AUTHENTICATIONGSSCONTINUE)
    GssContinue { data: &'a [u8] },
    /// [`AuthenticationSSPI`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-AUTHENTICATIONSSPI)
    Sspi,
    /// [`AuthenticationSASL`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-AUTHENTICATIONSASL)
    Sasl { mechanisms: Mechanisms<'a> },
    /// [`AuthenticationSASLContinue`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-AUTHENTICATIONSASLCONTINUE)
    SaslContinue { data: &'a [u8] },
    /// [`AuthenticationSASLFinal`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-AUTHENTICATIONSASLFINAL)
    SaslFinal { data: &'a [u8] },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthenticationRequest {
    Ok,
    KerberosV5,
    CleartextPassword,
    Md5Password,
    Gss,
    GssContinue,
    Sspi,
    Sasl,
    SaslContinue,
    SaslFinal,
}

impl<'a> Authentication<'a> {
    #[must_use]
    pub const fn request(&self) -> AuthenticationRequest {
        match self {
            Self::Ok => AuthenticationRequest::Ok,
            Self::KerberosV5 => AuthenticationRequest::KerberosV5,
            Self::CleartextPassword => AuthenticationRequest::CleartextPassword,
            Self::Md5Password { .. } => AuthenticationRequest::Md5Password,
            Self::Gss => AuthenticationRequest::Gss,
            Self::GssContinue { .. } => AuthenticationRequest::GssContinue,
            Self::Sspi => AuthenticationRequest::Sspi,
            Self::Sasl { .. } => AuthenticationRequest::Sasl,
            Self::SaslContinue { .. } => AuthenticationRequest::SaslContinue,
            Self::SaslFinal { .. } => AuthenticationRequest::SaslFinal,
        }
    }

    fn decode(mut cursor: Cursor<'a>) -> Result<Self, DecodeError> {
        let method = cursor.int32()?;

        let message = match method {
            0 => Self::Ok,
            2 => Self::KerberosV5,
            3 => Self::CleartextPassword,
            5 => {
                let salt = cursor.take(4)?;

                Self::Md5Password {
                    salt: [salt[0], salt[1], salt[2], salt[3]],
                }
            }
            7 => Self::Gss,
            8 => {
                return Ok(Self::GssContinue {
                    data: cursor.rest(),
                });
            }
            9 => Self::Sspi,
            10 => {
                return Ok(Self::Sasl {
                    mechanisms: Mechanisms::decode(cursor.rest())?,
                });
            }
            11 => {
                return Ok(Self::SaslContinue {
                    data: cursor.rest(),
                });
            }
            12 => {
                return Ok(Self::SaslFinal {
                    data: cursor.rest(),
                });
            }
            method => return Err(DecodeError::UnknownAuthentication { method }),
        };

        cursor.end()?;

        Ok(message)
    }
}

/// Sent when the requested minor version is newer than the server supports, and
/// also when the packet carried `_pq_.` options it does not implement. The two
/// are independent, and authentication continues at the version the server
/// names. Upstream says a client that cannot speak that version should close
/// the connection immediately.
///
/// Format: <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-NEGOTIATEPROTOCOLVERSION>
///
/// Flow: <https://www.postgresql.org/docs/current/protocol-flow.html#PROTOCOL-FLOW-START-UP>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NegotiateProtocolVersion {
    /// Upstream's message format calls this the newest *minor* version, but
    /// `SendNegotiateProtocolVersion` sends `FrontendProtocol`, which is
    /// `min(requested, latest)` — a whole version. PostgreSQL 17 answers 3.0,
    /// not 0.
    pub version: Version,
}

impl NegotiateProtocolVersion {
    fn decode(mut cursor: Cursor<'_>) -> Result<Self, DecodeError> {
        let code = cursor.uint32()?;
        let version =
            Version::from_code(code).ok_or(DecodeError::UnsupportedProtocolVersion { code })?;
        let options = cursor.int32()?;

        if options != 0 {
            let names = String::from_utf8_lossy(cursor.rest());

            return Err(DecodeError::UnsupportedProtocolOptions {
                options: names.split_terminator('\0').map(str::to_owned).collect(),
            });
        }

        cursor.end()?;

        Ok(Self { version })
    }
}

/// Upstream says more field types may be added, and that a frontend should
/// silently ignore ones it does not recognise.
///
/// <https://www.postgresql.org/docs/current/protocol-error-fields.html>
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldType(u8);

impl core::fmt::Debug for FieldType {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "FieldType({:?})", char::from(self.0))
    }
}

impl FieldType {
    /// Never localized, and sent by every server from PostgreSQL 9.6 on.
    pub const SEVERITY: Self = Self(b'V');
    /// May be a localized translation.
    pub const SEVERITY_LOCALIZED: Self = Self(b'S');
    pub const CODE: Self = Self(b'C');
    pub const MESSAGE: Self = Self(b'M');
    pub const DETAIL: Self = Self(b'D');
    pub const HINT: Self = Self(b'H');
    /// Index into the original query string, one based, in decimal ASCII.
    pub const POSITION: Self = Self(b'P');
    pub const INTERNAL_POSITION: Self = Self(b'p');
    pub const INTERNAL_QUERY: Self = Self(b'q');
    pub const WHERE: Self = Self(b'W');
    pub const SCHEMA_NAME: Self = Self(b's');
    pub const TABLE_NAME: Self = Self(b't');
    pub const COLUMN_NAME: Self = Self(b'c');
    pub const DATA_TYPE_NAME: Self = Self(b'd');
    pub const CONSTRAINT_NAME: Self = Self(b'n');
    pub const FILE: Self = Self(b'F');
    pub const LINE: Self = Self(b'L');
    pub const ROUTINE: Self = Self(b'R');

    #[must_use]
    pub const fn new(code: u8) -> Self {
        Self(code)
    }

    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

/// PostgreSQL sends the first three in errors and the rest in notices.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Fatal,
    Panic,
    Warning,
    Notice,
    Debug,
    Info,
    Log,
}

impl Severity {
    fn new(value: &str) -> Result<Self, DecodeError> {
        Ok(match value {
            "ERROR" => Self::Error,
            "FATAL" => Self::Fatal,
            "PANIC" => Self::Panic,
            "WARNING" => Self::Warning,
            "NOTICE" => Self::Notice,
            "DEBUG" => Self::Debug,
            "INFO" => Self::Info,
            "LOG" => Self::Log,
            value => {
                return Err(DecodeError::UnknownSeverity {
                    value: value.to_owned(),
                });
            }
        })
    }

    #[must_use]
    pub const fn ends_session(self) -> bool {
        matches!(self, Self::Fatal | Self::Panic)
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "ERROR",
            Self::Fatal => "FATAL",
            Self::Panic => "PANIC",
            Self::Warning => "WARNING",
            Self::Notice => "NOTICE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Log => "LOG",
        }
    }
}

/// As the SQL standard defines them: two characters for the class, three for
/// the subclass. `RAISE ... USING ERRCODE` and extensions may raise any five
/// characters they like.
///
/// Generated from the list PostgreSQL generates its own `errcodes.h` from:
/// <https://github.com/postgres/postgres/blob/master/src/backend/utils/errcodes.txt>
/// Variant names are that file's `ERRCODE_` names, which are the only ones
/// unique per code; the condition names beside them are shared between codes
/// and so cannot name a variant.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SqlState {
    SuccessfulCompletion,

    Warning,
    WarningDynamicResultSetsReturned,
    WarningImplicitZeroBitPadding,
    WarningNullValueEliminatedInSetFunction,
    WarningPrivilegeNotGranted,
    WarningPrivilegeNotRevoked,
    WarningStringDataRightTruncation,
    WarningDeprecatedFeature,

    NoData,
    NoAdditionalDynamicResultSetsReturned,

    SqlStatementNotYetComplete,

    ConnectionException,
    ConnectionDoesNotExist,
    ConnectionFailure,
    SqlclientUnableToEstablishSqlconnection,
    SqlserverRejectedEstablishmentOfSqlconnection,
    TransactionResolutionUnknown,
    ProtocolViolation,

    TriggeredActionException,

    FeatureNotSupported,

    InvalidTransactionInitiation,

    LocatorException,
    LEInvalidSpecification,

    InvalidGrantor,
    InvalidGrantOperation,

    InvalidRoleSpecification,

    DiagnosticsException,
    StackedDiagnosticsAccessedWithoutActiveHandler,

    InvalidArgumentForXquery,

    CaseNotFound,

    CardinalityViolation,

    DataException,
    ArraySubscriptError,
    CharacterNotInRepertoire,
    DatetimeValueOutOfRange,
    DivisionByZero,
    ErrorInAssignment,
    EscapeCharacterConflict,
    IndicatorOverflow,
    IntervalFieldOverflow,
    InvalidArgumentForLog,
    InvalidArgumentForNtile,
    InvalidArgumentForNthValue,
    InvalidArgumentForPowerFunction,
    InvalidArgumentForWidthBucketFunction,
    InvalidCharacterValueForCast,
    InvalidDatetimeFormat,
    InvalidEscapeCharacter,
    InvalidEscapeOctet,
    InvalidEscapeSequence,
    NonstandardUseOfEscapeCharacter,
    InvalidIndicatorParameterValue,
    InvalidParameterValue,
    InvalidPrecedingOrFollowingSize,
    InvalidRegularExpression,
    InvalidRowCountInLimitClause,
    InvalidRowCountInResultOffsetClause,
    InvalidTablesampleArgument,
    InvalidTablesampleRepeat,
    InvalidTimeZoneDisplacementValue,
    InvalidUseOfEscapeCharacter,
    MostSpecificTypeMismatch,
    NullValueNotAllowed,
    NullValueNoIndicatorParameter,
    NumericValueOutOfRange,
    SequenceGeneratorLimitExceeded,
    StringDataLengthMismatch,
    StringDataRightTruncation,
    SubstringError,
    TrimError,
    UnterminatedCString,
    ZeroLengthCharacterString,
    FloatingPointException,
    InvalidTextRepresentation,
    InvalidBinaryRepresentation,
    BadCopyFileFormat,
    UntranslatableCharacter,
    NotAnXmlDocument,
    InvalidXmlDocument,
    InvalidXmlContent,
    InvalidXmlComment,
    InvalidXmlProcessingInstruction,
    DuplicateJsonObjectKeyValue,
    InvalidArgumentForSqlJsonDatetimeFunction,
    InvalidJsonText,
    InvalidSqlJsonSubscript,
    MoreThanOneSqlJsonItem,
    NoSqlJsonItem,
    NonNumericSqlJsonItem,
    NonUniqueKeysInAJsonObject,
    SingletonSqlJsonItemRequired,
    SqlJsonArrayNotFound,
    SqlJsonMemberNotFound,
    SqlJsonNumberNotFound,
    SqlJsonObjectNotFound,
    TooManyJsonArrayElements,
    TooManyJsonObjectMembers,
    SqlJsonScalarRequired,
    SqlJsonItemCannotBeCastToTargetType,

    IntegrityConstraintViolation,
    RestrictViolation,
    NotNullViolation,
    ForeignKeyViolation,
    UniqueViolation,
    CheckViolation,
    ExclusionViolation,

    InvalidCursorState,

    InvalidTransactionState,
    ActiveSqlTransaction,
    BranchTransactionAlreadyActive,
    HeldCursorRequiresSameIsolationLevel,
    InappropriateAccessModeForBranchTransaction,
    InappropriateIsolationLevelForBranchTransaction,
    NoActiveSqlTransactionForBranchTransaction,
    ReadOnlySqlTransaction,
    SchemaAndDataStatementMixingNotSupported,
    NoActiveSqlTransaction,
    InFailedSqlTransaction,
    IdleInTransactionSessionTimeout,
    TransactionTimeout,

    InvalidSqlStatementName,

    TriggeredDataChangeViolation,

    InvalidAuthorizationSpecification,
    InvalidPassword,

    DependentPrivilegeDescriptorsStillExist,
    DependentObjectsStillExist,

    InvalidTransactionTermination,

    SqlRoutineException,
    SREFunctionExecutedNoReturnStatement,
    SREModifyingSqlDataNotPermitted,
    SREProhibitedSqlStatementAttempted,
    SREReadingSqlDataNotPermitted,

    InvalidCursorName,

    ExternalRoutineException,
    EREContainingSqlNotPermitted,
    EREModifyingSqlDataNotPermitted,
    EREProhibitedSqlStatementAttempted,
    EREReadingSqlDataNotPermitted,

    ExternalRoutineInvocationException,
    ERIEInvalidSqlstateReturned,
    ERIENullValueNotAllowed,
    ERIETriggerProtocolViolated,
    ERIESrfProtocolViolated,
    ERIEEventTriggerProtocolViolated,

    SavepointException,
    SEInvalidSpecification,

    InvalidCatalogName,

    InvalidSchemaName,

    TransactionRollback,
    TRIntegrityConstraintViolation,
    TRSerializationFailure,
    TRStatementCompletionUnknown,
    TRDeadlockDetected,

    SyntaxErrorOrAccessRuleViolation,
    SyntaxError,
    InsufficientPrivilege,
    CannotCoerce,
    GroupingError,
    WindowingError,
    InvalidRecursion,
    InvalidForeignKey,
    InvalidName,
    NameTooLong,
    ReservedName,
    DatatypeMismatch,
    IndeterminateDatatype,
    CollationMismatch,
    IndeterminateCollation,
    WrongObjectType,
    GeneratedAlways,
    UndefinedColumn,
    UndefinedFunction,
    UndefinedTable,
    UndefinedParameter,
    UndefinedObject,
    DuplicateColumn,
    DuplicateCursor,
    DuplicateDatabase,
    DuplicateFunction,
    DuplicatePstatement,
    DuplicateSchema,
    DuplicateTable,
    DuplicateAlias,
    DuplicateObject,
    AmbiguousColumn,
    AmbiguousFunction,
    AmbiguousParameter,
    AmbiguousAlias,
    InvalidColumnReference,
    InvalidColumnDefinition,
    InvalidCursorDefinition,
    InvalidDatabaseDefinition,
    InvalidFunctionDefinition,
    InvalidPstatementDefinition,
    InvalidSchemaDefinition,
    InvalidTableDefinition,
    InvalidObjectDefinition,

    WithCheckOptionViolation,

    InsufficientResources,
    DiskFull,
    OutOfMemory,
    TooManyConnections,
    ConfigurationLimitExceeded,

    ProgramLimitExceeded,
    StatementTooComplex,
    TooManyColumns,
    TooManyArguments,

    ObjectNotInPrerequisiteState,
    ObjectInUse,
    CantChangeRuntimeParam,
    LockNotAvailable,
    UnsafeNewEnumValueUsage,

    OperatorIntervention,
    QueryCanceled,
    AdminShutdown,
    CrashShutdown,
    CannotConnectNow,
    DatabaseDropped,
    IdleSessionTimeout,

    SystemError,
    IoError,
    UndefinedFile,
    DuplicateFile,
    FileNameTooLong,

    ConfigFileError,
    LockFileExists,

    FdwError,
    FdwColumnNameNotFound,
    FdwDynamicParameterValueNeeded,
    FdwFunctionSequenceError,
    FdwInconsistentDescriptorInformation,
    FdwInvalidAttributeValue,
    FdwInvalidColumnName,
    FdwInvalidColumnNumber,
    FdwInvalidDataType,
    FdwInvalidDataTypeDescriptors,
    FdwInvalidDescriptorFieldIdentifier,
    FdwInvalidHandle,
    FdwInvalidOptionIndex,
    FdwInvalidOptionName,
    FdwInvalidStringLengthOrBufferLength,
    FdwInvalidStringFormat,
    FdwInvalidUseOfNullPointer,
    FdwTooManyHandles,
    FdwOutOfMemory,
    FdwNoSchemas,
    FdwOptionNameNotFound,
    FdwReplyHandle,
    FdwSchemaNotFound,
    FdwTableNotFound,
    FdwUnableToCreateExecution,
    FdwUnableToCreateReply,
    FdwUnableToEstablishConnection,

    PlpgsqlError,
    RaiseException,
    NoDataFound,
    TooManyRows,
    AssertFailure,

    InternalError,
    DataCorrupted,
    IndexCorrupted,

    Other(UnknownSqlState),
}

/// PostgreSQL gives these codes a second `ERRCODE_` name because it wants
/// one vocabulary for its own source where the SQL standard gave it two.
#[expect(
    non_upper_case_globals,
    reason = "these stand in for variants, and are matched like them"
)]
impl SqlState {
    pub const ArrayElementError: Self = Self::ArraySubscriptError;
    /// The only pair here where the variant is not the name upstream
    /// documents. `timestamp.c` raises the value-out-of-range name roughly
    /// eight times as often, so that is the one worth reading in a backtrace.
    pub const DatetimeFieldOverflow: Self = Self::DatetimeValueOutOfRange;
    pub const UndefinedCursor: Self = Self::InvalidCursorName;
    pub const UndefinedDatabase: Self = Self::InvalidCatalogName;
    pub const UndefinedPstatement: Self = Self::InvalidSqlStatementName;
    pub const UndefinedSchema: Self = Self::InvalidSchemaName;
}

impl SqlState {
    pub fn new(value: &str) -> Result<Self, DecodeError> {
        let invalid = || DecodeError::InvalidSqlState {
            value: value.to_owned(),
        };

        let bytes: [u8; 5] = value.as_bytes().try_into().map_err(|_| invalid())?;

        if !bytes
            .iter()
            .all(|byte| byte.is_ascii_digit() || byte.is_ascii_uppercase())
        {
            return Err(invalid());
        }

        Ok(match &bytes {
            b"00000" => Self::SuccessfulCompletion,
            b"01000" => Self::Warning,
            b"0100C" => Self::WarningDynamicResultSetsReturned,
            b"01008" => Self::WarningImplicitZeroBitPadding,
            b"01003" => Self::WarningNullValueEliminatedInSetFunction,
            b"01007" => Self::WarningPrivilegeNotGranted,
            b"01006" => Self::WarningPrivilegeNotRevoked,
            b"01004" => Self::WarningStringDataRightTruncation,
            b"01P01" => Self::WarningDeprecatedFeature,
            b"02000" => Self::NoData,
            b"02001" => Self::NoAdditionalDynamicResultSetsReturned,
            b"03000" => Self::SqlStatementNotYetComplete,
            b"08000" => Self::ConnectionException,
            b"08003" => Self::ConnectionDoesNotExist,
            b"08006" => Self::ConnectionFailure,
            b"08001" => Self::SqlclientUnableToEstablishSqlconnection,
            b"08004" => Self::SqlserverRejectedEstablishmentOfSqlconnection,
            b"08007" => Self::TransactionResolutionUnknown,
            b"08P01" => Self::ProtocolViolation,
            b"09000" => Self::TriggeredActionException,
            b"0A000" => Self::FeatureNotSupported,
            b"0B000" => Self::InvalidTransactionInitiation,
            b"0F000" => Self::LocatorException,
            b"0F001" => Self::LEInvalidSpecification,
            b"0L000" => Self::InvalidGrantor,
            b"0LP01" => Self::InvalidGrantOperation,
            b"0P000" => Self::InvalidRoleSpecification,
            b"0Z000" => Self::DiagnosticsException,
            b"0Z002" => Self::StackedDiagnosticsAccessedWithoutActiveHandler,
            b"10608" => Self::InvalidArgumentForXquery,
            b"20000" => Self::CaseNotFound,
            b"21000" => Self::CardinalityViolation,
            b"22000" => Self::DataException,
            b"2202E" => Self::ArraySubscriptError,
            b"22021" => Self::CharacterNotInRepertoire,
            b"22008" => Self::DatetimeValueOutOfRange,
            b"22012" => Self::DivisionByZero,
            b"22005" => Self::ErrorInAssignment,
            b"2200B" => Self::EscapeCharacterConflict,
            b"22022" => Self::IndicatorOverflow,
            b"22015" => Self::IntervalFieldOverflow,
            b"2201E" => Self::InvalidArgumentForLog,
            b"22014" => Self::InvalidArgumentForNtile,
            b"22016" => Self::InvalidArgumentForNthValue,
            b"2201F" => Self::InvalidArgumentForPowerFunction,
            b"2201G" => Self::InvalidArgumentForWidthBucketFunction,
            b"22018" => Self::InvalidCharacterValueForCast,
            b"22007" => Self::InvalidDatetimeFormat,
            b"22019" => Self::InvalidEscapeCharacter,
            b"2200D" => Self::InvalidEscapeOctet,
            b"22025" => Self::InvalidEscapeSequence,
            b"22P06" => Self::NonstandardUseOfEscapeCharacter,
            b"22010" => Self::InvalidIndicatorParameterValue,
            b"22023" => Self::InvalidParameterValue,
            b"22013" => Self::InvalidPrecedingOrFollowingSize,
            b"2201B" => Self::InvalidRegularExpression,
            b"2201W" => Self::InvalidRowCountInLimitClause,
            b"2201X" => Self::InvalidRowCountInResultOffsetClause,
            b"2202H" => Self::InvalidTablesampleArgument,
            b"2202G" => Self::InvalidTablesampleRepeat,
            b"22009" => Self::InvalidTimeZoneDisplacementValue,
            b"2200C" => Self::InvalidUseOfEscapeCharacter,
            b"2200G" => Self::MostSpecificTypeMismatch,
            b"22004" => Self::NullValueNotAllowed,
            b"22002" => Self::NullValueNoIndicatorParameter,
            b"22003" => Self::NumericValueOutOfRange,
            b"2200H" => Self::SequenceGeneratorLimitExceeded,
            b"22026" => Self::StringDataLengthMismatch,
            b"22001" => Self::StringDataRightTruncation,
            b"22011" => Self::SubstringError,
            b"22027" => Self::TrimError,
            b"22024" => Self::UnterminatedCString,
            b"2200F" => Self::ZeroLengthCharacterString,
            b"22P01" => Self::FloatingPointException,
            b"22P02" => Self::InvalidTextRepresentation,
            b"22P03" => Self::InvalidBinaryRepresentation,
            b"22P04" => Self::BadCopyFileFormat,
            b"22P05" => Self::UntranslatableCharacter,
            b"2200L" => Self::NotAnXmlDocument,
            b"2200M" => Self::InvalidXmlDocument,
            b"2200N" => Self::InvalidXmlContent,
            b"2200S" => Self::InvalidXmlComment,
            b"2200T" => Self::InvalidXmlProcessingInstruction,
            b"22030" => Self::DuplicateJsonObjectKeyValue,
            b"22031" => Self::InvalidArgumentForSqlJsonDatetimeFunction,
            b"22032" => Self::InvalidJsonText,
            b"22033" => Self::InvalidSqlJsonSubscript,
            b"22034" => Self::MoreThanOneSqlJsonItem,
            b"22035" => Self::NoSqlJsonItem,
            b"22036" => Self::NonNumericSqlJsonItem,
            b"22037" => Self::NonUniqueKeysInAJsonObject,
            b"22038" => Self::SingletonSqlJsonItemRequired,
            b"22039" => Self::SqlJsonArrayNotFound,
            b"2203A" => Self::SqlJsonMemberNotFound,
            b"2203B" => Self::SqlJsonNumberNotFound,
            b"2203C" => Self::SqlJsonObjectNotFound,
            b"2203D" => Self::TooManyJsonArrayElements,
            b"2203E" => Self::TooManyJsonObjectMembers,
            b"2203F" => Self::SqlJsonScalarRequired,
            b"2203G" => Self::SqlJsonItemCannotBeCastToTargetType,
            b"23000" => Self::IntegrityConstraintViolation,
            b"23001" => Self::RestrictViolation,
            b"23502" => Self::NotNullViolation,
            b"23503" => Self::ForeignKeyViolation,
            b"23505" => Self::UniqueViolation,
            b"23514" => Self::CheckViolation,
            b"23P01" => Self::ExclusionViolation,
            b"24000" => Self::InvalidCursorState,
            b"25000" => Self::InvalidTransactionState,
            b"25001" => Self::ActiveSqlTransaction,
            b"25002" => Self::BranchTransactionAlreadyActive,
            b"25008" => Self::HeldCursorRequiresSameIsolationLevel,
            b"25003" => Self::InappropriateAccessModeForBranchTransaction,
            b"25004" => Self::InappropriateIsolationLevelForBranchTransaction,
            b"25005" => Self::NoActiveSqlTransactionForBranchTransaction,
            b"25006" => Self::ReadOnlySqlTransaction,
            b"25007" => Self::SchemaAndDataStatementMixingNotSupported,
            b"25P01" => Self::NoActiveSqlTransaction,
            b"25P02" => Self::InFailedSqlTransaction,
            b"25P03" => Self::IdleInTransactionSessionTimeout,
            b"25P04" => Self::TransactionTimeout,
            b"26000" => Self::InvalidSqlStatementName,
            b"27000" => Self::TriggeredDataChangeViolation,
            b"28000" => Self::InvalidAuthorizationSpecification,
            b"28P01" => Self::InvalidPassword,
            b"2B000" => Self::DependentPrivilegeDescriptorsStillExist,
            b"2BP01" => Self::DependentObjectsStillExist,
            b"2D000" => Self::InvalidTransactionTermination,
            b"2F000" => Self::SqlRoutineException,
            b"2F005" => Self::SREFunctionExecutedNoReturnStatement,
            b"2F002" => Self::SREModifyingSqlDataNotPermitted,
            b"2F003" => Self::SREProhibitedSqlStatementAttempted,
            b"2F004" => Self::SREReadingSqlDataNotPermitted,
            b"34000" => Self::InvalidCursorName,
            b"38000" => Self::ExternalRoutineException,
            b"38001" => Self::EREContainingSqlNotPermitted,
            b"38002" => Self::EREModifyingSqlDataNotPermitted,
            b"38003" => Self::EREProhibitedSqlStatementAttempted,
            b"38004" => Self::EREReadingSqlDataNotPermitted,
            b"39000" => Self::ExternalRoutineInvocationException,
            b"39001" => Self::ERIEInvalidSqlstateReturned,
            b"39004" => Self::ERIENullValueNotAllowed,
            b"39P01" => Self::ERIETriggerProtocolViolated,
            b"39P02" => Self::ERIESrfProtocolViolated,
            b"39P03" => Self::ERIEEventTriggerProtocolViolated,
            b"3B000" => Self::SavepointException,
            b"3B001" => Self::SEInvalidSpecification,
            b"3D000" => Self::InvalidCatalogName,
            b"3F000" => Self::InvalidSchemaName,
            b"40000" => Self::TransactionRollback,
            b"40002" => Self::TRIntegrityConstraintViolation,
            b"40001" => Self::TRSerializationFailure,
            b"40003" => Self::TRStatementCompletionUnknown,
            b"40P01" => Self::TRDeadlockDetected,
            b"42000" => Self::SyntaxErrorOrAccessRuleViolation,
            b"42601" => Self::SyntaxError,
            b"42501" => Self::InsufficientPrivilege,
            b"42846" => Self::CannotCoerce,
            b"42803" => Self::GroupingError,
            b"42P20" => Self::WindowingError,
            b"42P19" => Self::InvalidRecursion,
            b"42830" => Self::InvalidForeignKey,
            b"42602" => Self::InvalidName,
            b"42622" => Self::NameTooLong,
            b"42939" => Self::ReservedName,
            b"42804" => Self::DatatypeMismatch,
            b"42P18" => Self::IndeterminateDatatype,
            b"42P21" => Self::CollationMismatch,
            b"42P22" => Self::IndeterminateCollation,
            b"42809" => Self::WrongObjectType,
            b"428C9" => Self::GeneratedAlways,
            b"42703" => Self::UndefinedColumn,
            b"42883" => Self::UndefinedFunction,
            b"42P01" => Self::UndefinedTable,
            b"42P02" => Self::UndefinedParameter,
            b"42704" => Self::UndefinedObject,
            b"42701" => Self::DuplicateColumn,
            b"42P03" => Self::DuplicateCursor,
            b"42P04" => Self::DuplicateDatabase,
            b"42723" => Self::DuplicateFunction,
            b"42P05" => Self::DuplicatePstatement,
            b"42P06" => Self::DuplicateSchema,
            b"42P07" => Self::DuplicateTable,
            b"42712" => Self::DuplicateAlias,
            b"42710" => Self::DuplicateObject,
            b"42702" => Self::AmbiguousColumn,
            b"42725" => Self::AmbiguousFunction,
            b"42P08" => Self::AmbiguousParameter,
            b"42P09" => Self::AmbiguousAlias,
            b"42P10" => Self::InvalidColumnReference,
            b"42611" => Self::InvalidColumnDefinition,
            b"42P11" => Self::InvalidCursorDefinition,
            b"42P12" => Self::InvalidDatabaseDefinition,
            b"42P13" => Self::InvalidFunctionDefinition,
            b"42P14" => Self::InvalidPstatementDefinition,
            b"42P15" => Self::InvalidSchemaDefinition,
            b"42P16" => Self::InvalidTableDefinition,
            b"42P17" => Self::InvalidObjectDefinition,
            b"44000" => Self::WithCheckOptionViolation,
            b"53000" => Self::InsufficientResources,
            b"53100" => Self::DiskFull,
            b"53200" => Self::OutOfMemory,
            b"53300" => Self::TooManyConnections,
            b"53400" => Self::ConfigurationLimitExceeded,
            b"54000" => Self::ProgramLimitExceeded,
            b"54001" => Self::StatementTooComplex,
            b"54011" => Self::TooManyColumns,
            b"54023" => Self::TooManyArguments,
            b"55000" => Self::ObjectNotInPrerequisiteState,
            b"55006" => Self::ObjectInUse,
            b"55P02" => Self::CantChangeRuntimeParam,
            b"55P03" => Self::LockNotAvailable,
            b"55P04" => Self::UnsafeNewEnumValueUsage,
            b"57000" => Self::OperatorIntervention,
            b"57014" => Self::QueryCanceled,
            b"57P01" => Self::AdminShutdown,
            b"57P02" => Self::CrashShutdown,
            b"57P03" => Self::CannotConnectNow,
            b"57P04" => Self::DatabaseDropped,
            b"57P05" => Self::IdleSessionTimeout,
            b"58000" => Self::SystemError,
            b"58030" => Self::IoError,
            b"58P01" => Self::UndefinedFile,
            b"58P02" => Self::DuplicateFile,
            b"58P03" => Self::FileNameTooLong,
            b"F0000" => Self::ConfigFileError,
            b"F0001" => Self::LockFileExists,
            b"HV000" => Self::FdwError,
            b"HV005" => Self::FdwColumnNameNotFound,
            b"HV002" => Self::FdwDynamicParameterValueNeeded,
            b"HV010" => Self::FdwFunctionSequenceError,
            b"HV021" => Self::FdwInconsistentDescriptorInformation,
            b"HV024" => Self::FdwInvalidAttributeValue,
            b"HV007" => Self::FdwInvalidColumnName,
            b"HV008" => Self::FdwInvalidColumnNumber,
            b"HV004" => Self::FdwInvalidDataType,
            b"HV006" => Self::FdwInvalidDataTypeDescriptors,
            b"HV091" => Self::FdwInvalidDescriptorFieldIdentifier,
            b"HV00B" => Self::FdwInvalidHandle,
            b"HV00C" => Self::FdwInvalidOptionIndex,
            b"HV00D" => Self::FdwInvalidOptionName,
            b"HV090" => Self::FdwInvalidStringLengthOrBufferLength,
            b"HV00A" => Self::FdwInvalidStringFormat,
            b"HV009" => Self::FdwInvalidUseOfNullPointer,
            b"HV014" => Self::FdwTooManyHandles,
            b"HV001" => Self::FdwOutOfMemory,
            b"HV00P" => Self::FdwNoSchemas,
            b"HV00J" => Self::FdwOptionNameNotFound,
            b"HV00K" => Self::FdwReplyHandle,
            b"HV00Q" => Self::FdwSchemaNotFound,
            b"HV00R" => Self::FdwTableNotFound,
            b"HV00L" => Self::FdwUnableToCreateExecution,
            b"HV00M" => Self::FdwUnableToCreateReply,
            b"HV00N" => Self::FdwUnableToEstablishConnection,
            b"P0000" => Self::PlpgsqlError,
            b"P0001" => Self::RaiseException,
            b"P0002" => Self::NoDataFound,
            b"P0003" => Self::TooManyRows,
            b"P0004" => Self::AssertFailure,
            b"XX000" => Self::InternalError,
            b"XX001" => Self::DataCorrupted,
            b"XX002" => Self::IndexCorrupted,
            _ => Self::Other(UnknownSqlState(bytes)),
        })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::SuccessfulCompletion => "00000",
            Self::Warning => "01000",
            Self::WarningDynamicResultSetsReturned => "0100C",
            Self::WarningImplicitZeroBitPadding => "01008",
            Self::WarningNullValueEliminatedInSetFunction => "01003",
            Self::WarningPrivilegeNotGranted => "01007",
            Self::WarningPrivilegeNotRevoked => "01006",
            Self::WarningStringDataRightTruncation => "01004",
            Self::WarningDeprecatedFeature => "01P01",
            Self::NoData => "02000",
            Self::NoAdditionalDynamicResultSetsReturned => "02001",
            Self::SqlStatementNotYetComplete => "03000",
            Self::ConnectionException => "08000",
            Self::ConnectionDoesNotExist => "08003",
            Self::ConnectionFailure => "08006",
            Self::SqlclientUnableToEstablishSqlconnection => "08001",
            Self::SqlserverRejectedEstablishmentOfSqlconnection => "08004",
            Self::TransactionResolutionUnknown => "08007",
            Self::ProtocolViolation => "08P01",
            Self::TriggeredActionException => "09000",
            Self::FeatureNotSupported => "0A000",
            Self::InvalidTransactionInitiation => "0B000",
            Self::LocatorException => "0F000",
            Self::LEInvalidSpecification => "0F001",
            Self::InvalidGrantor => "0L000",
            Self::InvalidGrantOperation => "0LP01",
            Self::InvalidRoleSpecification => "0P000",
            Self::DiagnosticsException => "0Z000",
            Self::StackedDiagnosticsAccessedWithoutActiveHandler => "0Z002",
            Self::InvalidArgumentForXquery => "10608",
            Self::CaseNotFound => "20000",
            Self::CardinalityViolation => "21000",
            Self::DataException => "22000",
            Self::ArraySubscriptError => "2202E",
            Self::CharacterNotInRepertoire => "22021",
            Self::DatetimeValueOutOfRange => "22008",
            Self::DivisionByZero => "22012",
            Self::ErrorInAssignment => "22005",
            Self::EscapeCharacterConflict => "2200B",
            Self::IndicatorOverflow => "22022",
            Self::IntervalFieldOverflow => "22015",
            Self::InvalidArgumentForLog => "2201E",
            Self::InvalidArgumentForNtile => "22014",
            Self::InvalidArgumentForNthValue => "22016",
            Self::InvalidArgumentForPowerFunction => "2201F",
            Self::InvalidArgumentForWidthBucketFunction => "2201G",
            Self::InvalidCharacterValueForCast => "22018",
            Self::InvalidDatetimeFormat => "22007",
            Self::InvalidEscapeCharacter => "22019",
            Self::InvalidEscapeOctet => "2200D",
            Self::InvalidEscapeSequence => "22025",
            Self::NonstandardUseOfEscapeCharacter => "22P06",
            Self::InvalidIndicatorParameterValue => "22010",
            Self::InvalidParameterValue => "22023",
            Self::InvalidPrecedingOrFollowingSize => "22013",
            Self::InvalidRegularExpression => "2201B",
            Self::InvalidRowCountInLimitClause => "2201W",
            Self::InvalidRowCountInResultOffsetClause => "2201X",
            Self::InvalidTablesampleArgument => "2202H",
            Self::InvalidTablesampleRepeat => "2202G",
            Self::InvalidTimeZoneDisplacementValue => "22009",
            Self::InvalidUseOfEscapeCharacter => "2200C",
            Self::MostSpecificTypeMismatch => "2200G",
            Self::NullValueNotAllowed => "22004",
            Self::NullValueNoIndicatorParameter => "22002",
            Self::NumericValueOutOfRange => "22003",
            Self::SequenceGeneratorLimitExceeded => "2200H",
            Self::StringDataLengthMismatch => "22026",
            Self::StringDataRightTruncation => "22001",
            Self::SubstringError => "22011",
            Self::TrimError => "22027",
            Self::UnterminatedCString => "22024",
            Self::ZeroLengthCharacterString => "2200F",
            Self::FloatingPointException => "22P01",
            Self::InvalidTextRepresentation => "22P02",
            Self::InvalidBinaryRepresentation => "22P03",
            Self::BadCopyFileFormat => "22P04",
            Self::UntranslatableCharacter => "22P05",
            Self::NotAnXmlDocument => "2200L",
            Self::InvalidXmlDocument => "2200M",
            Self::InvalidXmlContent => "2200N",
            Self::InvalidXmlComment => "2200S",
            Self::InvalidXmlProcessingInstruction => "2200T",
            Self::DuplicateJsonObjectKeyValue => "22030",
            Self::InvalidArgumentForSqlJsonDatetimeFunction => "22031",
            Self::InvalidJsonText => "22032",
            Self::InvalidSqlJsonSubscript => "22033",
            Self::MoreThanOneSqlJsonItem => "22034",
            Self::NoSqlJsonItem => "22035",
            Self::NonNumericSqlJsonItem => "22036",
            Self::NonUniqueKeysInAJsonObject => "22037",
            Self::SingletonSqlJsonItemRequired => "22038",
            Self::SqlJsonArrayNotFound => "22039",
            Self::SqlJsonMemberNotFound => "2203A",
            Self::SqlJsonNumberNotFound => "2203B",
            Self::SqlJsonObjectNotFound => "2203C",
            Self::TooManyJsonArrayElements => "2203D",
            Self::TooManyJsonObjectMembers => "2203E",
            Self::SqlJsonScalarRequired => "2203F",
            Self::SqlJsonItemCannotBeCastToTargetType => "2203G",
            Self::IntegrityConstraintViolation => "23000",
            Self::RestrictViolation => "23001",
            Self::NotNullViolation => "23502",
            Self::ForeignKeyViolation => "23503",
            Self::UniqueViolation => "23505",
            Self::CheckViolation => "23514",
            Self::ExclusionViolation => "23P01",
            Self::InvalidCursorState => "24000",
            Self::InvalidTransactionState => "25000",
            Self::ActiveSqlTransaction => "25001",
            Self::BranchTransactionAlreadyActive => "25002",
            Self::HeldCursorRequiresSameIsolationLevel => "25008",
            Self::InappropriateAccessModeForBranchTransaction => "25003",
            Self::InappropriateIsolationLevelForBranchTransaction => "25004",
            Self::NoActiveSqlTransactionForBranchTransaction => "25005",
            Self::ReadOnlySqlTransaction => "25006",
            Self::SchemaAndDataStatementMixingNotSupported => "25007",
            Self::NoActiveSqlTransaction => "25P01",
            Self::InFailedSqlTransaction => "25P02",
            Self::IdleInTransactionSessionTimeout => "25P03",
            Self::TransactionTimeout => "25P04",
            Self::InvalidSqlStatementName => "26000",
            Self::TriggeredDataChangeViolation => "27000",
            Self::InvalidAuthorizationSpecification => "28000",
            Self::InvalidPassword => "28P01",
            Self::DependentPrivilegeDescriptorsStillExist => "2B000",
            Self::DependentObjectsStillExist => "2BP01",
            Self::InvalidTransactionTermination => "2D000",
            Self::SqlRoutineException => "2F000",
            Self::SREFunctionExecutedNoReturnStatement => "2F005",
            Self::SREModifyingSqlDataNotPermitted => "2F002",
            Self::SREProhibitedSqlStatementAttempted => "2F003",
            Self::SREReadingSqlDataNotPermitted => "2F004",
            Self::InvalidCursorName => "34000",
            Self::ExternalRoutineException => "38000",
            Self::EREContainingSqlNotPermitted => "38001",
            Self::EREModifyingSqlDataNotPermitted => "38002",
            Self::EREProhibitedSqlStatementAttempted => "38003",
            Self::EREReadingSqlDataNotPermitted => "38004",
            Self::ExternalRoutineInvocationException => "39000",
            Self::ERIEInvalidSqlstateReturned => "39001",
            Self::ERIENullValueNotAllowed => "39004",
            Self::ERIETriggerProtocolViolated => "39P01",
            Self::ERIESrfProtocolViolated => "39P02",
            Self::ERIEEventTriggerProtocolViolated => "39P03",
            Self::SavepointException => "3B000",
            Self::SEInvalidSpecification => "3B001",
            Self::InvalidCatalogName => "3D000",
            Self::InvalidSchemaName => "3F000",
            Self::TransactionRollback => "40000",
            Self::TRIntegrityConstraintViolation => "40002",
            Self::TRSerializationFailure => "40001",
            Self::TRStatementCompletionUnknown => "40003",
            Self::TRDeadlockDetected => "40P01",
            Self::SyntaxErrorOrAccessRuleViolation => "42000",
            Self::SyntaxError => "42601",
            Self::InsufficientPrivilege => "42501",
            Self::CannotCoerce => "42846",
            Self::GroupingError => "42803",
            Self::WindowingError => "42P20",
            Self::InvalidRecursion => "42P19",
            Self::InvalidForeignKey => "42830",
            Self::InvalidName => "42602",
            Self::NameTooLong => "42622",
            Self::ReservedName => "42939",
            Self::DatatypeMismatch => "42804",
            Self::IndeterminateDatatype => "42P18",
            Self::CollationMismatch => "42P21",
            Self::IndeterminateCollation => "42P22",
            Self::WrongObjectType => "42809",
            Self::GeneratedAlways => "428C9",
            Self::UndefinedColumn => "42703",
            Self::UndefinedFunction => "42883",
            Self::UndefinedTable => "42P01",
            Self::UndefinedParameter => "42P02",
            Self::UndefinedObject => "42704",
            Self::DuplicateColumn => "42701",
            Self::DuplicateCursor => "42P03",
            Self::DuplicateDatabase => "42P04",
            Self::DuplicateFunction => "42723",
            Self::DuplicatePstatement => "42P05",
            Self::DuplicateSchema => "42P06",
            Self::DuplicateTable => "42P07",
            Self::DuplicateAlias => "42712",
            Self::DuplicateObject => "42710",
            Self::AmbiguousColumn => "42702",
            Self::AmbiguousFunction => "42725",
            Self::AmbiguousParameter => "42P08",
            Self::AmbiguousAlias => "42P09",
            Self::InvalidColumnReference => "42P10",
            Self::InvalidColumnDefinition => "42611",
            Self::InvalidCursorDefinition => "42P11",
            Self::InvalidDatabaseDefinition => "42P12",
            Self::InvalidFunctionDefinition => "42P13",
            Self::InvalidPstatementDefinition => "42P14",
            Self::InvalidSchemaDefinition => "42P15",
            Self::InvalidTableDefinition => "42P16",
            Self::InvalidObjectDefinition => "42P17",
            Self::WithCheckOptionViolation => "44000",
            Self::InsufficientResources => "53000",
            Self::DiskFull => "53100",
            Self::OutOfMemory => "53200",
            Self::TooManyConnections => "53300",
            Self::ConfigurationLimitExceeded => "53400",
            Self::ProgramLimitExceeded => "54000",
            Self::StatementTooComplex => "54001",
            Self::TooManyColumns => "54011",
            Self::TooManyArguments => "54023",
            Self::ObjectNotInPrerequisiteState => "55000",
            Self::ObjectInUse => "55006",
            Self::CantChangeRuntimeParam => "55P02",
            Self::LockNotAvailable => "55P03",
            Self::UnsafeNewEnumValueUsage => "55P04",
            Self::OperatorIntervention => "57000",
            Self::QueryCanceled => "57014",
            Self::AdminShutdown => "57P01",
            Self::CrashShutdown => "57P02",
            Self::CannotConnectNow => "57P03",
            Self::DatabaseDropped => "57P04",
            Self::IdleSessionTimeout => "57P05",
            Self::SystemError => "58000",
            Self::IoError => "58030",
            Self::UndefinedFile => "58P01",
            Self::DuplicateFile => "58P02",
            Self::FileNameTooLong => "58P03",
            Self::ConfigFileError => "F0000",
            Self::LockFileExists => "F0001",
            Self::FdwError => "HV000",
            Self::FdwColumnNameNotFound => "HV005",
            Self::FdwDynamicParameterValueNeeded => "HV002",
            Self::FdwFunctionSequenceError => "HV010",
            Self::FdwInconsistentDescriptorInformation => "HV021",
            Self::FdwInvalidAttributeValue => "HV024",
            Self::FdwInvalidColumnName => "HV007",
            Self::FdwInvalidColumnNumber => "HV008",
            Self::FdwInvalidDataType => "HV004",
            Self::FdwInvalidDataTypeDescriptors => "HV006",
            Self::FdwInvalidDescriptorFieldIdentifier => "HV091",
            Self::FdwInvalidHandle => "HV00B",
            Self::FdwInvalidOptionIndex => "HV00C",
            Self::FdwInvalidOptionName => "HV00D",
            Self::FdwInvalidStringLengthOrBufferLength => "HV090",
            Self::FdwInvalidStringFormat => "HV00A",
            Self::FdwInvalidUseOfNullPointer => "HV009",
            Self::FdwTooManyHandles => "HV014",
            Self::FdwOutOfMemory => "HV001",
            Self::FdwNoSchemas => "HV00P",
            Self::FdwOptionNameNotFound => "HV00J",
            Self::FdwReplyHandle => "HV00K",
            Self::FdwSchemaNotFound => "HV00Q",
            Self::FdwTableNotFound => "HV00R",
            Self::FdwUnableToCreateExecution => "HV00L",
            Self::FdwUnableToCreateReply => "HV00M",
            Self::FdwUnableToEstablishConnection => "HV00N",
            Self::PlpgsqlError => "P0000",
            Self::RaiseException => "P0001",
            Self::NoDataFound => "P0002",
            Self::TooManyRows => "P0003",
            Self::AssertFailure => "P0004",
            Self::InternalError => "XX000",
            Self::DataCorrupted => "XX001",
            Self::IndexCorrupted => "XX002",
            Self::Other(code) => code.as_str(),
        }
    }

    /// Worth matching on where a specific code is not. A server may raise a
    /// more precise subclass than an older one did for the same situation, but
    /// it stays inside its class.
    #[must_use]
    pub fn class(&self) -> Class {
        let bytes = self.as_str().as_bytes();

        Class::new([bytes[0], bytes[1]])
    }

    fn names(&self) -> &'static [&'static str] {
        match self {
            Self::SuccessfulCompletion => &["SuccessfulCompletion"],
            Self::Warning => &["Warning"],
            Self::WarningDynamicResultSetsReturned => &["WarningDynamicResultSetsReturned"],
            Self::WarningImplicitZeroBitPadding => &["WarningImplicitZeroBitPadding"],
            Self::WarningNullValueEliminatedInSetFunction => {
                &["WarningNullValueEliminatedInSetFunction"]
            }
            Self::WarningPrivilegeNotGranted => &["WarningPrivilegeNotGranted"],
            Self::WarningPrivilegeNotRevoked => &["WarningPrivilegeNotRevoked"],
            Self::WarningStringDataRightTruncation => &["WarningStringDataRightTruncation"],
            Self::WarningDeprecatedFeature => &["WarningDeprecatedFeature"],
            Self::NoData => &["NoData"],
            Self::NoAdditionalDynamicResultSetsReturned => {
                &["NoAdditionalDynamicResultSetsReturned"]
            }
            Self::SqlStatementNotYetComplete => &["SqlStatementNotYetComplete"],
            Self::ConnectionException => &["ConnectionException"],
            Self::ConnectionDoesNotExist => &["ConnectionDoesNotExist"],
            Self::ConnectionFailure => &["ConnectionFailure"],
            Self::SqlclientUnableToEstablishSqlconnection => {
                &["SqlclientUnableToEstablishSqlconnection"]
            }
            Self::SqlserverRejectedEstablishmentOfSqlconnection => {
                &["SqlserverRejectedEstablishmentOfSqlconnection"]
            }
            Self::TransactionResolutionUnknown => &["TransactionResolutionUnknown"],
            Self::ProtocolViolation => &["ProtocolViolation"],
            Self::TriggeredActionException => &["TriggeredActionException"],
            Self::FeatureNotSupported => &["FeatureNotSupported"],
            Self::InvalidTransactionInitiation => &["InvalidTransactionInitiation"],
            Self::LocatorException => &["LocatorException"],
            Self::LEInvalidSpecification => &["LEInvalidSpecification"],
            Self::InvalidGrantor => &["InvalidGrantor"],
            Self::InvalidGrantOperation => &["InvalidGrantOperation"],
            Self::InvalidRoleSpecification => &["InvalidRoleSpecification"],
            Self::DiagnosticsException => &["DiagnosticsException"],
            Self::StackedDiagnosticsAccessedWithoutActiveHandler => {
                &["StackedDiagnosticsAccessedWithoutActiveHandler"]
            }
            Self::InvalidArgumentForXquery => &["InvalidArgumentForXquery"],
            Self::CaseNotFound => &["CaseNotFound"],
            Self::CardinalityViolation => &["CardinalityViolation"],
            Self::DataException => &["DataException"],
            Self::ArraySubscriptError => &["ArraySubscriptError", "ArrayElementError"],
            Self::CharacterNotInRepertoire => &["CharacterNotInRepertoire"],
            Self::DatetimeValueOutOfRange => &["DatetimeValueOutOfRange", "DatetimeFieldOverflow"],
            Self::DivisionByZero => &["DivisionByZero"],
            Self::ErrorInAssignment => &["ErrorInAssignment"],
            Self::EscapeCharacterConflict => &["EscapeCharacterConflict"],
            Self::IndicatorOverflow => &["IndicatorOverflow"],
            Self::IntervalFieldOverflow => &["IntervalFieldOverflow"],
            Self::InvalidArgumentForLog => &["InvalidArgumentForLog"],
            Self::InvalidArgumentForNtile => &["InvalidArgumentForNtile"],
            Self::InvalidArgumentForNthValue => &["InvalidArgumentForNthValue"],
            Self::InvalidArgumentForPowerFunction => &["InvalidArgumentForPowerFunction"],
            Self::InvalidArgumentForWidthBucketFunction => {
                &["InvalidArgumentForWidthBucketFunction"]
            }
            Self::InvalidCharacterValueForCast => &["InvalidCharacterValueForCast"],
            Self::InvalidDatetimeFormat => &["InvalidDatetimeFormat"],
            Self::InvalidEscapeCharacter => &["InvalidEscapeCharacter"],
            Self::InvalidEscapeOctet => &["InvalidEscapeOctet"],
            Self::InvalidEscapeSequence => &["InvalidEscapeSequence"],
            Self::NonstandardUseOfEscapeCharacter => &["NonstandardUseOfEscapeCharacter"],
            Self::InvalidIndicatorParameterValue => &["InvalidIndicatorParameterValue"],
            Self::InvalidParameterValue => &["InvalidParameterValue"],
            Self::InvalidPrecedingOrFollowingSize => &["InvalidPrecedingOrFollowingSize"],
            Self::InvalidRegularExpression => &["InvalidRegularExpression"],
            Self::InvalidRowCountInLimitClause => &["InvalidRowCountInLimitClause"],
            Self::InvalidRowCountInResultOffsetClause => &["InvalidRowCountInResultOffsetClause"],
            Self::InvalidTablesampleArgument => &["InvalidTablesampleArgument"],
            Self::InvalidTablesampleRepeat => &["InvalidTablesampleRepeat"],
            Self::InvalidTimeZoneDisplacementValue => &["InvalidTimeZoneDisplacementValue"],
            Self::InvalidUseOfEscapeCharacter => &["InvalidUseOfEscapeCharacter"],
            Self::MostSpecificTypeMismatch => &["MostSpecificTypeMismatch"],
            Self::NullValueNotAllowed => &["NullValueNotAllowed"],
            Self::NullValueNoIndicatorParameter => &["NullValueNoIndicatorParameter"],
            Self::NumericValueOutOfRange => &["NumericValueOutOfRange"],
            Self::SequenceGeneratorLimitExceeded => &["SequenceGeneratorLimitExceeded"],
            Self::StringDataLengthMismatch => &["StringDataLengthMismatch"],
            Self::StringDataRightTruncation => &["StringDataRightTruncation"],
            Self::SubstringError => &["SubstringError"],
            Self::TrimError => &["TrimError"],
            Self::UnterminatedCString => &["UnterminatedCString"],
            Self::ZeroLengthCharacterString => &["ZeroLengthCharacterString"],
            Self::FloatingPointException => &["FloatingPointException"],
            Self::InvalidTextRepresentation => &["InvalidTextRepresentation"],
            Self::InvalidBinaryRepresentation => &["InvalidBinaryRepresentation"],
            Self::BadCopyFileFormat => &["BadCopyFileFormat"],
            Self::UntranslatableCharacter => &["UntranslatableCharacter"],
            Self::NotAnXmlDocument => &["NotAnXmlDocument"],
            Self::InvalidXmlDocument => &["InvalidXmlDocument"],
            Self::InvalidXmlContent => &["InvalidXmlContent"],
            Self::InvalidXmlComment => &["InvalidXmlComment"],
            Self::InvalidXmlProcessingInstruction => &["InvalidXmlProcessingInstruction"],
            Self::DuplicateJsonObjectKeyValue => &["DuplicateJsonObjectKeyValue"],
            Self::InvalidArgumentForSqlJsonDatetimeFunction => {
                &["InvalidArgumentForSqlJsonDatetimeFunction"]
            }
            Self::InvalidJsonText => &["InvalidJsonText"],
            Self::InvalidSqlJsonSubscript => &["InvalidSqlJsonSubscript"],
            Self::MoreThanOneSqlJsonItem => &["MoreThanOneSqlJsonItem"],
            Self::NoSqlJsonItem => &["NoSqlJsonItem"],
            Self::NonNumericSqlJsonItem => &["NonNumericSqlJsonItem"],
            Self::NonUniqueKeysInAJsonObject => &["NonUniqueKeysInAJsonObject"],
            Self::SingletonSqlJsonItemRequired => &["SingletonSqlJsonItemRequired"],
            Self::SqlJsonArrayNotFound => &["SqlJsonArrayNotFound"],
            Self::SqlJsonMemberNotFound => &["SqlJsonMemberNotFound"],
            Self::SqlJsonNumberNotFound => &["SqlJsonNumberNotFound"],
            Self::SqlJsonObjectNotFound => &["SqlJsonObjectNotFound"],
            Self::TooManyJsonArrayElements => &["TooManyJsonArrayElements"],
            Self::TooManyJsonObjectMembers => &["TooManyJsonObjectMembers"],
            Self::SqlJsonScalarRequired => &["SqlJsonScalarRequired"],
            Self::SqlJsonItemCannotBeCastToTargetType => &["SqlJsonItemCannotBeCastToTargetType"],
            Self::IntegrityConstraintViolation => &["IntegrityConstraintViolation"],
            Self::RestrictViolation => &["RestrictViolation"],
            Self::NotNullViolation => &["NotNullViolation"],
            Self::ForeignKeyViolation => &["ForeignKeyViolation"],
            Self::UniqueViolation => &["UniqueViolation"],
            Self::CheckViolation => &["CheckViolation"],
            Self::ExclusionViolation => &["ExclusionViolation"],
            Self::InvalidCursorState => &["InvalidCursorState"],
            Self::InvalidTransactionState => &["InvalidTransactionState"],
            Self::ActiveSqlTransaction => &["ActiveSqlTransaction"],
            Self::BranchTransactionAlreadyActive => &["BranchTransactionAlreadyActive"],
            Self::HeldCursorRequiresSameIsolationLevel => &["HeldCursorRequiresSameIsolationLevel"],
            Self::InappropriateAccessModeForBranchTransaction => {
                &["InappropriateAccessModeForBranchTransaction"]
            }
            Self::InappropriateIsolationLevelForBranchTransaction => {
                &["InappropriateIsolationLevelForBranchTransaction"]
            }
            Self::NoActiveSqlTransactionForBranchTransaction => {
                &["NoActiveSqlTransactionForBranchTransaction"]
            }
            Self::ReadOnlySqlTransaction => &["ReadOnlySqlTransaction"],
            Self::SchemaAndDataStatementMixingNotSupported => {
                &["SchemaAndDataStatementMixingNotSupported"]
            }
            Self::NoActiveSqlTransaction => &["NoActiveSqlTransaction"],
            Self::InFailedSqlTransaction => &["InFailedSqlTransaction"],
            Self::IdleInTransactionSessionTimeout => &["IdleInTransactionSessionTimeout"],
            Self::TransactionTimeout => &["TransactionTimeout"],
            Self::InvalidSqlStatementName => &["InvalidSqlStatementName", "UndefinedPstatement"],
            Self::TriggeredDataChangeViolation => &["TriggeredDataChangeViolation"],
            Self::InvalidAuthorizationSpecification => &["InvalidAuthorizationSpecification"],
            Self::InvalidPassword => &["InvalidPassword"],
            Self::DependentPrivilegeDescriptorsStillExist => {
                &["DependentPrivilegeDescriptorsStillExist"]
            }
            Self::DependentObjectsStillExist => &["DependentObjectsStillExist"],
            Self::InvalidTransactionTermination => &["InvalidTransactionTermination"],
            Self::SqlRoutineException => &["SqlRoutineException"],
            Self::SREFunctionExecutedNoReturnStatement => &["SREFunctionExecutedNoReturnStatement"],
            Self::SREModifyingSqlDataNotPermitted => &["SREModifyingSqlDataNotPermitted"],
            Self::SREProhibitedSqlStatementAttempted => &["SREProhibitedSqlStatementAttempted"],
            Self::SREReadingSqlDataNotPermitted => &["SREReadingSqlDataNotPermitted"],
            Self::InvalidCursorName => &["InvalidCursorName", "UndefinedCursor"],
            Self::ExternalRoutineException => &["ExternalRoutineException"],
            Self::EREContainingSqlNotPermitted => &["EREContainingSqlNotPermitted"],
            Self::EREModifyingSqlDataNotPermitted => &["EREModifyingSqlDataNotPermitted"],
            Self::EREProhibitedSqlStatementAttempted => &["EREProhibitedSqlStatementAttempted"],
            Self::EREReadingSqlDataNotPermitted => &["EREReadingSqlDataNotPermitted"],
            Self::ExternalRoutineInvocationException => &["ExternalRoutineInvocationException"],
            Self::ERIEInvalidSqlstateReturned => &["ERIEInvalidSqlstateReturned"],
            Self::ERIENullValueNotAllowed => &["ERIENullValueNotAllowed"],
            Self::ERIETriggerProtocolViolated => &["ERIETriggerProtocolViolated"],
            Self::ERIESrfProtocolViolated => &["ERIESrfProtocolViolated"],
            Self::ERIEEventTriggerProtocolViolated => &["ERIEEventTriggerProtocolViolated"],
            Self::SavepointException => &["SavepointException"],
            Self::SEInvalidSpecification => &["SEInvalidSpecification"],
            Self::InvalidCatalogName => &["InvalidCatalogName", "UndefinedDatabase"],
            Self::InvalidSchemaName => &["InvalidSchemaName", "UndefinedSchema"],
            Self::TransactionRollback => &["TransactionRollback"],
            Self::TRIntegrityConstraintViolation => &["TRIntegrityConstraintViolation"],
            Self::TRSerializationFailure => &["TRSerializationFailure"],
            Self::TRStatementCompletionUnknown => &["TRStatementCompletionUnknown"],
            Self::TRDeadlockDetected => &["TRDeadlockDetected"],
            Self::SyntaxErrorOrAccessRuleViolation => &["SyntaxErrorOrAccessRuleViolation"],
            Self::SyntaxError => &["SyntaxError"],
            Self::InsufficientPrivilege => &["InsufficientPrivilege"],
            Self::CannotCoerce => &["CannotCoerce"],
            Self::GroupingError => &["GroupingError"],
            Self::WindowingError => &["WindowingError"],
            Self::InvalidRecursion => &["InvalidRecursion"],
            Self::InvalidForeignKey => &["InvalidForeignKey"],
            Self::InvalidName => &["InvalidName"],
            Self::NameTooLong => &["NameTooLong"],
            Self::ReservedName => &["ReservedName"],
            Self::DatatypeMismatch => &["DatatypeMismatch"],
            Self::IndeterminateDatatype => &["IndeterminateDatatype"],
            Self::CollationMismatch => &["CollationMismatch"],
            Self::IndeterminateCollation => &["IndeterminateCollation"],
            Self::WrongObjectType => &["WrongObjectType"],
            Self::GeneratedAlways => &["GeneratedAlways"],
            Self::UndefinedColumn => &["UndefinedColumn"],
            Self::UndefinedFunction => &["UndefinedFunction"],
            Self::UndefinedTable => &["UndefinedTable"],
            Self::UndefinedParameter => &["UndefinedParameter"],
            Self::UndefinedObject => &["UndefinedObject"],
            Self::DuplicateColumn => &["DuplicateColumn"],
            Self::DuplicateCursor => &["DuplicateCursor"],
            Self::DuplicateDatabase => &["DuplicateDatabase"],
            Self::DuplicateFunction => &["DuplicateFunction"],
            Self::DuplicatePstatement => &["DuplicatePstatement"],
            Self::DuplicateSchema => &["DuplicateSchema"],
            Self::DuplicateTable => &["DuplicateTable"],
            Self::DuplicateAlias => &["DuplicateAlias"],
            Self::DuplicateObject => &["DuplicateObject"],
            Self::AmbiguousColumn => &["AmbiguousColumn"],
            Self::AmbiguousFunction => &["AmbiguousFunction"],
            Self::AmbiguousParameter => &["AmbiguousParameter"],
            Self::AmbiguousAlias => &["AmbiguousAlias"],
            Self::InvalidColumnReference => &["InvalidColumnReference"],
            Self::InvalidColumnDefinition => &["InvalidColumnDefinition"],
            Self::InvalidCursorDefinition => &["InvalidCursorDefinition"],
            Self::InvalidDatabaseDefinition => &["InvalidDatabaseDefinition"],
            Self::InvalidFunctionDefinition => &["InvalidFunctionDefinition"],
            Self::InvalidPstatementDefinition => &["InvalidPstatementDefinition"],
            Self::InvalidSchemaDefinition => &["InvalidSchemaDefinition"],
            Self::InvalidTableDefinition => &["InvalidTableDefinition"],
            Self::InvalidObjectDefinition => &["InvalidObjectDefinition"],
            Self::WithCheckOptionViolation => &["WithCheckOptionViolation"],
            Self::InsufficientResources => &["InsufficientResources"],
            Self::DiskFull => &["DiskFull"],
            Self::OutOfMemory => &["OutOfMemory"],
            Self::TooManyConnections => &["TooManyConnections"],
            Self::ConfigurationLimitExceeded => &["ConfigurationLimitExceeded"],
            Self::ProgramLimitExceeded => &["ProgramLimitExceeded"],
            Self::StatementTooComplex => &["StatementTooComplex"],
            Self::TooManyColumns => &["TooManyColumns"],
            Self::TooManyArguments => &["TooManyArguments"],
            Self::ObjectNotInPrerequisiteState => &["ObjectNotInPrerequisiteState"],
            Self::ObjectInUse => &["ObjectInUse"],
            Self::CantChangeRuntimeParam => &["CantChangeRuntimeParam"],
            Self::LockNotAvailable => &["LockNotAvailable"],
            Self::UnsafeNewEnumValueUsage => &["UnsafeNewEnumValueUsage"],
            Self::OperatorIntervention => &["OperatorIntervention"],
            Self::QueryCanceled => &["QueryCanceled"],
            Self::AdminShutdown => &["AdminShutdown"],
            Self::CrashShutdown => &["CrashShutdown"],
            Self::CannotConnectNow => &["CannotConnectNow"],
            Self::DatabaseDropped => &["DatabaseDropped"],
            Self::IdleSessionTimeout => &["IdleSessionTimeout"],
            Self::SystemError => &["SystemError"],
            Self::IoError => &["IoError"],
            Self::UndefinedFile => &["UndefinedFile"],
            Self::DuplicateFile => &["DuplicateFile"],
            Self::FileNameTooLong => &["FileNameTooLong"],
            Self::ConfigFileError => &["ConfigFileError"],
            Self::LockFileExists => &["LockFileExists"],
            Self::FdwError => &["FdwError"],
            Self::FdwColumnNameNotFound => &["FdwColumnNameNotFound"],
            Self::FdwDynamicParameterValueNeeded => &["FdwDynamicParameterValueNeeded"],
            Self::FdwFunctionSequenceError => &["FdwFunctionSequenceError"],
            Self::FdwInconsistentDescriptorInformation => &["FdwInconsistentDescriptorInformation"],
            Self::FdwInvalidAttributeValue => &["FdwInvalidAttributeValue"],
            Self::FdwInvalidColumnName => &["FdwInvalidColumnName"],
            Self::FdwInvalidColumnNumber => &["FdwInvalidColumnNumber"],
            Self::FdwInvalidDataType => &["FdwInvalidDataType"],
            Self::FdwInvalidDataTypeDescriptors => &["FdwInvalidDataTypeDescriptors"],
            Self::FdwInvalidDescriptorFieldIdentifier => &["FdwInvalidDescriptorFieldIdentifier"],
            Self::FdwInvalidHandle => &["FdwInvalidHandle"],
            Self::FdwInvalidOptionIndex => &["FdwInvalidOptionIndex"],
            Self::FdwInvalidOptionName => &["FdwInvalidOptionName"],
            Self::FdwInvalidStringLengthOrBufferLength => &["FdwInvalidStringLengthOrBufferLength"],
            Self::FdwInvalidStringFormat => &["FdwInvalidStringFormat"],
            Self::FdwInvalidUseOfNullPointer => &["FdwInvalidUseOfNullPointer"],
            Self::FdwTooManyHandles => &["FdwTooManyHandles"],
            Self::FdwOutOfMemory => &["FdwOutOfMemory"],
            Self::FdwNoSchemas => &["FdwNoSchemas"],
            Self::FdwOptionNameNotFound => &["FdwOptionNameNotFound"],
            Self::FdwReplyHandle => &["FdwReplyHandle"],
            Self::FdwSchemaNotFound => &["FdwSchemaNotFound"],
            Self::FdwTableNotFound => &["FdwTableNotFound"],
            Self::FdwUnableToCreateExecution => &["FdwUnableToCreateExecution"],
            Self::FdwUnableToCreateReply => &["FdwUnableToCreateReply"],
            Self::FdwUnableToEstablishConnection => &["FdwUnableToEstablishConnection"],
            Self::PlpgsqlError => &["PlpgsqlError"],
            Self::RaiseException => &["RaiseException"],
            Self::NoDataFound => &["NoDataFound"],
            Self::TooManyRows => &["TooManyRows"],
            Self::AssertFailure => &["AssertFailure"],
            Self::InternalError => &["InternalError"],
            Self::DataCorrupted => &["DataCorrupted"],
            Self::IndexCorrupted => &["IndexCorrupted"],
            Self::Other(_) => &["Other"],
        }
    }
}

impl core::fmt::Debug for SqlState {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "SqlState({}", self.as_str())?;

        for name in self.names() {
            write!(formatter, " {name}")?;
        }

        formatter.write_str(")")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UnknownSqlState([u8; 5]);

impl UnknownSqlState {
    #[must_use]
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.0).expect("validated as ASCII on construction")
    }
}

impl core::fmt::Debug for UnknownSqlState {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "UnknownSqlState({})", self.as_str())
    }
}

/// Classes beginning `0` to `4` or `A` to `H` are the SQL standard's, and so
/// are the subclasses within them. Everything else PostgreSQL defines for
/// itself, along with subclasses beginning `P` inside standard classes.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Class {
    SuccessfulCompletion,
    Warning,
    NoData,
    SqlStatementNotYetComplete,
    ConnectionException,
    TriggeredActionException,
    FeatureNotSupported,
    InvalidTransactionInitiation,
    LocatorException,
    InvalidGrantor,
    InvalidRoleSpecification,
    DiagnosticsException,
    XqueryError,
    CaseNotFound,
    CardinalityViolation,
    DataException,
    IntegrityConstraintViolation,
    InvalidCursorState,
    InvalidTransactionState,
    InvalidSqlStatementName,
    TriggeredDataChangeViolation,
    InvalidAuthorizationSpecification,
    DependentPrivilegeDescriptorsStillExist,
    InvalidTransactionTermination,
    SqlRoutineException,
    InvalidCursorName,
    ExternalRoutineException,
    ExternalRoutineInvocationException,
    SavepointException,
    InvalidCatalogName,
    InvalidSchemaName,
    TransactionRollback,
    SyntaxErrorOrAccessRuleViolation,
    WithCheckOptionViolation,
    InsufficientResources,
    ProgramLimitExceeded,
    ObjectNotInPrerequisiteState,
    OperatorIntervention,
    SystemError,
    ConfigurationFileError,
    ForeignDataWrapperError,
    PlPgsqlError,
    InternalError,

    Other(UnknownClass),
}

impl Class {
    fn new(bytes: [u8; 2]) -> Self {
        match &bytes {
            b"00" => Self::SuccessfulCompletion,
            b"01" => Self::Warning,
            b"02" => Self::NoData,
            b"03" => Self::SqlStatementNotYetComplete,
            b"08" => Self::ConnectionException,
            b"09" => Self::TriggeredActionException,
            b"0A" => Self::FeatureNotSupported,
            b"0B" => Self::InvalidTransactionInitiation,
            b"0F" => Self::LocatorException,
            b"0L" => Self::InvalidGrantor,
            b"0P" => Self::InvalidRoleSpecification,
            b"0Z" => Self::DiagnosticsException,
            b"10" => Self::XqueryError,
            b"20" => Self::CaseNotFound,
            b"21" => Self::CardinalityViolation,
            b"22" => Self::DataException,
            b"23" => Self::IntegrityConstraintViolation,
            b"24" => Self::InvalidCursorState,
            b"25" => Self::InvalidTransactionState,
            b"26" => Self::InvalidSqlStatementName,
            b"27" => Self::TriggeredDataChangeViolation,
            b"28" => Self::InvalidAuthorizationSpecification,
            b"2B" => Self::DependentPrivilegeDescriptorsStillExist,
            b"2D" => Self::InvalidTransactionTermination,
            b"2F" => Self::SqlRoutineException,
            b"34" => Self::InvalidCursorName,
            b"38" => Self::ExternalRoutineException,
            b"39" => Self::ExternalRoutineInvocationException,
            b"3B" => Self::SavepointException,
            b"3D" => Self::InvalidCatalogName,
            b"3F" => Self::InvalidSchemaName,
            b"40" => Self::TransactionRollback,
            b"42" => Self::SyntaxErrorOrAccessRuleViolation,
            b"44" => Self::WithCheckOptionViolation,
            b"53" => Self::InsufficientResources,
            b"54" => Self::ProgramLimitExceeded,
            b"55" => Self::ObjectNotInPrerequisiteState,
            b"57" => Self::OperatorIntervention,
            b"58" => Self::SystemError,
            b"F0" => Self::ConfigurationFileError,
            b"HV" => Self::ForeignDataWrapperError,
            b"P0" => Self::PlPgsqlError,
            b"XX" => Self::InternalError,
            _ => Self::Other(UnknownClass(bytes)),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::SuccessfulCompletion => "00",
            Self::Warning => "01",
            Self::NoData => "02",
            Self::SqlStatementNotYetComplete => "03",
            Self::ConnectionException => "08",
            Self::TriggeredActionException => "09",
            Self::FeatureNotSupported => "0A",
            Self::InvalidTransactionInitiation => "0B",
            Self::LocatorException => "0F",
            Self::InvalidGrantor => "0L",
            Self::InvalidRoleSpecification => "0P",
            Self::DiagnosticsException => "0Z",
            Self::XqueryError => "10",
            Self::CaseNotFound => "20",
            Self::CardinalityViolation => "21",
            Self::DataException => "22",
            Self::IntegrityConstraintViolation => "23",
            Self::InvalidCursorState => "24",
            Self::InvalidTransactionState => "25",
            Self::InvalidSqlStatementName => "26",
            Self::TriggeredDataChangeViolation => "27",
            Self::InvalidAuthorizationSpecification => "28",
            Self::DependentPrivilegeDescriptorsStillExist => "2B",
            Self::InvalidTransactionTermination => "2D",
            Self::SqlRoutineException => "2F",
            Self::InvalidCursorName => "34",
            Self::ExternalRoutineException => "38",
            Self::ExternalRoutineInvocationException => "39",
            Self::SavepointException => "3B",
            Self::InvalidCatalogName => "3D",
            Self::InvalidSchemaName => "3F",
            Self::TransactionRollback => "40",
            Self::SyntaxErrorOrAccessRuleViolation => "42",
            Self::WithCheckOptionViolation => "44",
            Self::InsufficientResources => "53",
            Self::ProgramLimitExceeded => "54",
            Self::ObjectNotInPrerequisiteState => "55",
            Self::OperatorIntervention => "57",
            Self::SystemError => "58",
            Self::ConfigurationFileError => "F0",
            Self::ForeignDataWrapperError => "HV",
            Self::PlPgsqlError => "P0",
            Self::InternalError => "XX",
            Self::Other(class) => class.as_str(),
        }
    }

    fn names(&self) -> &'static [&'static str] {
        match self {
            Self::SuccessfulCompletion => &["SuccessfulCompletion"],
            Self::Warning => &["Warning"],
            Self::NoData => &["NoData"],
            Self::SqlStatementNotYetComplete => &["SqlStatementNotYetComplete"],
            Self::ConnectionException => &["ConnectionException"],
            Self::TriggeredActionException => &["TriggeredActionException"],
            Self::FeatureNotSupported => &["FeatureNotSupported"],
            Self::InvalidTransactionInitiation => &["InvalidTransactionInitiation"],
            Self::LocatorException => &["LocatorException"],
            Self::InvalidGrantor => &["InvalidGrantor"],
            Self::InvalidRoleSpecification => &["InvalidRoleSpecification"],
            Self::DiagnosticsException => &["DiagnosticsException"],
            Self::XqueryError => &["XqueryError"],
            Self::CaseNotFound => &["CaseNotFound"],
            Self::CardinalityViolation => &["CardinalityViolation"],
            Self::DataException => &["DataException"],
            Self::IntegrityConstraintViolation => &["IntegrityConstraintViolation"],
            Self::InvalidCursorState => &["InvalidCursorState"],
            Self::InvalidTransactionState => &["InvalidTransactionState"],
            Self::InvalidSqlStatementName => &["InvalidSqlStatementName"],
            Self::TriggeredDataChangeViolation => &["TriggeredDataChangeViolation"],
            Self::InvalidAuthorizationSpecification => &["InvalidAuthorizationSpecification"],
            Self::DependentPrivilegeDescriptorsStillExist => {
                &["DependentPrivilegeDescriptorsStillExist"]
            }
            Self::InvalidTransactionTermination => &["InvalidTransactionTermination"],
            Self::SqlRoutineException => &["SqlRoutineException"],
            Self::InvalidCursorName => &["InvalidCursorName"],
            Self::ExternalRoutineException => &["ExternalRoutineException"],
            Self::ExternalRoutineInvocationException => &["ExternalRoutineInvocationException"],
            Self::SavepointException => &["SavepointException"],
            Self::InvalidCatalogName => &["InvalidCatalogName"],
            Self::InvalidSchemaName => &["InvalidSchemaName"],
            Self::TransactionRollback => &["TransactionRollback"],
            Self::SyntaxErrorOrAccessRuleViolation => &["SyntaxErrorOrAccessRuleViolation"],
            Self::WithCheckOptionViolation => &["WithCheckOptionViolation"],
            Self::InsufficientResources => &["InsufficientResources"],
            Self::ProgramLimitExceeded => &["ProgramLimitExceeded"],
            Self::ObjectNotInPrerequisiteState => &["ObjectNotInPrerequisiteState"],
            Self::OperatorIntervention => &["OperatorIntervention"],
            Self::SystemError => &["SystemError"],
            Self::ConfigurationFileError => &["ConfigurationFileError"],
            Self::ForeignDataWrapperError => &["ForeignDataWrapperError"],
            Self::PlPgsqlError => &["PlPgsqlError"],
            Self::InternalError => &["InternalError"],
            Self::Other(_) => &["Other"],
        }
    }
}

impl core::fmt::Debug for Class {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "Class({}", self.as_str())?;

        for name in self.names() {
            write!(formatter, " {name}")?;
        }

        formatter.write_str(")")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UnknownClass([u8; 2]);

impl UnknownClass {
    #[must_use]
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.0).expect("validated as ASCII on construction")
    }
}

impl core::fmt::Debug for UnknownClass {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "UnknownClass({})", self.as_str())
    }
}

/// Upstream describes the `ErrorResponse` and `NoticeResponse` bodies together
/// as "Error and Notice Message Fields"; of those, [`FieldType::SEVERITY`],
/// [`FieldType::CODE`] and [`FieldType::MESSAGE`] are documented as always
/// present.
///
/// Format: <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-ERRORRESPONSE>
///
/// Fields: <https://www.postgresql.org/docs/current/protocol-error-fields.html>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic<'a> {
    severity: Severity,
    code: SqlState,
    message: &'a str,
    position: Option<u32>,
    internal_position: Option<u32>,
    line: Option<u32>,
    schema: Option<Schema<'a>>,
    table: Option<Table<'a>>,
    column: Option<Column<'a>>,
    data_type: Option<Type<'a>>,
    constraint: Option<Constraint<'a>>,
    fields: &'a [u8],
}

impl<'a> Diagnostic<'a> {
    #[must_use]
    pub const fn severity(&self) -> Severity {
        self.severity
    }

    #[must_use]
    pub const fn code(&self) -> SqlState {
        self.code
    }

    /// The primary human readable message, typically one line.
    #[must_use]
    pub const fn message(&self) -> &'a str {
        self.message
    }

    #[must_use]
    pub const fn position(&self) -> Option<u32> {
        self.position
    }

    /// One based index into an internally generated command, which
    /// [`FieldType::INTERNAL_QUERY`] carries.
    #[must_use]
    pub const fn internal_position(&self) -> Option<u32> {
        self.internal_position
    }

    /// Line of the server source location that reported this.
    #[must_use]
    pub const fn line(&self) -> Option<u32> {
        self.line
    }

    #[must_use]
    pub const fn schema(&self) -> Option<&Schema<'a>> {
        self.schema.as_ref()
    }

    #[must_use]
    pub const fn table(&self) -> Option<&Table<'a>> {
        self.table.as_ref()
    }

    #[must_use]
    pub const fn column(&self) -> Option<&Column<'a>> {
        self.column.as_ref()
    }

    #[must_use]
    pub const fn data_type(&self) -> Option<&Type<'a>> {
        self.data_type.as_ref()
    }

    #[must_use]
    pub const fn constraint(&self) -> Option<&Constraint<'a>> {
        self.constraint.as_ref()
    }

    /// A field type appears at most once per message.
    #[must_use]
    pub fn field(&self, field: FieldType) -> Option<&'a str> {
        let mut cursor = Cursor::new(self.fields);

        loop {
            match cursor.byte() {
                Ok(0) | Err(_) => return None,
                Ok(code) => match cursor.string() {
                    Err(_) => return None,
                    Ok(value) if code == field.get() => return Some(value),
                    Ok(_) => {}
                },
            }
        }
    }

    fn decode(mut cursor: Cursor<'a>) -> Result<Self, DecodeError> {
        let fields = cursor.rest();

        let mut severity = None;
        let mut code = None;
        let mut message = None;
        let mut position = None;
        let mut internal_position = None;
        let mut line = None;
        let mut schema = None;
        let mut table = None;
        let mut column = None;
        let mut data_type = None;
        let mut constraint = None;

        loop {
            let field = FieldType::new(cursor.byte()?);

            if field.get() == 0 {
                break;
            }

            let value = cursor.string()?;

            let integer = || {
                value.parse().map_err(|_| DecodeError::NotAnInteger {
                    field,
                    value: value.to_owned(),
                })
            };

            match field {
                FieldType::SEVERITY => severity = Some(Severity::new(value)?),
                FieldType::CODE => code = Some(SqlState::new(value)?),
                FieldType::MESSAGE => message = Some(value),
                FieldType::POSITION => position = Some(integer()?),
                FieldType::INTERNAL_POSITION => internal_position = Some(integer()?),
                FieldType::LINE => line = Some(integer()?),
                FieldType::SCHEMA_NAME => {
                    schema = Some(parse_identifier(field, value, Schema::new)?);
                }
                FieldType::TABLE_NAME => table = Some(parse_identifier(field, value, Table::new)?),
                FieldType::COLUMN_NAME => {
                    column = Some(parse_identifier(field, value, Column::new)?);
                }
                FieldType::DATA_TYPE_NAME => {
                    data_type = Some(parse_identifier(field, value, Type::new)?);
                }
                FieldType::CONSTRAINT_NAME => {
                    constraint = Some(parse_identifier(field, value, Constraint::new)?);
                }
                _ => {}
            }
        }

        cursor.end()?;

        Ok(Self {
            severity: severity.ok_or(DecodeError::MissingField {
                field: FieldType::SEVERITY,
            })?,
            code: code.ok_or(DecodeError::MissingField {
                field: FieldType::CODE,
            })?,
            message: message.ok_or(DecodeError::MissingField {
                field: FieldType::MESSAGE,
            })?,
            position,
            internal_position,
            line,
            schema,
            table,
            column,
            data_type,
            constraint,
            fields,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessId(i32);

impl ProcessId {
    #[must_use]
    pub(crate) const fn new(value: i32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> i32 {
        self.0
    }
}

/// PostgreSQL sends at most 32 bytes, but the protocol allows more so that
/// poolers and other middleware can wrap the server's key with their own data.
/// Upstream requires the frontend to save it. Before protocol 3.2 this was
/// always 4 bytes.
#[derive(Clone, PartialEq, Eq)]
pub struct CancelKey(Vec<u8>);

impl fmt::Debug for CancelKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "CancelKey({} bytes)", self.0.len())
    }
}

impl CancelKey {
    pub(crate) fn new(bytes: &[u8]) -> Result<Self, DecodeError> {
        if (4..=256).contains(&bytes.len()) {
            Ok(Self(bytes.to_vec()))
        } else {
            Err(DecodeError::InvalidCancelKey {
                length: bytes.len(),
            })
        }
    }

    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

/// <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-BACKENDKEYDATA>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BackendKeyData {
    process_id: ProcessId,
    cancel_key: CancelKey,
}

impl BackendKeyData {
    pub(crate) const fn new(process_id: ProcessId, cancel_key: CancelKey) -> Self {
        Self {
            process_id,
            cancel_key,
        }
    }

    #[must_use]
    pub const fn process_id(&self) -> ProcessId {
        self.process_id
    }

    #[must_use]
    pub const fn cancel_key(&self) -> &CancelKey {
        &self.cancel_key
    }

    fn decode(mut cursor: Cursor<'_>) -> Result<Self, DecodeError> {
        Ok(Self::new(
            ProcessId::new(cursor.int32()?),
            CancelKey::new(cursor.rest())?,
        ))
    }
}

/// Sent at startup and again whenever the parameter changes.
///
/// Only parameters flagged `GUC_REPORT` arrive this way.
///
/// <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-PARAMETERSTATUS>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterStatus {
    pub name: parameter::Name,
    pub value: parameter::Value,
}

impl ParameterStatus {
    fn decode(mut cursor: Cursor<'_>) -> Result<Self, DecodeError> {
        let name = cursor.string()?;
        let value = cursor.string()?;

        cursor.end()?;

        Ok(Self {
            name: name
                .parse()
                .map_err(|source| DecodeError::NotAParameterName {
                    value: name.to_owned(),
                    source,
                })?,
            value: value
                .parse()
                .map_err(|source| DecodeError::NotAParameterValue {
                    value: value.to_owned(),
                    source,
                })?,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionStatus {
    Idle,
    InTransaction,
    /// Queries are rejected until the transaction block ends.
    InFailedTransaction,
}

impl TransactionStatus {
    fn new(status: u8) -> Result<Self, DecodeError> {
        match status {
            b'I' => Ok(Self::Idle),
            b'T' => Ok(Self::InTransaction),
            b'E' => Ok(Self::InFailedTransaction),
            status => Err(DecodeError::UnknownTransactionStatus { status }),
        }
    }
}

/// The row counts are documented per command, but the set of commands is not
/// documented as closed.
///
/// <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-COMMANDCOMPLETE>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandComplete<'a> {
    Insert {
        rows: u64,
    },
    Delete {
        rows: u64,
    },
    Update {
        rows: u64,
    },
    Merge {
        rows: u64,
    },
    /// Also `CREATE TABLE AS`.
    Select {
        rows: u64,
    },
    Move {
        rows: u64,
    },
    Fetch {
        rows: u64,
    },
    Copy {
        rows: u64,
    },
    Other(&'a str),
}

impl<'a> CommandComplete<'a> {
    fn decode(mut cursor: Cursor<'a>) -> Result<Self, DecodeError> {
        let tag = cursor.string()?;

        cursor.end()?;

        let malformed = || DecodeError::MalformedCommandTag {
            tag: tag.to_owned(),
        };

        let mut words = tag.split(' ');
        let command = words.next().ok_or_else(malformed)?;

        let mut number = || {
            words
                .next()
                .ok_or_else(malformed)?
                .parse()
                .map_err(|_| malformed())
        };

        Ok(match command {
            "INSERT" => {
                // Upstream says the object ID is always zero now that OID
                // system columns are gone.
                let _object_id = number()?;

                Self::Insert { rows: number()? }
            }
            "DELETE" => Self::Delete { rows: number()? },
            "UPDATE" => Self::Update { rows: number()? },
            "MERGE" => Self::Merge { rows: number()? },
            "SELECT" => Self::Select { rows: number()? },
            "MOVE" => Self::Move { rows: number()? },
            "FETCH" => Self::Fetch { rows: number()? },
            "COPY" => Self::Copy { rows: number()? },
            _ => Self::Other(tag),
        })
    }
}

/// A column derived from an expression gets `?column?`, but that arrives
/// identically to a column deliberately named `"?column?"`, so the two are not
/// distinguishable here.
///
/// The server truncates an alias longer than 63 bytes before it reaches the
/// wire, and reports the truncation as a notice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColumnDescription<'a> {
    pub name: Column<'a>,
    pub table: Option<Oid>,
    pub attribute: Option<i16>,
    pub data_type: Oid,
    /// `pg_type.typlen`. Negative denotes a variable width type.
    pub type_size: i16,
    /// `pg_attribute.atttypmod`, whose meaning is specific to the type.
    pub type_modifier: i32,
    /// Always [`Format::Text`] in a description returned from the statement
    /// variant of `Describe`, where the format is not yet chosen.
    pub format: Format,
}

impl ColumnDescription<'_> {
    fn decode<'a>(cursor: &mut Cursor<'a>) -> Result<ColumnDescription<'a>, DecodeError> {
        let name = cursor.string()?;

        Ok(ColumnDescription {
            name: Column::new(name).map_err(|source| DecodeError::InvalidColumnName {
                value: name.to_owned(),
                source,
            })?,
            // Zero means the field is not a column of a specific table.
            table: Some(cursor.oid()?).filter(|oid| !oid.is_none()),
            attribute: Some(cursor.int16()?).filter(|attribute| *attribute != 0),
            data_type: cursor.oid()?,
            type_size: cursor.int16()?,
            type_modifier: cursor.int32()?,
            format: format(cursor.int16()?)?,
        })
    }
}

/// <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-ROWDESCRIPTION>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RowDescription<'a> {
    columns: usize,
    fields: &'a [u8],
}

impl<'a> RowDescription<'a> {
    #[must_use]
    pub const fn len(&self) -> usize {
        self.columns
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.columns == 0
    }

    fn decode(mut cursor: Cursor<'a>) -> Result<Self, DecodeError> {
        let columns = cursor.count()?;
        let fields = cursor.rest();

        for _ in 0..columns {
            ColumnDescription::decode(&mut cursor)?;
        }

        cursor.end()?;

        Ok(Self { columns, fields })
    }
}

impl<'a> IntoIterator for RowDescription<'a> {
    type IntoIter = Columns<'a>;
    type Item = ColumnDescription<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Columns {
            remaining: self.columns,
            fields: self.fields,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Columns<'a> {
    remaining: usize,
    fields: &'a [u8],
}

impl<'a> Iterator for Columns<'a> {
    type Item = ColumnDescription<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.remaining = self.remaining.checked_sub(1)?;

        let mut cursor = Cursor::new(self.fields);

        let column = ColumnDescription::decode(&mut cursor).ok()?;

        self.fields = cursor.rest();

        Some(column)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for Columns<'_> {}

/// Upstream notes that every column of a given `COPY` uses one format in the
/// present implementation, while the message deliberately does not assume that.
///
/// <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-COPYINRESPONSE>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CopyResponse {
    /// Text means rows separated by newlines and columns by a separator; binary
    /// resembles [`DataRow`].
    pub format: Format,
    pub columns: Vec<Format>,
}

impl CopyResponse {
    fn decode(mut cursor: Cursor<'_>) -> Result<Self, DecodeError> {
        let overall = format(i16::from(cursor.byte()?))?;
        let count = cursor.count()?;
        let mut columns = Vec::with_capacity(count);

        for _ in 0..count {
            let column = format(cursor.int16()?)?;

            if overall == Format::Text && column == Format::Binary {
                return Err(DecodeError::TextualCopyWithBinaryColumn);
            }

            columns.push(column);
        }

        cursor.end()?;

        Ok(Self {
            format: overall,
            columns,
        })
    }
}

/// Ends the command. In the extended protocol the server then discards
/// everything until a `Sync`.
///
/// <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-ERRORRESPONSE>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErrorResponse<'a> {
    pub diagnostic: Diagnostic<'a>,
}

/// The command carries on. May also arrive unprompted, with no command
/// outstanding at all.
///
/// <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-NOTICERESPONSE>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NoticeResponse<'a> {
    pub diagnostic: Diagnostic<'a>,
}

/// Values are in the formats `Bind` chose.
///
/// <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-DATAROW>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataRow<'a> {
    columns: usize,
    values: &'a [u8],
}

impl<'a> DataRow<'a> {
    #[must_use]
    pub const fn len(&self) -> usize {
        self.columns
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.columns == 0
    }

    fn decode(mut cursor: Cursor<'a>) -> Result<Self, DecodeError> {
        let columns = cursor.count()?;
        let values = cursor.rest();

        for _ in 0..columns {
            let length = cursor.int32()?;

            if length >= 0 {
                cursor.take(usize::try_from(length).unwrap_or(0))?;
            }
        }

        cursor.end()?;

        Ok(Self { columns, values })
    }
}

impl<'a> IntoIterator for DataRow<'a> {
    type IntoIter = Values<'a>;
    type Item = Option<&'a [u8]>;

    fn into_iter(self) -> Self::IntoIter {
        Values {
            remaining: self.columns,
            values: self.values,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Values<'a> {
    remaining: usize,
    values: &'a [u8],
}

impl<'a> Iterator for Values<'a> {
    type Item = Option<&'a [u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        self.remaining = self.remaining.checked_sub(1)?;

        let mut cursor = Cursor::new(self.values);

        let length = cursor.int32().ok()?;

        let value = if length < 0 {
            None
        } else {
            Some(cursor.take(usize::try_from(length).ok()?).ok()?)
        };

        self.values = cursor.rest();

        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for Values<'_> {}

/// <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-PARAMETERDESCRIPTION>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParameterDescription {
    pub types: Vec<Oid>,
}

impl ParameterDescription {
    fn decode(mut cursor: Cursor<'_>) -> Result<Self, DecodeError> {
        let count = cursor.count()?;
        let mut types = Vec::with_capacity(count);

        for _ in 0..count {
            types.push(cursor.oid()?);
        }

        cursor.end()?;

        Ok(Self { types })
    }
}

/// Sent by the server for a `NOTIFY`; it may arrive at any point in the
/// stream.
///
/// <https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-NOTIFICATIONRESPONSE>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotificationResponse<'a> {
    /// The backend that raised it, which need not be this session's.
    pub process_id: ProcessId,
    pub channel: Channel<'a>,
    pub payload: &'a str,
}

impl<'a> NotificationResponse<'a> {
    fn decode(mut cursor: Cursor<'a>) -> Result<Self, DecodeError> {
        let process_id = ProcessId::new(cursor.int32()?);
        let channel = cursor.string()?;

        let notification = Self {
            process_id,
            channel: Channel::new(channel).map_err(|source| DecodeError::InvalidChannel {
                value: channel.to_owned(),
                source,
            })?,
            payload: cursor.string()?,
        };

        cursor.end()?;

        Ok(notification)
    }
}

fn empty<'a>(cursor: Cursor<'_>, message: Message<'a>) -> Result<Message<'a>, DecodeError> {
    cursor.end()?;

    Ok(message)
}

fn format(code: i16) -> Result<Format, DecodeError> {
    match code {
        0 => Ok(Format::Text),
        1 => Ok(Format::Binary),
        code => Err(DecodeError::UnknownFormat { code }),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Message<'a> {
    Authentication(Authentication<'a>),
    BackendKeyData(BackendKeyData),
    BindComplete,
    CloseComplete,
    CommandComplete(CommandComplete<'a>),
    /// Upstream says it "is used only for Streaming Replication", in answer to
    /// `START_REPLICATION`.
    CopyBothResponse(CopyResponse),
    /// Part of a copy stream, in either direction. Copying out, one message is
    /// one row; copying in, the frontend may split the stream anywhere.
    CopyData(&'a [u8]),
    CopyDone,
    CopyInResponse(CopyResponse),
    /// This cannot be refused: a frontend that does not want the rows either
    /// discards them or cancels the query.
    CopyOutResponse(CopyResponse),
    DataRow(DataRow<'a>),
    EmptyQueryResponse,
    ErrorResponse(ErrorResponse<'a>),
    NegotiateProtocolVersion(NegotiateProtocolVersion),
    NoData,
    NoticeResponse(NoticeResponse<'a>),
    NotificationResponse(NotificationResponse<'a>),
    ParameterDescription(ParameterDescription),
    ParameterStatus(ParameterStatus),
    ParseComplete,
    PortalSuspended,
    ReadyForQuery(TransactionStatus),
    RowDescription(RowDescription<'a>),
}

impl<'a> Message<'a> {
    pub fn decode(frame: &'a Frame) -> Result<Self, DecodeError> {
        let cursor = Cursor::new(frame.body().as_slice());

        match frame.tag() {
            b'R' => Ok(Self::Authentication(Authentication::decode(cursor)?)),
            b'1' => empty(cursor, Self::ParseComplete),
            b'2' => empty(cursor, Self::BindComplete),
            b'3' => empty(cursor, Self::CloseComplete),
            b'A' => Ok(Self::NotificationResponse(NotificationResponse::decode(
                cursor,
            )?)),
            b'C' => Ok(Self::CommandComplete(CommandComplete::decode(cursor)?)),
            b'D' => Ok(Self::DataRow(DataRow::decode(cursor)?)),
            b'G' => Ok(Self::CopyInResponse(CopyResponse::decode(cursor)?)),
            b'H' => Ok(Self::CopyOutResponse(CopyResponse::decode(cursor)?)),
            b'W' => Ok(Self::CopyBothResponse(CopyResponse::decode(cursor)?)),
            b'c' => empty(cursor, Self::CopyDone),
            b'd' => Ok(Self::CopyData(cursor.rest())),
            b'I' => empty(cursor, Self::EmptyQueryResponse),
            b'T' => Ok(Self::RowDescription(RowDescription::decode(cursor)?)),
            b'n' => empty(cursor, Self::NoData),
            b's' => empty(cursor, Self::PortalSuspended),
            b't' => Ok(Self::ParameterDescription(ParameterDescription::decode(
                cursor,
            )?)),
            b'E' => Ok(Self::ErrorResponse(ErrorResponse {
                diagnostic: Diagnostic::decode(cursor)?,
            })),
            b'K' => Ok(Self::BackendKeyData(BackendKeyData::decode(cursor)?)),
            b'N' => Ok(Self::NoticeResponse(NoticeResponse {
                diagnostic: Diagnostic::decode(cursor)?,
            })),
            b'S' => Ok(Self::ParameterStatus(ParameterStatus::decode(cursor)?)),
            b'Z' => {
                let mut cursor = cursor;
                let status = TransactionStatus::new(cursor.byte()?)?;

                cursor.end()?;

                Ok(Self::ReadyForQuery(status))
            }
            b'v' => Ok(Self::NegotiateProtocolVersion(
                NegotiateProtocolVersion::decode(cursor)?,
            )),
            tag => Err(DecodeError::UnknownTag { tag }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::protocol::Body;
    use pretty_assertions::assert_eq;
    use std::sync::Arc;

    fn frame(tag: u8, body: &[u8]) -> Frame {
        Frame::new(tag, Body::claim(Arc::new(body.to_vec()), 0..body.len()))
    }

    #[test]
    fn keeps_an_offered_mechanism_it_does_not_implement_as_text() {
        let mechanisms = Mechanisms::decode(b"OAUTHBEARER\0SCRAM-SHA-256\0\0").unwrap();

        assert_eq!(
            mechanisms.into_iter().collect::<Vec<_>>(),
            vec![
                Mechanism::Other("OAUTHBEARER"),
                Mechanism::Known(crate::authentication::Mechanism::ScramSha256),
            ]
        );
    }

    #[test]
    fn decodes_an_error_with_its_constrained_fields_parsed() {
        let body = [
            b"SERROR\x00".as_slice(),
            b"VERROR\x00",
            b"C42P01\x00",
            b"Mrelation \"nope\" does not exist\x00",
            b"P15\x00",
            b"Fparse_relation.c\x00",
            b"L1234\x00",
            b"spublic\x00",
            b"tusers\x00",
            b"nusers_pkey\x00",
            b"\x00",
        ]
        .concat();

        let frame = frame(b'E', &body);

        let Ok(Message::ErrorResponse(ErrorResponse { diagnostic: error })) =
            Message::decode(&frame)
        else {
            panic!("expected an error, got {:?}", Message::decode(&frame));
        };

        assert_eq!(error.severity(), Severity::Error);
        assert_eq!(error.code(), SqlState::UndefinedTable);
        assert_eq!(
            error.code().class(),
            Class::SyntaxErrorOrAccessRuleViolation,
        );
        assert_eq!(error.message(), "relation \"nope\" does not exist");
        assert_eq!(error.position(), Some(15));
        assert_eq!(error.internal_position(), None);
        assert_eq!(error.line(), Some(1234));

        assert_eq!(error.schema(), Some(&"public".parse().unwrap()));
        assert_eq!(error.table(), Some(&"users".parse().unwrap()));
        assert_eq!(error.constraint(), Some(&"users_pkey".parse().unwrap()));
        assert_eq!(error.column(), None);

        assert_eq!(error.field(FieldType::FILE), Some("parse_relation.c"));
        assert_eq!(error.field(FieldType::HINT), None);
    }

    #[test]
    fn rejects_a_severity_upstream_does_not_define() {
        let body = [
            b"VOOPS\x00".as_slice(),
            b"C42P01\x00",
            b"Mnope\x00",
            b"\x00",
        ]
        .concat();

        assert_eq!(
            Message::decode(&frame(b'E', &body)),
            Err(DecodeError::UnknownSeverity {
                value: "OOPS".to_owned(),
            }),
        );
    }

    #[test]
    fn decodes_a_cancel_key_that_runs_to_the_end_of_the_message() {
        let body = [7i32.to_be_bytes().as_slice(), &[0xab; 32]].concat();

        assert_eq!(
            Message::decode(&frame(b'K', &body)),
            Ok(Message::BackendKeyData(BackendKeyData {
                process_id: ProcessId::new(7),
                cancel_key: CancelKey::new(&[0xab; 32]).unwrap(),
            })),
        );
    }

    #[test]
    fn keeps_a_cancel_key_out_of_its_debug() {
        let key = CancelKey::new(&[0xab; 32]).unwrap();

        assert_eq!(format!("{key:?}"), "CancelKey(32 bytes)");

        let data = BackendKeyData {
            process_id: ProcessId::new(7),
            cancel_key: key,
        };

        assert!(!format!("{data:?}").contains("ab"));
    }

    #[test]
    fn rejects_a_cancel_key_shorter_than_upstream_allows() {
        let body = [7i32.to_be_bytes().as_slice(), &[0xab; 3]].concat();

        assert_eq!(
            Message::decode(&frame(b'K', &body)),
            Err(DecodeError::InvalidCancelKey { length: 3 }),
        );
    }

    #[test]
    fn decodes_a_reported_parameter() {
        assert_eq!(
            Message::decode(&frame(b'S', b"client_encoding\x00UTF8\x00")),
            Ok(Message::ParameterStatus(ParameterStatus {
                name: "client_encoding".parse().unwrap(),
                value: "UTF8".parse().unwrap(),
            })),
        );
    }

    #[test]
    fn decodes_every_transaction_status() {
        for (byte, status) in [
            (b'I', TransactionStatus::Idle),
            (b'T', TransactionStatus::InTransaction),
            (b'E', TransactionStatus::InFailedTransaction),
        ] {
            assert_eq!(
                Message::decode(&frame(b'Z', &[byte])),
                Ok(Message::ReadyForQuery(status)),
            );
        }

        assert_eq!(
            Message::decode(&frame(b'Z', b"X")),
            Err(DecodeError::UnknownTransactionStatus { status: b'X' }),
        );
    }

    #[test]
    fn decodes_command_tags_that_carry_counts() {
        let insert = frame(b'C', b"INSERT 0 3\x00");
        let select = frame(b'C', b"SELECT 42\x00");
        let create = frame(b'C', b"CREATE TABLE\x00");
        let missing = frame(b'C', b"SELECT\x00");

        assert_eq!(
            Message::decode(&insert),
            Ok(Message::CommandComplete(CommandComplete::Insert {
                rows: 3
            })),
        );
        assert_eq!(
            Message::decode(&select),
            Ok(Message::CommandComplete(CommandComplete::Select {
                rows: 42
            })),
        );
        assert_eq!(
            Message::decode(&create),
            Ok(Message::CommandComplete(CommandComplete::Other(
                "CREATE TABLE"
            ))),
        );
        assert_eq!(
            Message::decode(&missing),
            Err(DecodeError::MalformedCommandTag {
                tag: "SELECT".to_owned(),
            }),
        );
    }

    #[test]
    fn decodes_the_version_a_server_offers() {
        let body = [196_608i32.to_be_bytes(), 0i32.to_be_bytes()].concat();

        assert_eq!(
            Message::decode(&frame(b'v', &body)),
            Ok(Message::NegotiateProtocolVersion(
                NegotiateProtocolVersion {
                    version: Version::V3_0,
                },
            )),
        );
    }

    #[test]
    fn refuses_a_version_it_does_not_speak() {
        let body = [0x0003_0003u32.to_be_bytes(), 0u32.to_be_bytes()].concat();

        assert_eq!(
            Message::decode(&frame(b'v', &body)),
            Err(DecodeError::UnsupportedProtocolVersion { code: 0x0003_0003 }),
        );
    }

    #[test]
    fn rejects_protocol_options_that_were_never_requested() {
        let body = [
            0x0003_0000u32.to_be_bytes().as_slice(),
            &1i32.to_be_bytes(),
            b"_pq_.something\x00",
        ]
        .concat();

        assert_eq!(
            Message::decode(&frame(b'v', &body)),
            Err(DecodeError::UnsupportedProtocolOptions {
                options: vec!["_pq_.something".to_owned()],
            }),
        );
    }

    #[test]
    fn decodes_a_binary_copy_out_response() {
        let body = [
            b"\x01".as_slice(),
            &2i16.to_be_bytes(),
            &1i16.to_be_bytes(),
            &1i16.to_be_bytes(),
        ]
        .concat();

        assert_eq!(
            Message::decode(&frame(b'H', &body)),
            Ok(Message::CopyOutResponse(CopyResponse {
                format: Format::Binary,
                columns: vec![Format::Binary, Format::Binary],
            })),
        );
    }

    #[test]
    fn decodes_a_textual_copy_in_response() {
        let body = [b"\x00".as_slice(), &1i16.to_be_bytes(), &0i16.to_be_bytes()].concat();

        assert_eq!(
            Message::decode(&frame(b'G', &body)),
            Ok(Message::CopyInResponse(CopyResponse {
                format: Format::Text,
                columns: vec![Format::Text],
            })),
        );
    }

    #[test]
    fn decodes_a_copy_both_response() {
        let body = [b"\x00".as_slice(), &1i16.to_be_bytes(), &0i16.to_be_bytes()].concat();

        assert_eq!(
            Message::decode(&frame(b'W', &body)),
            Ok(Message::CopyBothResponse(CopyResponse {
                format: Format::Text,
                columns: vec![Format::Text],
            })),
        );
    }

    #[test]
    fn rejects_a_binary_column_in_a_textual_copy() {
        let body = [b"\x00".as_slice(), &1i16.to_be_bytes(), &1i16.to_be_bytes()].concat();

        assert_eq!(
            Message::decode(&frame(b'G', &body)),
            Err(DecodeError::TextualCopyWithBinaryColumn),
        );
    }

    #[test]
    fn decodes_copy_data_as_the_whole_body() {
        assert_eq!(
            Message::decode(&frame(b'd', b"1\tone\n")),
            Ok(Message::CopyData(b"1\tone\n")),
        );
    }

    #[test]
    fn decodes_copy_done() {
        assert_eq!(Message::decode(&frame(b'c', b"")), Ok(Message::CopyDone));
    }

    #[test]
    fn keeps_a_code_it_does_not_name() {
        let code = SqlState::new("ZZ999").unwrap();

        assert_eq!(code.as_str(), "ZZ999");
        assert_eq!(format!("{code:?}"), "SqlState(ZZ999 Other)");
        assert_eq!(format!("{:?}", code.class()), "Class(ZZ Other)");
    }

    #[test]
    fn rejects_a_code_that_is_not_five_upper_case_or_digits() {
        assert_eq!(
            SqlState::new("23514 "),
            Err(DecodeError::InvalidSqlState {
                value: "23514 ".to_owned(),
            }),
        );
    }

    #[test]
    fn carries_every_name_postgresql_has_for_a_code() {
        let code = SqlState::new("3D000").unwrap();

        assert_eq!(code, SqlState::InvalidCatalogName);
        assert_eq!(code, SqlState::UndefinedDatabase);

        assert_eq!(
            format!("{code:?}"),
            "SqlState(3D000 InvalidCatalogName UndefinedDatabase)",
        );
    }

    #[test]
    fn matches_a_second_name_in_pattern_position() {
        let described = match SqlState::new("3D000").unwrap() {
            SqlState::UndefinedDatabase => "no such database",
            _ => "something else",
        };

        assert_eq!(described, "no such database");
    }

    #[test]
    fn reports_the_class_of_a_code() {
        let code = SqlState::new("23514").unwrap();

        assert_eq!(code.class(), Class::IntegrityConstraintViolation);
        assert_eq!(
            format!("{:?}", code.class()),
            "Class(23 IntegrityConstraintViolation)",
        );
    }
}
