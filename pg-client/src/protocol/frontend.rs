//! `FunctionCall` is deliberately absent: upstream calls it "a legacy feature
//! that is probably best avoided in new code", replaceable by `Bind` and
//! `Execute`.

use core::num::NonZeroU32;

use crate::name::{Database, Portal, Statement, User};
use crate::parameter;
use crate::protocol::{Format, Oid, SendBuffer, Version, backend};

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum MessageError {
    #[error(transparent)]
    TooLong(#[from] TooLong),
    #[error("{count} entries exceed the Int16 count field")]
    TooMany { count: usize },
}

pub trait Message {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError>;
}

pub trait Parameter {
    fn write_to(&self, writer: &mut Writer<'_>);
}

impl Parameter for &[u8] {
    fn write_to(&self, writer: &mut Writer<'_>) {
        writer.bytes(self);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("{length} bytes exceed the Int32 length field")]
pub struct TooLong {
    pub length: usize,
}

#[derive(Debug)]
pub struct Writer<'a> {
    bytes: &'a mut Vec<u8>,
}

#[derive(Debug)]
#[must_use = "a reserved hole goes out as zeroes unless it is filled"]
pub struct Reserved<const N: usize> {
    at: usize,
}

impl<'a> Writer<'a> {
    pub(crate) const fn new(bytes: &'a mut Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl Writer<'_> {
    pub fn byte(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub fn int8(&mut self, value: i8) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub fn int16(&mut self, value: i16) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub fn int32(&mut self, value: i32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub fn int64(&mut self, value: i64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub fn uint16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub fn uint32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub fn uint64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    pub fn bytes(&mut self, value: &[u8]) {
        self.bytes.extend_from_slice(value);
    }

    pub fn reserve<const N: usize>(&mut self) -> Reserved<N> {
        let at = self.bytes.len();

        self.bytes.extend_from_slice(&[0; N]);

        Reserved { at }
    }

    #[must_use]
    pub fn written_since<const N: usize>(&self, hole: &Reserved<N>) -> usize {
        self.bytes.len() - hole.at - N
    }

    pub fn fill<const N: usize>(&mut self, hole: Reserved<N>, bytes: [u8; N]) {
        self.bytes[hole.at..hole.at + N].copy_from_slice(&bytes);
    }

    pub fn length_prefixed<E: From<TooLong>>(
        &mut self,
        value: impl FnOnce(&mut Writer<'_>) -> Result<(), E>,
    ) -> Result<(), E> {
        let length = self.reserve::<{ super::LENGTH_PREFIX_SIZE }>();

        value(self)?;

        let written = self.written_since(&length);
        let size = i32::try_from(written).map_err(|_| TooLong { length: written })?;

        self.fill(length, size.to_be_bytes());

        Ok(())
    }

    fn count16(&mut self, count: usize) -> Result<(), MessageError> {
        self.int16(i16::try_from(count).map_err(|_| MessageError::TooMany { count })?);

        Ok(())
    }

    pub fn string(&mut self, value: &super::String<'_>) {
        self.bytes.extend_from_slice(value.as_str().as_bytes());
        self.bytes.push(0);
    }
}

/// Which authentication response this tag names is decided by where the
/// exchange has got to, not by the byte.
const AUTHENTICATION: u8 = b'p';

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Replication {
    /// Physical replication.
    True,
    /// The server assumes this when the parameter is absent.
    False,
    /// Logical replication, connected to a database.
    Database,
}

impl Replication {
    fn as_protocol_string(self) -> &'static super::String<'static> {
        static TRUE: super::String<'static> = super::String::from_static_or_panic("true");
        static FALSE: super::String<'static> = super::String::from_static_or_panic("false");
        static DATABASE: super::String<'static> = super::String::from_static_or_panic("database");

        match self {
            Self::True => &TRUE,
            Self::False => &FALSE,
            Self::Database => &DATABASE,
        }
    }
}

/// [`StartupMessage`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-STARTUPMESSAGE): opens a session.
///
/// `user` is required and has no default. `database` defaults to the user name
/// when absent. `options` is deprecated upstream in favour of setting run-time
/// parameters individually, and is deliberately not offered.
///
/// Run-time parameters are applied at backend start and then act as session
/// defaults.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartupMessage<'a> {
    pub version: Version,
    pub user: &'a User<'a>,
    pub database: Option<&'a Database<'a>>,
    pub replication: Option<Replication>,
    pub parameters: &'a parameter::Map,
}

const USER: super::String<'static> = super::String::from_static_or_panic("user");
const DATABASE: super::String<'static> = super::String::from_static_or_panic("database");
const REPLICATION: super::String<'static> = super::String::from_static_or_panic("replication");

impl Message for StartupMessage<'_> {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.untagged(|writer| {
            writer.uint32(self.version.code());

            writer.string(&USER);
            writer.string(self.user.as_protocol_string());

            if let Some(database) = self.database {
                writer.string(&DATABASE);
                writer.string(database.as_protocol_string());
            }

            if let Some(replication) = self.replication {
                writer.string(&REPLICATION);
                writer.string(replication.as_protocol_string());
            }

            for (name, value) in self.parameters {
                writer.string(name.as_protocol_string());
                writer.string(value.as_protocol_string());
            }

            // An empty key closes the list.
            writer.byte(0);

            Ok(())
        })
    }
}

/// Holds 1234 in the high half and 5678 in the low, chosen so that it can never
/// collide with a protocol version number.
const CANCEL_REQUEST_CODE: i32 = 80_877_102;

/// [`CancelRequest`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-CANCELREQUEST):
/// asks the server to abort what a backend is running.
///
/// Goes out on a *new* connection, in place of [`StartupMessage`], because the
/// backend running the query is not reading its socket. That connection never
/// authenticates: the key is the credential, so whoever holds it can cancel,
/// including a process other than the one that asked the question.
///
/// Nothing comes back — the server acts and closes, deliberately, so that the
/// outcome cannot be probed. A caller learns what happened only by continuing
/// to read the original connection, where an effective cancel surfaces as an
/// error. It is best effort regardless: one that arrives after the query
/// finished does nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CancelRequest<'a> {
    pub key: &'a backend::BackendKeyData,
}

impl Message for CancelRequest<'_> {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.untagged(|writer| {
            writer.int32(CANCEL_REQUEST_CODE);
            writer.int32(self.key.process_id().get());
            writer.bytes(self.key.cancel_key().as_slice());

            Ok(())
        })
    }
}

