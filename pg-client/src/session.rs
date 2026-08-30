pub mod copy_in;
pub mod copy_out;
pub mod statement;
pub mod statements;
pub mod transaction;

use std::sync::Arc;

use crate::name::Database;
use crate::protocol::{backend, frontend};
use crate::{authentication, config, connection, parameter, protocol};

#[derive(Clone, Debug)]
pub struct Config {
    pub connection: connection::Config,
    pub authentication: authentication::Config,
    pub database: Database<'static>,
    pub parameters: Parameters,
}

impl Config {
    #[must_use]
    pub fn new(
        connection: connection::Config,
        authentication: authentication::Config,
        database: Database<'static>,
    ) -> Self {
        Self {
            connection,
            authentication,
            database,
            parameters: Parameters::new(),
        }
    }

    #[must_use]
    pub fn with_parameters(self, parameters: Parameters) -> Self {
        Self { parameters, ..self }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("a dynamic password could not be resolved")]
pub struct StaticPasswordError(#[source] authentication::PasswordError);

impl Config {
    pub async fn with_static_passwords(
        &self,
    ) -> Result<std::borrow::Cow<'_, Self>, StaticPasswordError> {
        let methods = &self.authentication.methods;

        let dynamic = [
            methods.scram.as_ref().map(|scram| &scram.password),
            methods
                .cleartext_password
                .as_ref()
                .map(|cleartext| &cleartext.password),
        ]
        .into_iter()
        .flatten()
        .any(|password| matches!(password, authentication::Password::Dynamic(_)));

        if !dynamic {
            return Ok(std::borrow::Cow::Borrowed(self));
        }

        let mut config = self.clone();
        let methods = &mut config.authentication.methods;

        // Methods sharing one source share one value: two calls could mint two
        // different one-time tokens for what is a single credential.
        let mut resolved: Option<(Arc<dyn authentication::PasswordSource>, config::Password)> =
            None;

        for password in [
            methods.scram.as_mut().map(|scram| &mut scram.password),
            methods
                .cleartext_password
                .as_mut()
                .map(|cleartext| &mut cleartext.password),
        ]
        .into_iter()
        .flatten()
        {
            let authentication::Password::Dynamic(source) = password else {
                continue;
            };

            let value = match &resolved {
                Some((shared, value)) if Arc::ptr_eq(shared, source) => value.clone(),
                _ => {
                    let value = source.password().await.map_err(StaticPasswordError)?;

                    resolved = Some((Arc::clone(source), value.clone()));

                    value
                }
            };

            *password = authentication::Password::Static(value);
        }

        Ok(std::borrow::Cow::Owned(config))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Parameters(parameter::Map);

/// The server would silently rewrite `application_name` it cannot keep, so it
/// goes through [`Parameters::set_application_name`] and its checked type.
/// pg-client decodes text as UTF-8 and sets `client_encoding` itself.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ParametersError {
    #[error("{name} has a dedicated setter")]
    Dedicated { name: parameter::Name },
    #[error("{name} is set by pg-client")]
    Forced { name: parameter::Name },
}

impl Parameters {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        name: parameter::Name,
        value: parameter::Value,
    ) -> Result<Option<parameter::Value>, ParametersError> {
        if name
            .as_str()
            .eq_ignore_ascii_case(APPLICATION_NAME.as_str())
        {
            Err(ParametersError::Dedicated { name })
        } else if name.as_str().eq_ignore_ascii_case(CLIENT_ENCODING.as_str()) {
            Err(ParametersError::Forced { name })
        } else {
            Ok(self.0.insert(name, value))
        }
    }

    pub fn set_application_name(&mut self, application_name: crate::name::ApplicationName) {
        self.0
            .insert(APPLICATION_NAME, application_name.into_value());
    }

    #[must_use]
    pub fn get(&self, name: &parameter::Name) -> Option<&parameter::Value> {
        self.0.get(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&parameter::Name, &parameter::Value)> {
        self.0.iter()
    }
}

const APPLICATION_NAME: parameter::Name =
    parameter::Name::from_static_or_panic(parameter::APPLICATION_NAME);

#[derive(Debug, thiserror::Error)]
pub enum OpenError {
    #[error("could not start the session")]
    Start(#[from] StartError),
    #[error("could not terminate the session")]
    Terminate(#[from] TerminateError),
}

pub async fn with_session<T>(
    config: Arc<Config>,
    action: impl AsyncFnOnce(&mut Session) -> T,
) -> Result<T, OpenError> {
    let mut session = start(config).await?;

    let result = action(&mut session).await;

    session.send_terminate().await?;

    Ok(result)
}

pub const CLIENT_ENCODING: parameter::Name =
    parameter::Name::from_static_or_panic("client_encoding");

pub const UTF8: parameter::Value = parameter::Value::from_static_or_panic("UTF8");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    CommandProcessing,
    CopyIn,
    CopyOut,
    CopyBoth,
}

#[derive(Debug)]
pub struct Session {
    version: protocol::Version,
    connection: connection::Connection,
    parameters: parameter::RuntimeParameters,
    transaction_status: backend::TransactionStatus,
    backend_key: backend::BackendKeyData,
    mode: Mode,
    // Exchanges sent whose ReadyForQuery has not been read: one per Sync or
    // Query, in wire order. A reader drains all but its own, which is last.
    outstanding: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("not legal in {mode:?} mode")]
pub struct ModeError {
    pub mode: Mode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum QueueError {
    #[error(transparent)]
    Mode(#[from] ModeError),
    #[error("message cannot be encoded: {0}")]
    Message(#[from] frontend::MessageError),
    #[error("the connection is closed")]
    Closed,
}

#[derive(Debug, thiserror::Error)]
pub enum TerminateError {
    #[error(transparent)]
    Queue(#[from] QueueError),
    #[error("writing the terminate: {0}")]
    Send(#[from] protocol::SendError),
}

#[derive(Debug, thiserror::Error)]
pub enum CopyDataError {
    #[error(transparent)]
    Queue(#[from] QueueError),
    #[error("writing copy data: {0}")]
    Send(#[from] protocol::SendError),
}

#[derive(Debug, thiserror::Error)]
pub enum ReceiveError {
    #[error("writing buffered messages: {0}")]
    Send(#[from] protocol::SendError),
    #[error("reading a frame: {0}")]
    Read(#[from] protocol::ReceiveError),
    #[error("decoding a message: {0}")]
    Decode(#[from] backend::DecodeError),
    #[error("reading a parameter status: {0}")]
    ParameterError(#[from] parameter::RuntimeParameterError),
    #[error("refusing COPY FROM STDIN: {0}")]
    RefuseCopyIn(#[source] frontend::MessageError),
    #[error("an abandoned COPY FROM STDIN left the connection unusable, so it was closed")]
    UnrequestedCopyIn,
    #[error("server broke the protocol: {0}")]
    ProtocolViolation(Violation),
}

#[derive(Debug, thiserror::Error)]
pub enum Violation {
    #[error("ready for a query that was never sent")]
    UnsolicitedReadyForQuery,
    #[error("ready for a query while still in {mode:?} mode")]
    ReadyForQueryDuringCopy { mode: Mode },
    #[error("unexpected message: {}", statement::describe(.0))]
    UnexpectedMessage(protocol::Frame),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreparedStatement {
    name: frontend::StatementName,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Portal {
    name: frontend::PortalName,
}

impl Session {
    #[must_use]
    pub const fn parameters(&self) -> &parameter::RuntimeParameters {
        &self.parameters
    }

    // TODO: a cancel method: send `CancelRequest` with this key over a new
    // connection to the same endpoint, usable while a statement runs.
    #[must_use]
    pub const fn backend_key(&self) -> &backend::BackendKeyData {
        &self.backend_key
    }

    #[must_use]
    pub const fn version(&self) -> protocol::Version {
        self.version
    }

    #[must_use]
    pub const fn transaction_status(&self) -> backend::TransactionStatus {
        self.transaction_status
    }

    // The last ReadyForQuery only speaks for the session while nothing sent
    // after it is still unanswered.
    const fn is_known_idle(&self) -> bool {
        self.outstanding == 0 && matches!(self.transaction_status, backend::TransactionStatus::Idle)
    }

    #[must_use]
    pub const fn mode(&self) -> Mode {
        self.mode
    }

    // A server streaming COPY TO output reads nothing until its ReadyForQuery
    // (copyto.c only sends; postgres.c reads the next command after the query
    // returns), so commands sent meanwhile queue up in order behind it.
    fn queue_send_in(
        &mut self,
        mode: Mode,
        message: impl frontend::Message,
    ) -> Result<(), QueueError> {
        if self.connection.is_closed() {
            return Err(QueueError::Closed);
        }

        let sendable = match mode {
            Mode::CommandProcessing => {
                matches!(self.mode, Mode::CommandProcessing | Mode::CopyOut)
            }
            required => self.mode == required,
        };

        if sendable {
            Ok(self.connection.queue_send(message)?)
        } else {
            Err(ModeError { mode: self.mode }.into())
        }
    }

    pub fn queue_parse(
        &mut self,
        name: frontend::StatementName,
        sql: &crate::Sql<'_>,
        parameter_types: &[protocol::Oid],
    ) -> Result<PreparedStatement, QueueError> {
        self.queue_send_in(
            Mode::CommandProcessing,
            frontend::Parse {
                statement: &name,
                sql,
                parameter_types,
            },
        )?;

        Ok(PreparedStatement { name })
    }

    pub fn queue_bind<P: frontend::Parameter>(
        &mut self,
        statement: &PreparedStatement,
        name: frontend::PortalName,
        parameter_formats: frontend::Formats<'_>,
        parameters: &[Option<P>],
        result_formats: frontend::Formats<'_>,
    ) -> Result<Portal, QueueError> {
        self.queue_send_in(
            Mode::CommandProcessing,
            frontend::Bind {
                portal: &name,
                statement: &statement.name,
                parameter_formats,
                parameters,
                result_formats,
            },
        )?;

        Ok(Portal { name })
    }

    pub fn queue_describe_statement(
        &mut self,
        statement: &PreparedStatement,
    ) -> Result<(), QueueError> {
        self.queue_send_in(
            Mode::CommandProcessing,
            frontend::Describe {
                target: &frontend::Target::Statement(statement.name.clone()),
            },
        )
    }

    pub fn queue_close_statement(
        &mut self,
        statement: PreparedStatement,
    ) -> Result<(), QueueError> {
        self.queue_send_in(
            Mode::CommandProcessing,
            frontend::Close {
                target: &frontend::Target::Statement(statement.name),
            },
        )
    }

    pub fn queue_execute(
        &mut self,
        portal: &Portal,
        max_rows: Option<frontend::RowLimit>,
    ) -> Result<(), QueueError> {
        self.queue_send_in(
            Mode::CommandProcessing,
            frontend::Execute {
                portal: &portal.name,
                max_rows,
            },
        )
    }

    pub fn queue_describe_portal(&mut self, portal: &Portal) -> Result<(), QueueError> {
        self.queue_send_in(
            Mode::CommandProcessing,
            frontend::Describe {
                target: &frontend::Target::Portal(portal.name.clone()),
            },
        )
    }

    pub fn queue_close_portal(&mut self, portal: Portal) -> Result<(), QueueError> {
        self.queue_send_in(
            Mode::CommandProcessing,
            frontend::Close {
                target: &frontend::Target::Portal(portal.name),
            },
        )
    }

    pub fn queue_sync(&mut self) -> Result<(), QueueError> {
        self.queue_send_in(Mode::CommandProcessing, frontend::Sync)?;
        self.outstanding += 1;

        Ok(())
    }

    pub fn queue_query(&mut self, sql: &crate::Sql<'_>) -> Result<(), QueueError> {
        self.queue_send_in(Mode::CommandProcessing, frontend::Query { sql })?;
        self.outstanding += 1;

        Ok(())
    }

    pub async fn send_terminate(mut self) -> Result<(), TerminateError> {
        self.queue_send_in(Mode::CommandProcessing, frontend::Terminate)?;

        Ok(self.connection.flush_send().await?)
    }

    pub fn queue_flush(&mut self) -> Result<(), QueueError> {
        self.queue_send_in(Mode::CommandProcessing, frontend::Flush)
    }

    // A server that breaks the protocol leaves a stream nothing can be placed
    // in, so the connection is dropped and every later call fails on it.
    fn violation(&mut self, violation: Violation) -> ReceiveError {
        self.connection.close();

        ReceiveError::ProtocolViolation(violation)
    }

    pub async fn receive(&mut self) -> Result<protocol::Frame, ReceiveError> {
        self.connection.flush_send().await?;

        let frame = self.connection.receive().await?;
        let message = backend::Message::decode(&frame)?;

        // The server exits after FATAL (elog.c, proc_exit) and PANIC (abort),
        // so the connection is closed here too; the reader still gets the
        // error itself.
        if let backend::Message::ErrorResponse(response) = &message
            && response.diagnostic.severity().ends_session()
        {
            self.connection.close();

            return Ok(frame);
        }

        match (self.mode, message) {
            (_, backend::Message::ParameterStatus(status)) => self.parameters.apply(status)?,
            (Mode::CommandProcessing, backend::Message::CopyInResponse(_)) => {
                self.mode = Mode::CopyIn;
            }
            (Mode::CommandProcessing, backend::Message::CopyOutResponse(_)) => {
                self.mode = Mode::CopyOut;
            }
            (Mode::CommandProcessing, backend::Message::CopyBothResponse(_)) => {
                self.mode = Mode::CopyBoth;
            }
            (Mode::CommandProcessing, backend::Message::ReadyForQuery(status)) => {
                let Some(outstanding) = self.outstanding.checked_sub(1) else {
                    return Err(self.violation(Violation::UnsolicitedReadyForQuery));
                };

                self.transaction_status = status;
                self.outstanding = outstanding;
            }
            (
                mode @ (Mode::CopyIn | Mode::CopyOut | Mode::CopyBoth),
                backend::Message::ReadyForQuery(_),
            ) => {
                return Err(self.violation(Violation::ReadyForQueryDuringCopy { mode }));
            }
            // The server is done sending; `CommandComplete` follows.
            (Mode::CopyOut, backend::Message::CopyDone) => self.mode = Mode::CommandProcessing,
            (Mode::CopyBoth, backend::Message::CopyDone) => self.mode = Mode::CopyIn,
            (Mode::CopyIn | Mode::CopyOut | Mode::CopyBoth, backend::Message::ErrorResponse(_)) => {
                self.mode = Mode::CommandProcessing;
            }
            _ => (),
        }

        Ok(frame)
    }

    pub async fn send_copy_data(&mut self, data: &[u8]) -> Result<(), CopyDataError> {
        self.queue_send_in(Mode::CopyIn, frontend::CopyData { data })?;

        Ok(self.connection.flush_send().await?)
    }

    pub fn queue_copy_done(&mut self) -> Result<(), QueueError> {
        self.queue_send_in(Mode::CopyIn, frontend::CopyDone)?;

        self.mode = Mode::CommandProcessing;

        Ok(())
    }

    pub fn queue_copy_fail(&mut self, message: &protocol::String<'_>) -> Result<(), QueueError> {
        self.queue_send_in(Mode::CopyIn, frontend::CopyFail { message })?;

        self.mode = Mode::CommandProcessing;

        Ok(())
    }

    pub fn statement<'q>(&mut self, sql: &crate::Sql<'q>) -> statement::Builder<'q, &mut Self> {
        statement::Builder::new(self, sql)
    }

    pub fn statements<'q>(&mut self, sql: &crate::Sql<'q>) -> statements::Builder<'q, &mut Self> {
        statements::Builder::new(self, sql)
    }

    pub fn transaction(&mut self) -> transaction::Builder<&mut Self> {
        transaction::Builder::new(self)
    }

    // Reads the reply to one queued Query, up to its ReadyForQuery.
    async fn read_statements_reply(&mut self) -> Result<(), statement::Error> {
        let mut error = None;

        loop {
            let frame = self.receive().await?;

            if self.mode == Mode::CopyIn {
                self.refuse_copy_in()?;
            }

            let failure = match backend::Message::decode(&frame).map_err(ReceiveError::Decode)? {
                backend::Message::ReadyForQuery(_) => break,
                backend::Message::ErrorResponse(response) => {
                    let failure = statement::Error::Server(ServerError(frame.clone()));

                    if response.diagnostic.severity().ends_session() {
                        return Err(error.unwrap_or(failure));
                    }

                    Some(failure)
                }
                backend::Message::NoticeResponse(_)
                | backend::Message::NotificationResponse(_)
                | backend::Message::ParameterStatus(_)
                | backend::Message::RowDescription(_)
                | backend::Message::DataRow(_)
                | backend::Message::CommandComplete(_)
                | backend::Message::EmptyQueryResponse
                | backend::Message::CopyData(_)
                | backend::Message::CopyDone => None,
                backend::Message::CopyInResponse(_)
                | backend::Message::CopyOutResponse(_)
                | backend::Message::CopyBothResponse(_) => {
                    Some(statement::Error::WrongStatement(frame.clone()))
                }
                _ => {
                    return Err(self.violation(Violation::UnexpectedMessage(frame)).into());
                }
            };

            if error.is_none() {
                error = failure;
            }
        }

        error.map_or(Ok(()), Err)
    }

    pub fn copy_out<'q>(&mut self, sql: &crate::Sql<'q>) -> copy_out::Builder<'q, &mut Self> {
        copy_out::Builder::new(self, sql)
    }

    pub fn copy_in<'q>(&mut self, sql: &crate::Sql<'q>) -> copy_in::Builder<'q, &mut Self> {
        copy_in::Builder::new(self, sql)
    }

    // Called once the reader's own exchange is queued, so it is the last one
    // outstanding and everything ahead of it belongs to runs given up earlier.
    async fn drain_abandoned(&mut self, parent: &crate::trace::Span) -> Result<(), ReceiveError> {
        self.drain_abandoned_before(parent, 1).await
    }

    // Like `drain_abandoned`, for a reader that queued `own` exchanges.
    async fn drain_abandoned_before(
        &mut self,
        parent: &crate::trace::Span,
        own: usize,
    ) -> Result<(), ReceiveError> {
        use crate::trace::Instrument as _;

        if self.outstanding > own {
            self.read_until_outstanding(own)
                .instrument(crate::trace::debug_span!(parent: parent, "pg.drain"))
                .await?;
        }

        Ok(())
    }

    async fn read_to_ready(&mut self) -> Result<(), ReceiveError> {
        self.read_until_outstanding(0).await
    }

    async fn read_until_outstanding(&mut self, remaining: usize) -> Result<(), ReceiveError> {
        while self.outstanding > remaining {
            self.receive().await?;

            if self.mode == Mode::CopyIn {
                self.close_for_copy_in();

                return Err(ReceiveError::UnrequestedCopyIn);
            }
        }

        Ok(())
    }

    // During COPY FROM STDIN the server ignores Sync and fails on any other
    // message, consuming it either way:
    // https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/copyfromparse.c (CopyGetData)
    // Once anything may have followed the COPY onto the wire, there is no
    // telling how many replies the server still owes.
    fn close_for_copy_in(&mut self) {
        self.connection.close();
    }

    fn refuse_copy_in(&mut self) -> Result<(), ReceiveError> {
        const MESSAGE: protocol::String<'static> =
            protocol::String::from_static_or_panic("COPY FROM STDIN is not supported");

        self.connection
            .queue_send(frontend::CopyFail { message: &MESSAGE })
            .map_err(ReceiveError::RefuseCopyIn)?;

        self.mode = Mode::CommandProcessing;

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StartError {
    #[error("a startup message cannot be encoded")]
    Encode(#[source] frontend::MessageError),
    #[error("could not reach the server")]
    Connect(#[source] std::io::Error),
    #[error("TLS could not be established")]
    Tls(#[source] connection::TlsError),
    #[error("server presented no certificate")]
    NoCertificate,
    #[error("channel binding data could not be derived")]
    ChannelBinding(#[source] authentication::scram::ChannelBindingError),
    #[error("channel binding is required and the server did not offer it")]
    ChannelBindingRefused,
    #[error("could not obtain a password")]
    PasswordSourcing(#[source] authentication::PasswordError),
    #[error("server sends {requested:?}, which no configured authentication method answers")]
    MethodNotConfigured {
        requested: backend::AuthenticationRequest,
    },
    #[error("channel binding is required and the connection has no TLS")]
    ChannelBindingWithoutTls,
    #[error("server asks for the password in cleartext over unencrypted TCP")]
    CleartextOverUnencryptedTcp,
    #[error("server offers no SCRAM mechanism")]
    NoSupportedMechanism,
    #[error("SCRAM exchange failed")]
    Scram(#[source] authentication::scram::Error),
    #[error("SCRAM key derivation did not complete")]
    KeyDerivation(#[source] tokio::task::JoinError),
    #[error("server offers protocol version code {code:#010x}, and this client speaks 3.0 and 3.2")]
    UnsupportedProtocol { code: u32 },
    #[error("server rejected the connection: {0}")]
    Rejected(ServerError),
    #[error("unexpected message during startup: {message}")]
    Unexpected { message: String },
    #[error("server sent no cancellation key")]
    NoCancellationKey,
    #[error("server did not report a parameter this client can read")]
    ParameterError(#[source] parameter::RuntimeParameterError),
    #[error("could not send")]
    Send(#[source] protocol::SendError),
    #[error("could not receive")]
    Receive(#[source] protocol::ReceiveError),
    #[error("could not decode a message")]
    Decode(#[source] backend::DecodeError),
}

#[derive(Clone)]
pub struct ServerError(protocol::Frame);

impl ServerError {
    #[must_use]
    pub fn diagnostic(&self) -> backend::Diagnostic<'_> {
        let Ok(backend::Message::ErrorResponse(response)) = backend::Message::decode(&self.0)
        else {
            unreachable!("a ServerError is only built from a decoded ErrorResponse");
        };

        response.diagnostic
    }
}

impl core::fmt::Debug for ServerError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_tuple("ServerError")
            .field(&self.diagnostic())
            .finish()
    }
}

impl core::fmt::Display for ServerError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let diagnostic = self.diagnostic();

        formatter.write_str(diagnostic.message())?;

        if let Some(detail) = diagnostic.field(backend::FieldType::DETAIL) {
            write!(formatter, " (detail: {detail})")?;
        }

        if let Some(hint) = diagnostic.field(backend::FieldType::HINT) {
            write!(formatter, " (hint: {hint})")?;
        }

        Ok(())
    }
}

impl std::error::Error for ServerError {}

// Handing a session back is the source's business: a shared one is only
// unlocked, a fresh one is closed with Terminate so the server does not log
// an unexpected end of the connection.
pub trait Source {
    type Handle: core::ops::DerefMut<Target = Session>;
    type Error;
    type ReleaseError;

    fn acquire(&self) -> impl Future<Output = Result<Self::Handle, Self::Error>> + Send;

    fn release(
        &self,
        handle: Self::Handle,
    ) -> impl Future<Output = Result<(), Self::ReleaseError>> + Send;
}

impl Source for Arc<tokio::sync::Mutex<Session>> {
    type Handle = tokio::sync::OwnedMutexGuard<Session>;
    type Error = core::convert::Infallible;
    type ReleaseError = core::convert::Infallible;

    fn acquire(&self) -> impl Future<Output = Result<Self::Handle, Self::Error>> + Send {
        let shared = Arc::clone(self);

        async move { Ok(shared.lock_owned().await) }
    }

    fn release(
        &self,
        handle: Self::Handle,
    ) -> impl Future<Output = Result<(), Self::ReleaseError>> + Send {
        drop(handle);

        core::future::ready(Ok(()))
    }
}

impl Source for Arc<Config> {
    type Handle = Box<Session>;
    type Error = StartError;
    type ReleaseError = TerminateError;

    fn acquire(&self) -> impl Future<Output = Result<Self::Handle, Self::Error>> + Send {
        let config = Arc::clone(self);

        async move { start(config).await.map(Box::new) }
    }

    fn release(
        &self,
        handle: Self::Handle,
    ) -> impl Future<Output = Result<(), Self::ReleaseError>> + Send {
        (*handle).send_terminate()
    }
}

pub async fn start(config: Arc<Config>) -> Result<Session, StartError> {
    let mut parameters = config.parameters.0.clone();
    parameters.insert(CLIENT_ENCODING, UTF8);

    let (stream, certificate) = establish(&config.connection).await?;

    let mut connection = connection::Connection::new(stream, config.connection.receive_buffer);

    connection
        .queue_send(frontend::StartupMessage {
            version: protocol::Version::V3_2,
            user: &config.authentication.user,
            database: Some(&config.database),
            replication: None,
            parameters: &parameters,
        })
        .map_err(StartError::Encode)?;

    connection.flush_send().await.map_err(StartError::Send)?;

    let version = authenticate(&mut connection, &config, certificate).await?;

    let settled = settle(&mut connection).await?;

    Ok(Session {
        version,
        connection,
        parameters: settled.parameters,
        transaction_status: settled.transaction_status,
        backend_key: settled.backend_key,
        mode: Mode::CommandProcessing,
        outstanding: 0,
    })
}

async fn establish(
    config: &connection::Config,
) -> Result<
    (
        connection::Stream,
        Option<authentication::scram::ChannelBindingData>,
    ),
    StartError,
> {
    let (host, address, port, tls) = match &config.endpoint {
        connection::Endpoint::Unix { path } => {
            let socket = tokio::net::UnixStream::connect(path)
                .await
                .map_err(StartError::Connect)?;

            return Ok((connection::Stream::Unix(socket), None));
        }
        connection::Endpoint::Tcp {
            host,
            address,
            port,
            tls,
        } => (host, address, port, tls),
    };

    let port = u16::from(port);

    let socket = match (address, host) {
        (Some(address), _) => {
            tokio::net::TcpStream::connect((std::net::IpAddr::from(address), port)).await
        }
        (None, config::Host::IpAddr(address)) => {
            tokio::net::TcpStream::connect((*address, port)).await
        }
        (None, config::Host::HostName(name)) => {
            tokio::net::TcpStream::connect((name.as_str(), port)).await
        }
    }
    .map_err(StartError::Connect)?;

    let Some(tls) = tls else {
        return Ok((connection::Stream::Tcp(socket), None));
    };

    let stream = connection::Stream::tls(socket, Arc::clone(tls.client()), server_name(host))
        .await
        .map_err(StartError::Tls)?;

    let certificate = stream.peer_certificate().ok_or(StartError::NoCertificate)?;

    let binding = authentication::scram::ChannelBindingData::from_certificate(certificate)
        .map_err(StartError::ChannelBinding)?;

    Ok((stream, Some(binding)))
}

async fn authenticate(
    connection: &mut connection::Connection,
    config: &Config,
    certificate: Option<authentication::scram::ChannelBindingData>,
) -> Result<protocol::Version, StartError> {
    let mut frame = connection
        .receive_authentication()
        .await
        .map_err(StartError::Receive)?;

    let negotiated = match backend::AuthenticationMessage::decode(&frame) {
        Ok(backend::AuthenticationMessage::NegotiateProtocolVersion(negotiated)) => {
            Some(negotiated.version)
        }
        Ok(_) => None,
        Err(backend::DecodeError::UnsupportedProtocolVersion { code }) => {
            return Err(StartError::UnsupportedProtocol { code });
        }
        Err(error) => return Err(StartError::Decode(error)),
    };

    let version = match negotiated {
        Some(version) => {
            frame = connection
                .receive_authentication()
                .await
                .map_err(StartError::Receive)?;
            version
        }
        None => protocol::Version::V3_2,
    };

    let methods = &config.authentication.methods;

    let (scram, mechanisms) = match backend::AuthenticationMessage::decode(&frame)
        .map_err(StartError::Decode)?
    {
        // Unchallenged: nothing to prove, and nothing more to read.
        backend::AuthenticationMessage::Authentication(backend::Authentication::Ok)
            if methods.accept_unchallenged =>
        {
            return Ok(version);
        }
        backend::AuthenticationMessage::Authentication(backend::Authentication::Sasl {
            mechanisms,
        }) if let Some(scram) = &methods.scram => {
            (scram, mechanisms.into_iter().collect::<Vec<_>>())
        }
        backend::AuthenticationMessage::Authentication(
            backend::Authentication::CleartextPassword,
        ) if let Some(cleartext) = &methods.cleartext_password => {
            let confidential = certificate.is_some()
                || matches!(
                    config.connection.endpoint,
                    connection::Endpoint::Unix { .. }
                );

            return send_cleartext_password(connection, cleartext, confidential)
                .await
                .map(|()| version);
        }
        backend::AuthenticationMessage::Authentication(request) => {
            return Err(StartError::MethodNotConfigured {
                requested: request.request(),
            });
        }
        backend::AuthenticationMessage::ErrorResponse(_) => return Err(rejected(frame.clone())),
        // Admitted, but only as the first message.
        other @ backend::AuthenticationMessage::NegotiateProtocolVersion(_) => {
            return Err(unexpected(&other));
        }
    };

    let binding = choose_binding(scram.channel_binding, certificate, &mechanisms)?;
    let mechanism = binding.mechanism();

    if !mechanisms.contains(&backend::Mechanism::Known(mechanism)) {
        return Err(StartError::NoSupportedMechanism);
    }

    let password = obtain(&scram.password).await?;
    let iteration_bounds = scram.iteration_bounds;

    let (scram, client_first) = authentication::scram::Exchange::new(
        password.expose(),
        authentication::scram::Exchange::nonce(),
        binding,
    );

    connection
        .queue_send(frontend::SaslInitialResponse {
            mechanism,
            initial: Some(&client_first),
        })
        .map_err(StartError::Encode)?;

    connection.flush_send().await.map_err(StartError::Send)?;

    let frame = connection
        .receive_authentication()
        .await
        .map_err(StartError::Receive)?;

    let (verifier, client_final) = match backend::AuthenticationMessage::decode(&frame)
        .map_err(StartError::Decode)?
    {
        backend::AuthenticationMessage::Authentication(backend::Authentication::SaslContinue {
            data,
        }) => {
            let server_first = data.to_vec();

            // PBKDF2 runs for as many iterations as the server names: 0.6 ms
            // at PostgreSQL's default, seconds near the accepted maximum. On
            // the async worker that would stall every task sharing it, so it
            // runs on the blocking pool, which costs about 10 µs when warm.
            tokio::task::spawn_blocking(move || scram.respond(&server_first, iteration_bounds))
                .await
                .map_err(StartError::KeyDerivation)?
                .map_err(StartError::Scram)?
        }
        backend::AuthenticationMessage::ErrorResponse(_) => return Err(rejected(frame.clone())),
        other @ (backend::AuthenticationMessage::Authentication(_)
        | backend::AuthenticationMessage::NegotiateProtocolVersion(_)) => {
            return Err(unexpected(&other));
        }
    };

    connection
        .queue_send(frontend::SaslResponse {
            payload: &client_final,
        })
        .map_err(StartError::Encode)?;

    connection.flush_send().await.map_err(StartError::Send)?;

    let frame = connection
        .receive_authentication()
        .await
        .map_err(StartError::Receive)?;

    match backend::AuthenticationMessage::decode(&frame).map_err(StartError::Decode)? {
        backend::AuthenticationMessage::Authentication(backend::Authentication::SaslFinal {
            data,
        }) => {
            verifier.verify(data).map_err(StartError::Scram)?;
        }
        backend::AuthenticationMessage::ErrorResponse(_) => return Err(rejected(frame.clone())),
        other @ (backend::AuthenticationMessage::Authentication(_)
        | backend::AuthenticationMessage::NegotiateProtocolVersion(_)) => {
            return Err(unexpected(&other));
        }
    }

    receive_authentication_ok(connection)
        .await
        .map(|()| version)
}

async fn receive_authentication_ok(
    connection: &mut connection::Connection,
) -> Result<(), StartError> {
    let frame = connection
        .receive_authentication()
        .await
        .map_err(StartError::Receive)?;

    match backend::AuthenticationMessage::decode(&frame).map_err(StartError::Decode)? {
        backend::AuthenticationMessage::Authentication(backend::Authentication::Ok) => Ok(()),
        backend::AuthenticationMessage::ErrorResponse(_) => Err(rejected(frame.clone())),
        other @ (backend::AuthenticationMessage::Authentication(_)
        | backend::AuthenticationMessage::NegotiateProtocolVersion(_)) => Err(unexpected(&other)),
    }
}

async fn obtain(password: &authentication::Password) -> Result<config::Password, StartError> {
    password
        .resolve()
        .await
        .map_err(StartError::PasswordSourcing)
}

async fn send_cleartext_password(
    connection: &mut connection::Connection,
    method: &authentication::method::CleartextPassword,
    confidential: bool,
) -> Result<(), StartError> {
    if !confidential {
        return Err(StartError::CleartextOverUnencryptedTcp);
    }

    let password = obtain(&method.password).await?;

    let sent = match connection.queue_send(frontend::PasswordMessage {
        password: &password,
    }) {
        Ok(()) => connection.flush_send().await.map_err(StartError::Send),
        Err(error) => Err(StartError::Encode(error)),
    };

    // The only message that carries the secret itself, in the clear, so it is
    // wiped from the send buffer whether or not it went out.
    connection.wipe_send_buffer();

    sent?;

    receive_authentication_ok(connection).await
}

fn choose_binding(
    channel_binding: config::ChannelBinding,
    certificate: Option<authentication::scram::ChannelBindingData>,
    mechanisms: &[backend::Mechanism<'_>],
) -> Result<authentication::scram::ClientBinding, StartError> {
    let Some(data) = certificate else {
        return match channel_binding {
            config::ChannelBinding::Require => Err(StartError::ChannelBindingWithoutTls),
            config::ChannelBinding::Prefer | config::ChannelBinding::Disable => {
                Ok(authentication::scram::ClientBinding::Unavailable)
            }
        };
    };

    let offered = mechanisms.contains(&backend::Mechanism::Known(
        authentication::scram::Mechanism::ScramSha256Plus,
    ));

    match (channel_binding, offered) {
        // Declining is `n`, not `y`: `y` claims the server did not offer it.
        (config::ChannelBinding::Disable, _) => {
            Ok(authentication::scram::ClientBinding::Unavailable)
        }
        (_, true) => Ok(authentication::scram::ClientBinding::Bound(data)),
        (config::ChannelBinding::Require, false) => Err(StartError::ChannelBindingRefused),
        (config::ChannelBinding::Prefer, false) => {
            Ok(authentication::scram::ClientBinding::Unoffered)
        }
    }
}

async fn settle(connection: &mut connection::Connection) -> Result<Settled, StartError> {
    let mut parameters = parameter::StartupParameters::default();
    let mut backend_key = None;

    loop {
        let frame = connection.receive().await.map_err(StartError::Receive)?;

        match backend::Message::decode(&frame).map_err(StartError::Decode)? {
            backend::Message::ReadyForQuery(transaction_status) => {
                return Ok(Settled {
                    parameters: parameters.try_into().map_err(StartError::ParameterError)?,
                    transaction_status,
                    backend_key: backend_key.ok_or(StartError::NoCancellationKey)?,
                });
            }
            backend::Message::ParameterStatus(status) => {
                parameters
                    .apply(status)
                    .map_err(StartError::ParameterError)?;
            }
            backend::Message::BackendKeyData(data) => backend_key = Some(data),
            backend::Message::NoticeResponse(_) => (),
            backend::Message::ErrorResponse(_) => return Err(rejected(frame.clone())),
            other => return Err(unexpected(&other)),
        }
    }
}

struct Settled {
    parameters: parameter::RuntimeParameters,
    transaction_status: backend::TransactionStatus,
    backend_key: backend::BackendKeyData,
}

// What the server may send while answering a simple Query, short of the
// errors, notices and ReadyForQuery every reader handles itself.
const fn answers_a_query(message: &backend::Message<'_>) -> bool {
    matches!(
        message,
        backend::Message::RowDescription(_)
            | backend::Message::DataRow(_)
            | backend::Message::CommandComplete(_)
            | backend::Message::EmptyQueryResponse
            | backend::Message::CopyInResponse(_)
            | backend::Message::CopyOutResponse(_)
            | backend::Message::CopyBothResponse(_)
            | backend::Message::CopyData(_)
            | backend::Message::CopyDone
    )
}

fn rejected(frame: impl Into<protocol::Frame>) -> StartError {
    StartError::Rejected(ServerError(frame.into()))
}

fn unexpected(message: &impl core::fmt::Debug) -> StartError {
    StartError::Unexpected {
        message: format!("{message:?}"),
    }
}

fn server_name(host: &config::Host) -> connection::rustls::pki_types::ServerName<'static> {
    match host {
        config::Host::HostName(name) => {
            connection::rustls::pki_types::ServerName::DnsName(name.dns_name().clone())
        }
        config::Host::IpAddr(address) => {
            connection::rustls::pki_types::ServerName::IpAddress((*address).into())
        }
    }
}
