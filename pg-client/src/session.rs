pub mod execute;

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
    unfinished: bool,
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Statement {
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

    #[must_use]
    pub const fn mode(&self) -> Mode {
        self.mode
    }

    fn queue_send_in(
        &mut self,
        mode: Mode,
        message: impl frontend::Message,
    ) -> Result<(), QueueError> {
        if self.mode == mode {
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
    ) -> Result<Statement, QueueError> {
        self.queue_send_in(
            Mode::CommandProcessing,
            frontend::Parse {
                statement: &name,
                sql,
                parameter_types,
            },
        )?;

        Ok(Statement { name })
    }

    pub fn queue_bind<P: frontend::Parameter>(
        &mut self,
        statement: &Statement,
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

    pub fn queue_describe_statement(&mut self, statement: &Statement) -> Result<(), QueueError> {
        self.queue_send_in(
            Mode::CommandProcessing,
            frontend::Describe {
                target: &frontend::Target::Statement(statement.name.clone()),
            },
        )
    }

    pub fn queue_close_statement(&mut self, statement: Statement) -> Result<(), QueueError> {
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
        self.queue_send_in(Mode::CommandProcessing, frontend::Sync)
    }

    pub fn queue_query(&mut self, sql: &crate::Sql<'_>) -> Result<(), QueueError> {
        self.queue_send_in(Mode::CommandProcessing, frontend::Query { sql })
    }

    pub async fn send_terminate(mut self) -> Result<(), TerminateError> {
        self.queue_send_in(Mode::CommandProcessing, frontend::Terminate)?;

        Ok(self.connection.flush_send().await?)
    }

    pub fn queue_flush(&mut self) -> Result<(), QueueError> {
        self.queue_send_in(Mode::CommandProcessing, frontend::Flush)
    }

    pub async fn receive(&mut self) -> Result<protocol::Frame, ReceiveError> {
        self.connection.flush_send().await?;

        let frame = self.connection.receive().await?;

        match (self.mode, backend::Message::decode(&frame)?) {
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
                self.transaction_status = status;
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

    pub fn statement<'q>(&mut self, sql: &crate::Sql<'q>) -> execute::Builder<'q, &mut Self> {
        execute::Builder::new(self, sql)
    }

    pub async fn statements(&mut self, sql: &crate::Sql<'_>) -> Result<(), execute::Error> {
        if self.unfinished {
            self.read_to_ready().await?;
            self.unfinished = false;
        }

        self.queue_query(sql)?;
        self.unfinished = true;

        let mut error = None;

        loop {
            let frame = self.receive().await?;

            if self.mode == Mode::CopyIn {
                self.refuse_copy_in()?;
            }

            let failure = match backend::Message::decode(&frame).map_err(ReceiveError::Decode)? {
                backend::Message::ReadyForQuery(_) => break,
                backend::Message::ErrorResponse(response) => {
                    let failure = execute::Error::Server(ServerError(frame.clone()));

                    if response.diagnostic.severity().ends_session() {
                        self.unfinished = false;

                        return Err(error.unwrap_or(failure));
                    }

                    Some(failure)
                }
                backend::Message::NoticeResponse(_)
                | backend::Message::ParameterStatus(_)
                | backend::Message::RowDescription(_)
                | backend::Message::DataRow(_)
                | backend::Message::CommandComplete(_)
                | backend::Message::EmptyQueryResponse => None,
                _ => Some(execute::Error::Unexpected(frame.clone())),
            };

            if error.is_none() {
                error = failure;
            }
        }

        self.unfinished = false;

        error.map_or(Ok(()), Err)
    }

    pub async fn copy_in(&mut self, sql: &crate::Sql<'_>) -> Result<CopyIn<'_>, execute::Error> {
        if self.unfinished {
            self.read_to_ready().await?;
            self.unfinished = false;
        }

        self.queue_query(sql)?;
        self.unfinished = true;

        let mut error = None;

        loop {
            let frame = self.receive().await?;

            if self.mode == Mode::CopyIn {
                if error.is_none() {
                    return Ok(CopyIn {
                        session: self,
                        ended: false,
                    });
                }

                self.refuse_copy_in()?;

                continue;
            }

            let failure = match backend::Message::decode(&frame).map_err(ReceiveError::Decode)? {
                backend::Message::ReadyForQuery(_) => break,
                backend::Message::ErrorResponse(response) => {
                    let failure = execute::Error::Server(ServerError(frame.clone()));

                    if response.diagnostic.severity().ends_session() {
                        self.unfinished = false;

                        return Err(error.unwrap_or(failure));
                    }

                    Some(failure)
                }
                backend::Message::NoticeResponse(_) | backend::Message::ParameterStatus(_) => None,
                _ => Some(execute::Error::Unexpected(frame.clone())),
            };

            if error.is_none() {
                error = failure;
            }
        }

        self.unfinished = false;

        Err(error.expect("the server answers a query before it is ready again"))
    }

    async fn read_to_ready(&mut self) -> Result<(), ReceiveError> {
        loop {
            let frame = self.receive().await?;

            if self.mode == Mode::CopyIn {
                self.refuse_copy_in()?;
            }

            if self.mode == Mode::CommandProcessing
                && matches!(
                    backend::Message::decode(&frame)?,
                    backend::Message::ReadyForQuery(_)
                )
            {
                return Ok(());
            }
        }
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

#[derive(Debug)]
pub struct CopyIn<'s> {
    session: &'s mut Session,
    ended: bool,
}

impl CopyIn<'_> {
    pub async fn write(&mut self, data: &[u8]) -> Result<(), execute::Error> {
        self.session
            .send_copy_data(data)
            .await
            .map_err(|error| match error {
                CopyDataError::Queue(error) => execute::Error::Queue(error),
                CopyDataError::Send(error) => execute::Error::Send(error),
            })
    }

    pub async fn finish(mut self) -> Result<u64, execute::Error> {
        self.session.queue_copy_done()?;
        self.ended = true;

        let mut rows = None;
        let mut error = None;

        loop {
            let frame = self.session.receive().await?;

            let failure = match backend::Message::decode(&frame).map_err(ReceiveError::Decode)? {
                backend::Message::ReadyForQuery(_) => break,
                backend::Message::CommandComplete(backend::CommandComplete::Copy {
                    rows: copied,
                }) => {
                    rows = Some(copied);
                    None
                }
                backend::Message::ErrorResponse(response) => {
                    let failure = execute::Error::Server(ServerError(frame.clone()));

                    if response.diagnostic.severity().ends_session() {
                        self.session.unfinished = false;

                        return Err(error.unwrap_or(failure));
                    }

                    Some(failure)
                }
                backend::Message::NoticeResponse(_) | backend::Message::ParameterStatus(_) => None,
                _ => Some(execute::Error::Unexpected(frame.clone())),
            };

            if error.is_none() {
                error = failure;
            }
        }

        self.session.unfinished = false;

        match error {
            Some(error) => Err(error),
            None => Ok(rows.expect("a completed COPY reports its row count")),
        }
    }

    pub async fn abort(mut self, reason: &protocol::String<'_>) -> Result<(), execute::Error> {
        self.session.queue_copy_fail(reason)?;
        self.ended = true;

        self.session.read_to_ready().await?;
        self.session.unfinished = false;

        Ok(())
    }
}

impl Drop for CopyIn<'_> {
    fn drop(&mut self) {
        const ABANDONED: protocol::String<'static> =
            protocol::String::from_static_or_panic("COPY abandoned by the client");

        if !self.ended {
            self.session
                .queue_copy_fail(&ABANDONED)
                .expect("a COPY that has not ended is still in CopyIn mode");
        }
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
    ChannelBinding(#[source] authentication::ChannelBindingError),
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
    Scram(#[source] authentication::ScramError),
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

pub trait Source {
    type Handle: core::ops::DerefMut<Target = Session>;
    type Error;

    fn acquire(&self) -> impl Future<Output = Result<Self::Handle, Self::Error>> + Send;
}

impl Source for Arc<tokio::sync::Mutex<Session>> {
    type Handle = tokio::sync::OwnedMutexGuard<Session>;
    type Error = core::convert::Infallible;

    fn acquire(&self) -> impl Future<Output = Result<Self::Handle, Self::Error>> + Send {
        let shared = Arc::clone(self);

        async move { Ok(shared.lock_owned().await) }
    }
}

impl Source for Arc<Config> {
    type Handle = Box<Session>;
    type Error = StartError;

    fn acquire(&self) -> impl Future<Output = Result<Self::Handle, Self::Error>> + Send {
        let config = Arc::clone(self);

        async move { start(config).await.map(Box::new) }
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
        unfinished: false,
    })
}

async fn establish(
    config: &connection::Config,
) -> Result<
    (
        connection::Stream,
        Option<authentication::ChannelBindingData>,
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

    let binding = authentication::ChannelBindingData::from_certificate(certificate)
        .map_err(StartError::ChannelBinding)?;

    Ok((stream, Some(binding)))
}

async fn authenticate(
    connection: &mut connection::Connection,
    config: &Config,
    certificate: Option<authentication::ChannelBindingData>,
) -> Result<protocol::Version, StartError> {
    let mut frame = connection.receive().await.map_err(StartError::Receive)?;

    let negotiated = match backend::Message::decode(&frame) {
        Ok(backend::Message::NegotiateProtocolVersion(negotiated)) => Some(negotiated.version),
        Ok(_) => None,
        Err(backend::DecodeError::UnsupportedProtocolVersion { code }) => {
            return Err(StartError::UnsupportedProtocol { code });
        }
        Err(error) => return Err(StartError::Decode(error)),
    };

    let version = match negotiated {
        Some(version) => {
            frame = connection.receive().await.map_err(StartError::Receive)?;
            version
        }
        None => protocol::Version::V3_2,
    };

    let methods = &config.authentication.methods;

    let (scram, mechanisms) = match backend::Message::decode(&frame).map_err(StartError::Decode)? {
        // Unchallenged: nothing to prove, and nothing more to read.
        backend::Message::Authentication(backend::Authentication::Ok)
            if methods.accept_unchallenged =>
        {
            return Ok(version);
        }
        backend::Message::Authentication(backend::Authentication::Sasl { mechanisms })
            if let Some(scram) = &methods.scram =>
        {
            (scram, mechanisms.into_iter().collect::<Vec<_>>())
        }
        backend::Message::Authentication(backend::Authentication::CleartextPassword)
            if let Some(cleartext) = &methods.cleartext_password =>
        {
            let confidential = certificate.is_some()
                || matches!(
                    config.connection.endpoint,
                    connection::Endpoint::Unix { .. }
                );

            return send_cleartext_password(connection, cleartext, confidential)
                .await
                .map(|()| version);
        }
        backend::Message::Authentication(request) => {
            return Err(StartError::MethodNotConfigured {
                requested: request.request(),
            });
        }
        backend::Message::ErrorResponse(_) => return Err(rejected(&frame)),
        other => return Err(unexpected(&other)),
    };

    let binding = choose_binding(scram.channel_binding, certificate, &mechanisms)?;
    let mechanism = binding.mechanism();

    if !mechanisms.contains(&backend::Mechanism::Known(mechanism)) {
        return Err(StartError::NoSupportedMechanism);
    }

    let password = obtain(&scram.password).await?;

    let (scram, client_first) =
        authentication::Scram::new(password.as_str(), authentication::Scram::nonce(), binding);

    connection
        .queue_send(frontend::SaslInitialResponse {
            mechanism,
            initial: Some(&client_first),
        })
        .map_err(StartError::Encode)?;

    connection.flush_send().await.map_err(StartError::Send)?;

    let frame = connection.receive().await.map_err(StartError::Receive)?;

    let (verifier, client_final) =
        match backend::Message::decode(&frame).map_err(StartError::Decode)? {
            backend::Message::Authentication(backend::Authentication::SaslContinue { data }) => {
                scram.respond(data).map_err(StartError::Scram)?
            }
            backend::Message::ErrorResponse(_) => return Err(rejected(&frame)),
            other => return Err(unexpected(&other)),
        };

    connection
        .queue_send(frontend::SaslResponse {
            payload: &client_final,
        })
        .map_err(StartError::Encode)?;

    connection.flush_send().await.map_err(StartError::Send)?;

    let frame = connection.receive().await.map_err(StartError::Receive)?;

    match backend::Message::decode(&frame).map_err(StartError::Decode)? {
        backend::Message::Authentication(backend::Authentication::SaslFinal { data }) => {
            verifier.verify(data).map_err(StartError::Scram)?;
        }
        backend::Message::ErrorResponse(_) => return Err(rejected(&frame)),
        other => return Err(unexpected(&other)),
    }

    let frame = connection.receive().await.map_err(StartError::Receive)?;

    match backend::Message::decode(&frame).map_err(StartError::Decode)? {
        backend::Message::Authentication(backend::Authentication::Ok) => Ok(version),
        backend::Message::ErrorResponse(_) => Err(rejected(&frame)),
        other => Err(unexpected(&other)),
    }
}

async fn obtain(password: &authentication::Password) -> Result<config::Password, StartError> {
    match password {
        authentication::Password::Static(password) => Ok(password.clone()),
        authentication::Password::Dynamic(source) => source
            .password()
            .await
            .map_err(StartError::PasswordSourcing),
    }
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

    connection
        .queue_send(frontend::PasswordMessage {
            password: &password,
        })
        .map_err(StartError::Encode)?;

    connection.flush_send().await.map_err(StartError::Send)?;

    let frame = connection.receive().await.map_err(StartError::Receive)?;

    match backend::Message::decode(&frame).map_err(StartError::Decode)? {
        backend::Message::Authentication(backend::Authentication::Ok) => Ok(()),
        backend::Message::ErrorResponse(_) => Err(rejected(&frame)),
        other => Err(unexpected(&other)),
    }
}

fn choose_binding(
    channel_binding: config::ChannelBinding,
    certificate: Option<authentication::ChannelBindingData>,
    mechanisms: &[backend::Mechanism<'_>],
) -> Result<authentication::ClientBinding, StartError> {
    let Some(data) = certificate else {
        return match channel_binding {
            config::ChannelBinding::Require => Err(StartError::ChannelBindingWithoutTls),
            config::ChannelBinding::Prefer | config::ChannelBinding::Disable => {
                Ok(authentication::ClientBinding::Unavailable)
            }
        };
    };

    let offered = mechanisms.contains(&backend::Mechanism::Known(
        authentication::Mechanism::ScramSha256Plus,
    ));

    match (channel_binding, offered) {
        // Declining is `n`, not `y`: `y` claims the server did not offer it.
        (config::ChannelBinding::Disable, _) => Ok(authentication::ClientBinding::Unavailable),
        (_, true) => Ok(authentication::ClientBinding::Bound(data)),
        (config::ChannelBinding::Require, false) => Err(StartError::ChannelBindingRefused),
        (config::ChannelBinding::Prefer, false) => Ok(authentication::ClientBinding::Unoffered),
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
            backend::Message::ErrorResponse(_) => return Err(rejected(&frame)),
            other => return Err(unexpected(&other)),
        }
    }
}

struct Settled {
    parameters: parameter::RuntimeParameters,
    transaction_status: backend::TransactionStatus,
    backend_key: backend::BackendKeyData,
}

fn rejected(frame: &protocol::Frame) -> StartError {
    StartError::Rejected(ServerError(frame.clone()))
}

fn unexpected(message: &backend::Message<'_>) -> StartError {
    StartError::Unexpected {
        message: format!("{message:?}"),
    }
}

fn server_name(host: &config::Host) -> connection::rustls::pki_types::ServerName<'static> {
    match host {
        config::Host::HostName(name) => {
            connection::rustls::pki_types::ServerName::try_from(name.as_str().to_owned())
                .expect("a validated host name is a valid server name")
        }
        config::Host::IpAddr(address) => {
            connection::rustls::pki_types::ServerName::IpAddress((*address).into())
        }
    }
}