/// [`Query`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-QUERY):
/// runs SQL through the simple query protocol.
///
/// The text may hold several statements separated by semicolons, which the
/// server runs in one implicit transaction and answers with one result group
/// per statement, closed by a single `ReadyForQuery`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Query<'a> {
    pub sql: &'a crate::Sql<'a>,
}

impl Message for Query<'_> {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(b'Q', |writer| {
            writer.string(self.sql.as_protocol_string());

            Ok(())
        })
    }
}

/// [`SASLInitialResponse`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-SASLINITIALRESPONSE).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SaslInitialResponse<'a> {
    pub mechanism: crate::authentication::Mechanism,
    pub initial: Option<&'a [u8]>,
}

impl Message for SaslInitialResponse<'_> {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(AUTHENTICATION, |writer| {
            writer.string(self.mechanism.name());

            match self.initial {
                None => writer.int32(-1),
                Some(initial) => {
                    writer.int32(i32::try_from(initial.len()).map_err(|_| TooLong {
                        length: initial.len(),
                    })?);
                    writer.bytes(initial);
                }
            }

            Ok(())
        })
    }
}

/// [`SASLResponse`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-SASLRESPONSE): continues a SASL exchange.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SaslResponse<'a> {
    pub payload: &'a [u8],
}

impl Message for SaslResponse<'_> {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(AUTHENTICATION, |writer| {
            writer.bytes(self.payload);

            Ok(())
        })
    }
}

/// [`PasswordMessage`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-PASSWORDMESSAGE):
/// answers `AuthenticationCleartextPassword`.
#[derive(Clone, Copy, Debug)]
pub struct PasswordMessage<'a> {
    pub password: &'a crate::config::Password,
}

impl Message for PasswordMessage<'_> {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(AUTHENTICATION, |writer| {
            writer.bytes(self.password.as_str().as_bytes());
            writer.byte(0);

            Ok(())
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn encodes_a_startup_message() {
        let mut buffer = SendBuffer::new();

        let parameters = parameter::Map::from([(
            parameter::Name::from_static_or_panic("client_encoding"),
            parameter::Value::from_static_or_panic("UTF8"),
        )]);

        StartupMessage {
            version: Version::V3_2,
            user: &User::from_static_or_panic("someone"),
            database: Some(&Database::from_static_or_panic("somewhere")),
            replication: Some(Replication::Database),
            parameters: &parameters,
        }
        .write_to(&mut buffer)
        .unwrap();

        assert_eq!(
            buffer.bytes,
            [
                83i32.to_be_bytes().as_slice(),
                &0x0003_0002u32.to_be_bytes(),
                b"user\x00someone\x00",
                b"database\x00somewhere\x00",
                b"replication\x00database\x00",
                b"client_encoding\x00UTF8\x00",
                b"\x00",
            ]
            .concat(),
        );
    }

    #[test]
    fn encodes_a_cancel_request() {
        let mut buffer = SendBuffer::new();

        CancelRequest {
            key: &backend::BackendKeyData::new(
                backend::ProcessId::new(4242),
                backend::CancelKey::new(b"a longer key than 3.0 allowed").unwrap(),
            ),
        }
        .write_to(&mut buffer)
        .unwrap();

        assert_eq!(
            buffer.bytes,
            [
                41i32.to_be_bytes().as_slice(),
                &CANCEL_REQUEST_CODE.to_be_bytes(),
                &4242i32.to_be_bytes(),
                b"a longer key than 3.0 allowed",
            ]
            .concat(),
        );
    }

    #[test]
    fn encodes_a_sasl_initial_response() {
        let mut buffer = SendBuffer::new();

        SaslInitialResponse {
            mechanism: crate::authentication::Mechanism::ScramSha256,
            initial: Some(b"n,,n=,r=nonce"),
        }
        .write_to(&mut buffer)
        .unwrap();

        assert_eq!(
            buffer.bytes,
            [
                b"p".as_slice(),
                &35i32.to_be_bytes(),
                b"SCRAM-SHA-256\x00",
                &13i32.to_be_bytes(),
                b"n,,n=,r=nonce",
            ]
            .concat(),
        );
    }

    #[test]
    fn encodes_an_absent_initial_response_as_negative_one() {
        let mut buffer = SendBuffer::new();

        SaslInitialResponse {
            mechanism: crate::authentication::Mechanism::ScramSha256,
            initial: None,
        }
        .write_to(&mut buffer)
        .unwrap();

        assert_eq!(
            buffer.bytes,
            [
                b"p".as_slice(),
                &22i32.to_be_bytes(),
                b"SCRAM-SHA-256\x00",
                &(-1i32).to_be_bytes(),
            ]
            .concat(),
        );
    }

    #[test]
    fn limits_rows_to_what_the_int32_field_can_carry() {
        let largest = NonZeroU32::new(2_147_483_647).unwrap();
        let beyond = NonZeroU32::new(2_147_483_648).unwrap();

        assert_eq!(RowLimit::new(largest).map(RowLimit::get), Some(largest));
        assert_eq!(RowLimit::new(beyond), None);
    }

    #[test]
    fn refuses_more_parameter_types_than_the_count_field_holds_and_queues_nothing() {
        let mut buffer = SendBuffer::new();

        Sync.write_to(&mut buffer).unwrap();

        let queued = buffer.bytes.clone();

        let error = Parse {
            statement: &StatementName::Unnamed,
            sql: &crate::Sql::from_static_or_panic("SELECT 1"),
            parameter_types: &vec![Oid::NONE; 32_768],
        }
        .write_to(&mut buffer)
        .unwrap_err();

        assert_eq!(error, MessageError::TooMany { count: 32_768 });
        assert_eq!(buffer.bytes, queued);
    }

    #[test]
    fn writes_an_int4_parameter_as_four_big_endian_bytes() {
        let mut buffer = SendBuffer::new();
        let mut value = Vec::new();

        crate::value::Value::Int4(7)
            .write_to(&mut Writer::new(&mut value))
            .unwrap();

        Bind {
            portal: &PortalName::Unnamed,
            statement: &StatementName::Unnamed,
            parameter_formats: Formats::All(Format::Binary),
            parameters: &[Some(value.as_slice())],
            result_formats: Formats::Default,
        }
        .write_to(&mut buffer)
        .unwrap();

        assert_eq!(
            buffer.bytes,
            [
                b"B".as_slice(),
                &22i32.to_be_bytes(),
                b"\x00",
                b"\x00",
                &1i16.to_be_bytes(),
                &1i16.to_be_bytes(),
                &1i16.to_be_bytes(),
                &4i32.to_be_bytes(),
                &7i32.to_be_bytes(),
                &0i16.to_be_bytes(),
            ]
            .concat(),
        );
    }
}

/// The unnamed statement is replaced by the next unnamed `Parse`, where a named
/// one lives until it is closed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatementName {
    Unnamed,
    Named(Statement<'static>),
}

impl StatementName {
    fn as_protocol_string(&self) -> &super::String<'static> {
        match self {
            Self::Unnamed => &super::String::EMPTY,
            Self::Named(name) => name.as_protocol_string(),
        }
    }
}

/// The unnamed portal is replaced by the next unnamed `Bind`, and every portal
/// dies when its transaction ends, so a portal outliving a `Sync` needs an
/// explicit transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PortalName {
    Unnamed,
    Named(Portal<'static>),
}

impl PortalName {
    fn as_protocol_string(&self) -> &super::String<'static> {
        match self {
            Self::Unnamed => &super::String::EMPTY,
            Self::Named(name) => name.as_protocol_string(),
        }
    }
}

/// Statements and portals are separate namespaces on the server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    Statement(StatementName),
    Portal(PortalName),
}

impl Target {
    const fn code(&self) -> u8 {
        match self {
            Self::Statement(_) => b'S',
            Self::Portal(_) => b'P',
        }
    }

    fn name(&self) -> &super::String<'static> {
        match self {
            Self::Statement(name) => name.as_protocol_string(),
            Self::Portal(name) => name.as_protocol_string(),
        }
    }

    fn encode(&self, writer: &mut Writer<'_>) {
        writer.byte(self.code());
        writer.string(self.name());
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Formats<'a> {
    /// The server uses text for everything.
    Default,
    All(Format),
    Each(&'a [Format]),
}

impl Formats<'_> {
    fn encode(&self, writer: &mut Writer<'_>) -> Result<(), MessageError> {
        match self {
            Self::Default => writer.int16(0),
            Self::All(format) => {
                writer.int16(1);
                writer.int16(format.code());
            }
            Self::Each(formats) => {
                writer.count16(formats.len())?;

                for format in *formats {
                    writer.int16(format.code());
                }
            }
        }

        Ok(())
    }
}

/// [`Parse`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-PARSE).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parse<'a> {
    pub statement: &'a StatementName,
    pub sql: &'a crate::Sql<'a>,
    /// This need not cover every parameter in the query, and [`Oid::NONE`]
    /// leaves one for the server to infer.
    pub parameter_types: &'a [Oid],
}

impl Message for Parse<'_> {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(b'P', |writer| {
            writer.string(self.statement.as_protocol_string());
            writer.string(self.sql.as_protocol_string());
            writer.count16(self.parameter_types.len())?;

            for oid in self.parameter_types {
                writer.uint32(oid.get());
            }

            Ok(())
        })
    }
}

/// [`Bind`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-BIND).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bind<'a, P = &'a [u8]> {
    pub portal: &'a PortalName,
    pub statement: &'a StatementName,
    pub parameter_formats: Formats<'a>,
    /// `None` is NULL, which PostgreSQL treats as distinct from an empty value.
    pub parameters: &'a [Option<P>],
    pub result_formats: Formats<'a>,
}

impl<P: Parameter> Message for Bind<'_, P> {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(b'B', |writer| {
            writer.string(self.portal.as_protocol_string());
            writer.string(self.statement.as_protocol_string());

            self.parameter_formats.encode(writer)?;

            writer.count16(self.parameters.len())?;

            for parameter in self.parameters {
                match parameter {
                    None => writer.int32(-1),
                    Some(value) => writer.length_prefixed(|writer| {
                        value.write_to(writer);

                        Ok::<(), MessageError>(())
                    })?,
                }
            }

            self.result_formats.encode(writer)
        })
    }
}

/// [`Describe`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-DESCRIBE).
///
/// Describing a statement answers `ParameterDescription` then `RowDescription`;
/// describing a portal answers `RowDescription` alone. `Bind` reports no shape
/// of its own, which is why this exists.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Describe<'a> {
    pub target: &'a Target,
}

impl Message for Describe<'_> {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(b'D', |writer| {
            self.target.encode(writer);

            Ok(())
        })
    }
}

/// [`Execute`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-EXECUTE).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Execute<'a> {
    pub portal: &'a PortalName,
    /// A limit that is reached answers `PortalSuspended` instead of
    /// `CommandComplete`, and the portal can be executed again to continue.
    pub max_rows: Option<RowLimit>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RowLimit(NonZeroU32);

impl RowLimit {
    #[must_use]
    pub const fn new(rows: NonZeroU32) -> Option<Self> {
        if rows.get() <= i32::MAX.unsigned_abs() {
            Some(Self(rows))
        } else {
            None
        }
    }

    #[must_use]
    pub const fn get(self) -> NonZeroU32 {
        self.0
    }
}

impl Message for Execute<'_> {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(b'E', |writer| {
            writer.string(self.portal.as_protocol_string());
            writer.uint32(self.max_rows.map_or(0, |limit| limit.get().get()));

            Ok(())
        })
    }
}

/// [`Close`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-CLOSE).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Close<'a> {
    pub target: &'a Target,
}

impl Message for Close<'_> {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(b'C', |writer| {
            self.target.encode(writer);

            Ok(())
        })
    }
}

/// [`CopyData`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-COPYDATA):
/// part of a copy stream.
///
/// Sent after a `CopyInResponse`. The boundaries are free: upstream requires
/// nothing of them, so a message need not hold whole rows, and splitting on
/// rows is a convenience rather than a rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CopyData<'a> {
    pub data: &'a [u8],
}

impl Message for CopyData<'_> {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(b'd', |writer| {
            writer.bytes(self.data);

            Ok(())
        })
    }
}

/// [`CopyDone`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-COPYDONE).
///
/// The server then answers `CommandComplete`, or `ErrorResponse` if the data
/// did not satisfy the table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CopyDone;

impl Message for CopyDone {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(b'c', |_| Ok(()))
    }
}

/// [`CopyFail`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-COPYFAIL):
/// abandons a copy stream.
///
/// While copy-in lasts the server ignores [`Flush`] and [`Sync`] and treats
/// any other message as an error, which aborts the copy anyway but on the
/// server's terms rather than the caller's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CopyFail<'a> {
    pub message: &'a super::String<'a>,
}

impl Message for CopyFail<'_> {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(b'f', |writer| {
            writer.string(self.message);

            Ok(())
        })
    }
}

/// [`Flush`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-FLUSH):
/// pushes pending output without ending the batch.
///
/// Not a boundary: it creates no recovery point, ends no implicit transaction,
/// and produces no `ReadyForQuery`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Flush;

impl Message for Flush {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(b'H', |_| Ok(()))
    }
}

/// [`Sync`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-SYNC):
/// closes a batch.
///
/// Three things at once: it ends the implicit transaction, marks where the
/// server resumes after an error, and asks for `ReadyForQuery`. Where it goes
/// therefore decides what is atomic, so nothing places one on a caller's
/// behalf.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sync;

impl Message for Sync {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(b'S', |_| Ok(()))
    }
}

/// [`Terminate`](https://www.postgresql.org/docs/current/protocol-message-formats.html#PROTOCOL-MESSAGE-FORMATS-TERMINATE).
///
/// Open transactions roll back whether or not this is sent, so it buys an
/// orderly close rather than a different outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Terminate;

impl Message for Terminate {
    fn write_to(&self, buffer: &mut SendBuffer) -> Result<(), MessageError> {
        buffer.message(b'X', |_| Ok(()))
    }
}
