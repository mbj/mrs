use pg_client::protocol::{backend, frontend};
use pg_client::{authentication, config, connection, parameter, protocol, session};

fn definition(backend: ociman::Backend) -> pg_ephemeral::Definition {
    pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        "test".parse().unwrap(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30))
}

fn tcp(
    config: &session::Config,
) -> (
    &config::Host,
    Option<&config::HostAddr>,
    config::Port,
    Option<&connection::Tls>,
) {
    let connection::Endpoint::Tcp {
        host,
        address,
        port,
        tls,
    } = &config.connection.endpoint
    else {
        panic!("expected a TCP endpoint");
    };

    (host, address.as_ref(), *port, tls.as_ref())
}

fn password(config: &session::Config) -> &config::Password {
    let Some(authentication::method::Scram {
        password: authentication::Password::Static(password),
        ..
    }) = &config.authentication.methods.scram
    else {
        panic!("expected SCRAM with a static password");
    };

    password
}

async fn connect(config: &session::Config) -> tokio::net::TcpStream {
    let (host, _, port, _) = tcp(config);

    let host = match host {
        config::Host::HostName(name) => name.as_str().to_owned(),
        config::Host::IpAddr(address) => address.to_string(),
    };

    tokio::net::TcpStream::connect((host.as_str(), u16::from(port)))
        .await
        .unwrap()
}

/// The certificate name resolves nowhere on purpose, so a certificate for the
/// wrong name cannot pass by coincidence.
const CERTIFICATE_NAME: &str = "pg-ephemeral.test";

fn trust(config: &session::Config) -> connection::rustls::ClientConfig {
    let (_, _, _, tls) = tcp(config);

    connection::rustls::ClientConfig::clone(tls.expect("a TLS endpoint").client())
}

async fn connect_tls(
    config: &session::Config,
    name: &str,
) -> Result<connection::Stream, connection::TlsError> {
    connect_tls_with(config, name, trust(config)).await
}

async fn connect_tls_with(
    config: &session::Config,
    name: &str,
    client: connection::rustls::ClientConfig,
) -> Result<connection::Stream, connection::TlsError> {
    let (_, address, port, _) = tcp(config);

    let address = std::net::IpAddr::from(address.expect("endpoint host address"));
    let port = u16::from(port);

    connection::Stream::tls(
        tokio::net::TcpStream::connect((address, port))
            .await
            .unwrap(),
        std::sync::Arc::new(client),
        connection::rustls::pki_types::ServerName::try_from(name.to_owned()).unwrap(),
    )
    .await
}

/// `Terminate` is how the protocol prescribes ending a session. Nothing answers
/// it, so the close is the acknowledgement.
async fn terminate(mut connection: connection::Connection) {
    connection.queue_send(frontend::Terminate).unwrap();

    connection.flush_send().await.unwrap();

    match connection.receive().await {
        Err(protocol::ReceiveError::Closed) => (),
        other => panic!("expected the server to close, got {other:?}"),
    }
}

/// During authentication the server is reading a password message and takes
/// anything else, `Terminate` included, as a protocol violation.
async fn terminate_during_authentication(mut connection: connection::Connection) {
    connection.queue_send(frontend::Terminate).unwrap();

    connection.flush_send().await.unwrap();

    let frame = connection.receive().await.unwrap();

    let Ok(backend::Message::ErrorResponse(backend::ErrorResponse { diagnostic })) =
        backend::Message::decode(&frame)
    else {
        panic!(
            "expected an error, got {:?}",
            backend::Message::decode(&frame)
        );
    };

    assert_eq!(diagnostic.severity(), backend::Severity::Fatal);
    assert_eq!(diagnostic.code(), backend::SqlState::ProtocolViolation);

    match connection.receive().await {
        Err(protocol::ReceiveError::Closed) => (),
        other => panic!("expected the server to close, got {other:?}"),
    }
}

async fn cancel(config: &session::Config, key: &backend::BackendKeyData) {
    let mut connection = connection::Connection::new(
        connection::Stream::Tcp(connect(config).await),
        protocol::ReceiveBufferSize::DEFAULT,
    );

    connection
        .queue_send(frontend::CancelRequest { key })
        .unwrap();

    connection.flush_send().await.unwrap();

    match connection.receive().await {
        Err(protocol::ReceiveError::Closed) => (),
        other => panic!("expected the server to close, got {other:?}"),
    }
}

#[tokio::test]
async fn reads_the_authentication_frame() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .with_container(async |container| {
            let config = &container.session_config();
            let stream = connect(config).await;

            let mut connection = connection::Connection::new(
                connection::Stream::Tcp(stream),
                protocol::ReceiveBufferSize::DEFAULT,
            );

            connection
                .queue_send(frontend::StartupMessage {
                    version: protocol::Version::V3_2,
                    user: &config.authentication.user,
                    database: Some(&config.database),
                    replication: None,
                    parameters: &parameter::Map::new(),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            // pg-ephemeral sets a password and leaves the image to initialise
            // `scram-sha-256`, so the server demands SASL.
            let Ok(backend::Message::Authentication(backend::Authentication::Sasl { mechanisms })) =
                backend::Message::decode(&frame)
            else {
                panic!(
                    "expected a SASL request, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            assert_eq!(
                mechanisms.into_iter().collect::<Vec<_>>(),
                vec![backend::Mechanism::Known(
                    authentication::Mechanism::ScramSha256
                )],
            );

            terminate_during_authentication(connection).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn completes_a_scram_handshake_and_reads_the_startup_burst() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .with_container(async |container| {
            let config = &container.session_config();
            let stream = connect(config).await;

            let mut connection = connection::Connection::new(
                connection::Stream::Tcp(stream),
                protocol::ReceiveBufferSize::DEFAULT,
            );

            connection
                .queue_send(frontend::StartupMessage {
                    version: protocol::Version::V3_2,
                    user: &config.authentication.user,
                    database: Some(&config.database),
                    replication: None,
                    parameters: &parameter::Map::new(),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::Authentication(backend::Authentication::Sasl { mechanisms })) =
                backend::Message::decode(&frame)
            else {
                panic!(
                    "expected a SASL request, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            assert!(mechanisms.into_iter().any(|offered| offered
                == backend::Mechanism::Known(authentication::Mechanism::ScramSha256)));

            let password = password(config);

            let (scram, client_first) = authentication::Scram::new(
                password.as_str(),
                authentication::Scram::nonce(),
                authentication::ClientBinding::Unavailable,
            );

            connection
                .queue_send(frontend::SaslInitialResponse {
                    mechanism: authentication::Mechanism::ScramSha256,
                    initial: Some(&client_first),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::Authentication(backend::Authentication::SaslContinue {
                data,
            })) = backend::Message::decode(&frame)
            else {
                panic!(
                    "expected a challenge, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            let (verifier, client_final) = scram.respond(data).unwrap();

            connection
                .queue_send(frontend::SaslResponse {
                    payload: &client_final,
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::Authentication(backend::Authentication::SaslFinal { data })) =
                backend::Message::decode(&frame)
            else {
                panic!(
                    "expected a final message, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            verifier.verify(data).unwrap();

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::Authentication(
                    backend::Authentication::Ok
                )),
            );

            let mut parameters = Vec::new();
            let mut backend_key = None;

            let status = loop {
                let frame = connection.receive().await.unwrap();

                match backend::Message::decode(&frame).unwrap() {
                    backend::Message::ParameterStatus(status) => parameters.push(status.name),
                    backend::Message::BackendKeyData(data) => backend_key = Some(data),
                    backend::Message::ReadyForQuery(status) => break status,
                    other => panic!("unexpected message during startup: {other:?}"),
                }
            };

            assert_eq!(status, backend::TransactionStatus::Idle);
            assert!(backend_key.is_some(), "server sent no cancellation key");

            let names: Vec<&str> = parameters.iter().map(parameter::Name::as_str).collect();

            assert!(
                names.contains(&"server_version") && names.contains(&"client_encoding"),
                "startup parameters: {names:?}",
            );

            terminate(connection).await;
        })
        .await
        .unwrap()
}

async fn authenticate(
    config: &session::Config,
) -> (connection::Connection, backend::BackendKeyData) {
    start(config, connection::Stream::Tcp(connect(config).await)).await
}

async fn start(
    config: &session::Config,
    stream: connection::Stream,
) -> (connection::Connection, backend::BackendKeyData) {
    let mut connection = connection::Connection::new(stream, protocol::ReceiveBufferSize::DEFAULT);

    connection
        .queue_send(frontend::StartupMessage {
            version: protocol::Version::V3_2,
            user: &config.authentication.user,
            database: Some(&config.database),
            replication: None,
            parameters: &parameter::Map::new(),
        })
        .unwrap();

    connection.flush_send().await.unwrap();

    let mut frame = connection.receive().await.unwrap();

    if let Ok(backend::Message::NegotiateProtocolVersion(_)) = backend::Message::decode(&frame) {
        frame = connection.receive().await.unwrap();
    }

    let Ok(backend::Message::Authentication(backend::Authentication::Sasl { .. })) =
        backend::Message::decode(&frame)
    else {
        panic!(
            "expected a SASL request, got {:?}",
            backend::Message::decode(&frame)
        );
    };

    let (scram, client_first) = authentication::Scram::new(
        password(config).as_str(),
        authentication::Scram::nonce(),
        authentication::ClientBinding::Unavailable,
    );

    connection
        .queue_send(frontend::SaslInitialResponse {
            mechanism: authentication::Mechanism::ScramSha256,
            initial: Some(&client_first),
        })
        .unwrap();

    connection.flush_send().await.unwrap();

    let frame = connection.receive().await.unwrap();

    let Ok(backend::Message::Authentication(backend::Authentication::SaslContinue { data })) =
        backend::Message::decode(&frame)
    else {
        panic!(
            "expected a challenge, got {:?}",
            backend::Message::decode(&frame)
        );
    };

    let (verifier, client_final) = scram.respond(data).unwrap();

    connection
        .queue_send(frontend::SaslResponse {
            payload: &client_final,
        })
        .unwrap();

    connection.flush_send().await.unwrap();

    let frame = connection.receive().await.unwrap();

    let Ok(backend::Message::Authentication(backend::Authentication::SaslFinal { data })) =
        backend::Message::decode(&frame)
    else {
        panic!(
            "expected a final message, got {:?}",
            backend::Message::decode(&frame)
        );
    };

    verifier.verify(data).unwrap();

    let mut key = None;

    loop {
        let frame = connection.receive().await.unwrap();

        match backend::Message::decode(&frame).unwrap() {
            backend::Message::BackendKeyData(data) => key = Some(data),
            backend::Message::ReadyForQuery(_) => break,
            _ => (),
        }
    }

    (connection, key.expect("server sent no cancellation key"))
}

#[tokio::test]
async fn runs_a_simple_query_and_reads_its_rows() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .with_container(async |container| {
            let (mut connection, _) = authenticate(&container.session_config()).await;

            connection
                .queue_send(frontend::Query {
                    sql: &pg_client::Sql::from_static_or_panic(
                        "SELECT 1 AS one, 'two' AS word, NULL AS nothing",
                    ),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::RowDescription(description)) =
                backend::Message::decode(&frame)
            else {
                panic!(
                    "expected a row description, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            let names: Vec<String> = description
                .into_iter()
                .map(|column| column.name.to_string())
                .collect();

            assert_eq!(names, vec!["one", "word", "nothing"]);

            // Simple query always answers in text format.
            assert!(
                description
                    .into_iter()
                    .all(|column| column.format == protocol::Format::Text),
            );

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::DataRow(row)) = backend::Message::decode(&frame) else {
                panic!("expected a row, got {:?}", backend::Message::decode(&frame));
            };

            assert_eq!(
                row.into_iter().collect::<Vec<_>>(),
                vec![Some(b"1".as_slice()), Some(b"two".as_slice()), None],
            );

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::CommandComplete(
                    backend::CommandComplete::Select { rows: 1 },
                )),
            );

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::ReadyForQuery(
                    backend::TransactionStatus::Idle
                )),
            );

            connection
                .queue_send(frontend::Query {
                    sql: &pg_client::Sql::from_static_or_panic("SELECT 1"),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::RowDescription(description)) =
                backend::Message::decode(&frame)
            else {
                panic!(
                    "expected a row description, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            assert_eq!(
                description.into_iter().next().unwrap().name.as_str(),
                "?column?"
            );

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::DataRow(_)) = backend::Message::decode(&frame) else {
                panic!("expected a row, got {:?}", backend::Message::decode(&frame));
            };

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::CommandComplete(
                    backend::CommandComplete::Select { rows: 1 },
                )),
            );

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::ReadyForQuery(
                    backend::TransactionStatus::Idle
                )),
            );

            terminate(connection).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn runs_an_extended_query_with_a_bound_parameter() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .with_container(async |container| {
            let (mut connection, _) = authenticate(&container.session_config()).await;

            let statement = frontend::StatementName::Unnamed;
            let portal = frontend::PortalName::Unnamed;
            let sql = pg_client::Sql::from_static_or_panic("SELECT $1::int + 1 AS incremented");

            connection
                .queue_send(frontend::Parse {
                    statement: &statement,
                    sql: &sql,
                    parameter_types: &[],
                })
                .unwrap();

            connection
                .queue_send(frontend::Bind {
                    portal: &portal,
                    statement: &statement,
                    parameter_formats: frontend::Formats::Default,
                    parameters: &[Some(b"41".as_slice())],
                    result_formats: frontend::Formats::Default,
                })
                .unwrap();

            connection
                .queue_send(frontend::Describe {
                    target: &frontend::Target::Portal(portal.clone()),
                })
                .unwrap();

            connection
                .queue_send(frontend::Execute {
                    portal: &portal,
                    max_rows: None,
                })
                .unwrap();

            connection.queue_send(frontend::Sync).unwrap();

            connection.flush_send().await.unwrap();

            let mut seen = Vec::new();

            let status = loop {
                let frame = connection.receive().await.unwrap();

                match backend::Message::decode(&frame).unwrap() {
                    backend::Message::ReadyForQuery(status) => break status,
                    backend::Message::DataRow(row) => {
                        assert_eq!(
                            row.into_iter().collect::<Vec<_>>(),
                            vec![Some(b"42".as_slice())],
                        );

                        seen.push("DataRow");
                    }
                    backend::Message::RowDescription(description) => {
                        assert_eq!(
                            description.into_iter().next().unwrap().name.as_str(),
                            "incremented"
                        );

                        seen.push("RowDescription");
                    }
                    backend::Message::ParseComplete => seen.push("ParseComplete"),
                    backend::Message::BindComplete => seen.push("BindComplete"),
                    backend::Message::CommandComplete(_) => seen.push("CommandComplete"),
                    other => panic!("unexpected message: {other:?}"),
                }
            };

            assert_eq!(
                seen,
                vec![
                    "ParseComplete",
                    "BindComplete",
                    "RowDescription",
                    "DataRow",
                    "CommandComplete",
                ],
            );

            assert_eq!(status, backend::TransactionStatus::Idle);

            terminate(connection).await;
        })
        .await
        .unwrap()
}

/// The query connection itself cannot say when its statement began: a backend
/// buffers its whole reply and flushes at `ReadyForQuery`, so a simple query
/// reads as nothing at all until it is over.
async fn await_active(config: &session::Config, process_id: backend::ProcessId) {
    let (mut connection, _) = authenticate(config).await;

    let statement = frontend::StatementName::Unnamed;
    let portal = frontend::PortalName::Unnamed;
    let pid = process_id.get().to_string();

    connection
        .queue_send(frontend::Parse {
            statement: &statement,
            sql: &pg_client::Sql::from_static_or_panic(
                "SELECT count(*) FROM pg_stat_activity WHERE pid = $1 AND state = 'active'",
            ),
            parameter_types: &[],
        })
        .unwrap();

    connection.queue_send(frontend::Sync).unwrap();

    connection.flush_send().await.unwrap();

    loop {
        let frame = connection.receive().await.unwrap();

        match backend::Message::decode(&frame).unwrap() {
            backend::Message::ParseComplete => (),
            backend::Message::ReadyForQuery(_) => break,
            other => panic!("unexpected message while preparing: {other:?}"),
        }
    }

    for _ in 0..1000 {
        connection
            .queue_send(frontend::Bind {
                portal: &portal,
                statement: &statement,
                parameter_formats: frontend::Formats::Default,
                parameters: &[Some(pid.as_bytes())],
                result_formats: frontend::Formats::Default,
            })
            .unwrap();

        connection
            .queue_send(frontend::Execute {
                portal: &portal,
                max_rows: None,
            })
            .unwrap();

        connection.queue_send(frontend::Sync).unwrap();

        connection.flush_send().await.unwrap();

        let mut active = false;

        loop {
            let frame = connection.receive().await.unwrap();

            match backend::Message::decode(&frame).unwrap() {
                backend::Message::DataRow(row) => {
                    active = row.into_iter().next().unwrap() == Some(b"1".as_slice());
                }
                backend::Message::BindComplete | backend::Message::CommandComplete(_) => (),
                backend::Message::ReadyForQuery(_) => break,
                other => panic!("unexpected message while polling: {other:?}"),
            }
        }

        if active {
            terminate(connection).await;

            return;
        }
    }

    panic!("backend {} never started a query", process_id.get());
}

async fn cancel_a_running_query(config: &session::Config) -> backend::BackendKeyData {
    let (mut connection, key) = authenticate(config).await;

    connection
        .queue_send(frontend::Query {
            sql: &pg_client::Sql::from_static_or_panic("SELECT pg_sleep(30)"),
        })
        .unwrap();

    connection.flush_send().await.unwrap();

    await_active(config, key.process_id()).await;

    cancel(config, &key).await;

    loop {
        let frame = connection.receive().await.unwrap();

        match backend::Message::decode(&frame) {
            // Written before the statement was interrupted, and flushed
            // alongside the error rather than dropped.
            Ok(backend::Message::RowDescription(_)) => (),
            Ok(backend::Message::ErrorResponse(backend::ErrorResponse { diagnostic })) => {
                assert_eq!(diagnostic.severity(), backend::Severity::Error);
                assert_eq!(diagnostic.code(), backend::SqlState::QueryCanceled);

                break;
            }
            other => panic!("expected the query to be cancelled, got {other:?}"),
        }
    }

    let frame = connection.receive().await.unwrap();

    assert_eq!(
        backend::Message::decode(&frame),
        Ok(backend::Message::ReadyForQuery(
            backend::TransactionStatus::Idle
        )),
    );

    terminate(connection).await;

    key
}

#[tokio::test]
async fn cancels_a_running_query() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .with_container(async |container| {
            let key = cancel_a_running_query(&container.session_config()).await;

            assert_eq!(key.cancel_key().as_slice().len(), 32);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn cancels_a_running_query_with_a_3_0_key() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .image(pg_ephemeral::Image::OfficialRelease {
            major: pg_ephemeral::image::Major::new(17),
            minor: pg_ephemeral::image::Minor::Latest,
            os: pg_ephemeral::image::OS::Default,
            digest: None,
        })
        .with_container(async |container| {
            let key = cancel_a_running_query(&container.session_config()).await;

            assert_eq!(key.cancel_key().as_slice().len(), 4);
        })
        .await
        .unwrap()
}

async fn run(connection: &mut connection::Connection, sql: &pg_client::Sql<'_>) {
    connection.queue_send(frontend::Query { sql }).unwrap();

    connection.flush_send().await.unwrap();

    loop {
        let frame = connection.receive().await.unwrap();

        match backend::Message::decode(&frame).unwrap() {
            backend::Message::RowDescription(_)
            | backend::Message::DataRow(_)
            | backend::Message::CommandComplete(_) => (),
            backend::Message::ReadyForQuery(_) => break,
            other => panic!("unexpected message: {other:?}"),
        }
    }
}

#[tokio::test]
async fn copies_rows_in_and_back_out() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .with_container(async |container| {
            let (mut connection, _) = authenticate(&container.session_config()).await;

            run(
                &mut connection,
                &pg_client::Sql::from_static_or_panic("CREATE TABLE example (id int, word text)"),
            )
            .await;

            connection
                .queue_send(frontend::Query {
                    sql: &pg_client::Sql::from_static_or_panic("COPY example FROM STDIN"),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::CopyInResponse(backend::CopyResponse {
                    format: protocol::Format::Text,
                    columns: vec![protocol::Format::Text, protocol::Format::Text],
                })),
            );

            connection
                .queue_send(frontend::CopyData {
                    data: b"1\tone\n2\tt",
                })
                .unwrap();
            connection
                .queue_send(frontend::CopyData { data: b"wo\n" })
                .unwrap();
            connection.queue_send(frontend::CopyDone).unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::CommandComplete(
                    backend::CommandComplete::Copy { rows: 2 },
                )),
            );

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::ReadyForQuery(
                    backend::TransactionStatus::Idle
                )),
            );

            connection
                .queue_send(frontend::Query {
                    sql: &pg_client::Sql::from_static_or_panic("COPY example TO STDOUT"),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::CopyOutResponse(backend::CopyResponse {
                    format: protocol::Format::Text,
                    columns: vec![protocol::Format::Text, protocol::Format::Text],
                })),
            );

            let mut rows: Vec<Vec<u8>> = Vec::new();

            loop {
                let frame = connection.receive().await.unwrap();

                match backend::Message::decode(&frame).unwrap() {
                    backend::Message::CopyData(data) => rows.push(data.to_vec()),
                    backend::Message::CopyDone => break,
                    other => panic!("unexpected message: {other:?}"),
                }
            }

            assert_eq!(rows, vec![b"1\tone\n".to_vec(), b"2\ttwo\n".to_vec()]);

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::CommandComplete(
                    backend::CommandComplete::Copy { rows: 2 },
                )),
            );

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::ReadyForQuery(
                    backend::TransactionStatus::Idle
                )),
            );

            terminate(connection).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn reports_a_constraint_violation_during_copy_in() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .with_container(async |container| {
            let (mut connection, _) = authenticate(&container.session_config()).await;

            run(
                &mut connection,
                &pg_client::Sql::from_static_or_panic(
                    "CREATE TABLE guarded (id int CONSTRAINT id_positive CHECK (id > 0))",
                ),
            )
            .await;

            connection
                .queue_send(frontend::Query {
                    sql: &pg_client::Sql::from_static_or_panic("COPY guarded FROM STDIN"),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::CopyInResponse(backend::CopyResponse {
                    format: protocol::Format::Text,
                    columns: vec![protocol::Format::Text],
                })),
            );

            // Everything after the failure is dropped, so sending `CopyDone`
            // regardless is harmless.
            connection
                .queue_send(frontend::CopyData { data: b"1\n-1\n" })
                .unwrap();
            connection.queue_send(frontend::CopyDone).unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::ErrorResponse(backend::ErrorResponse { diagnostic })) =
                backend::Message::decode(&frame)
            else {
                panic!(
                    "expected the copy to fail, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            assert_eq!(diagnostic.severity(), backend::Severity::Error);
            assert_eq!(diagnostic.code(), backend::SqlState::CheckViolation);
            assert_eq!(
                diagnostic.constraint().map(ToString::to_string),
                Some("id_positive".to_owned()),
            );

            // A simple query discards the rest of the message and readies the
            // session, so the connection survives a failed copy.
            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::ReadyForQuery(
                    backend::TransactionStatus::Idle
                )),
            );

            run(
                &mut connection,
                &pg_client::Sql::from_static_or_panic("SELECT 1"),
            )
            .await;

            terminate(connection).await;
        })
        .await
        .unwrap()
}

fn tls_definition(backend: ociman::Backend) -> pg_ephemeral::Definition {
    tls_definition_named(backend, "test")
}

fn tls_definition_named(backend: ociman::Backend, instance: &str) -> pg_ephemeral::Definition {
    pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        instance.parse().unwrap(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30))
    .ssl_config(pg_ephemeral::definition::SslConfig::Generated {
        hostname: CERTIFICATE_NAME.parse().unwrap(),
    })
}

fn channel_binding(stream: &connection::Stream) -> authentication::ChannelBindingData {
    let connection::Stream::Tls(tls) = stream else {
        panic!("expected a TLS stream, got {stream:?}");
    };

    let certificate = tls
        .get_ref()
        .1
        .peer_certificates()
        .and_then(<[_]>::first)
        .expect("server certificate");

    authentication::ChannelBindingData::from_certificate(certificate).unwrap()
}

#[tokio::test]
async fn runs_a_query_over_verified_tls() {
    let backend = ociman::test_backend_setup!();

    tls_definition(backend)
        .with_container(async |container| {
            let config = &container.session_config();
            let stream = connect_tls(config, CERTIFICATE_NAME).await.unwrap();

            let connection::Stream::Tls(tls) = &stream else {
                panic!("expected a TLS stream, got {stream:?}");
            };

            assert_eq!(
                tls.get_ref().1.alpn_protocol(),
                Some(connection::ALPN_PROTOCOL),
            );

            let (mut connection, _) = start(config, stream).await;

            connection
                .queue_send(frontend::Query {
                    sql: &pg_client::Sql::from_static_or_panic(
                        "SELECT ssl, version FROM pg_stat_ssl WHERE pid = pg_backend_pid()",
                    ),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::RowDescription(_)) = backend::Message::decode(&frame) else {
                panic!(
                    "expected a row description, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::DataRow(row)) = backend::Message::decode(&frame) else {
                panic!("expected a row, got {:?}", backend::Message::decode(&frame));
            };

            assert_eq!(
                row.into_iter().collect::<Vec<_>>(),
                vec![Some(b"t".as_slice()), Some(b"TLSv1.3".as_slice())],
            );

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::CommandComplete(
                    backend::CommandComplete::Select { rows: 1 },
                )),
            );

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::ReadyForQuery(
                    backend::TransactionStatus::Idle
                )),
            );

            terminate(connection).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_a_certificate_issued_for_another_name() {
    let backend = ociman::test_backend_setup!();

    tls_definition(backend)
        .with_container(async |container| {
            let error = connect_tls(&container.session_config(), "not-the-name.test")
                .await
                .unwrap_err();

            let connection::TlsError::Handshake(source) = &error else {
                panic!("expected a handshake failure, got {error:?}");
            };

            let reason = source
                .get_ref()
                .and_then(|source| source.downcast_ref::<connection::rustls::Error>())
                .unwrap_or_else(|| panic!("expected a rustls error, got {source:?}"));

            assert!(
                matches!(
                    reason,
                    connection::rustls::Error::InvalidCertificate(
                        connection::rustls::CertificateError::NotValidForNameContext { .. }
                    )
                ),
                "expected a name mismatch, got {reason:?}",
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn offers_channel_binding_over_tls() {
    let backend = ociman::test_backend_setup!();

    tls_definition(backend)
        .with_container(async |container| {
            let config = &container.session_config();
            let stream = connect_tls(config, CERTIFICATE_NAME).await.unwrap();

            let mut connection =
                connection::Connection::new(stream, protocol::ReceiveBufferSize::DEFAULT);

            connection
                .queue_send(frontend::StartupMessage {
                    version: protocol::Version::V3_2,
                    user: &config.authentication.user,
                    database: Some(&config.database),
                    replication: None,
                    parameters: &parameter::Map::new(),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::Authentication(backend::Authentication::Sasl { mechanisms })) =
                backend::Message::decode(&frame)
            else {
                panic!(
                    "expected a SASL request, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            assert_eq!(
                mechanisms.into_iter().collect::<Vec<_>>(),
                vec![
                    backend::Mechanism::Known(authentication::Mechanism::ScramSha256Plus),
                    backend::Mechanism::Known(authentication::Mechanism::ScramSha256),
                ],
            );

            terminate_during_authentication(connection).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn authenticates_with_channel_binding() {
    let backend = ociman::test_backend_setup!();

    tls_definition(backend)
        .with_container(async |container| {
            let config = &container.session_config();
            let stream = connect_tls(config, CERTIFICATE_NAME).await.unwrap();

            let binding = channel_binding(&stream);

            let mut connection =
                connection::Connection::new(stream, protocol::ReceiveBufferSize::DEFAULT);

            connection
                .queue_send(frontend::StartupMessage {
                    version: protocol::Version::V3_2,
                    user: &config.authentication.user,
                    database: Some(&config.database),
                    replication: None,
                    parameters: &parameter::Map::new(),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::Authentication(backend::Authentication::Sasl { mechanisms })) =
                backend::Message::decode(&frame)
            else {
                panic!(
                    "expected a SASL request, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            assert!(mechanisms.into_iter().any(|offered| offered
                == backend::Mechanism::Known(authentication::Mechanism::ScramSha256Plus)));

            let password = password(config);

            let (scram, client_first) = authentication::Scram::new(
                password.as_str(),
                authentication::Scram::nonce(),
                authentication::ClientBinding::Bound(binding),
            );

            connection
                .queue_send(frontend::SaslInitialResponse {
                    mechanism: authentication::Mechanism::ScramSha256Plus,
                    initial: Some(&client_first),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::Authentication(backend::Authentication::SaslContinue {
                data,
            })) = backend::Message::decode(&frame)
            else {
                panic!(
                    "expected a challenge, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            let (verifier, client_final) = scram.respond(data).unwrap();

            connection
                .queue_send(frontend::SaslResponse {
                    payload: &client_final,
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::Authentication(backend::Authentication::SaslFinal { data })) =
                backend::Message::decode(&frame)
            else {
                panic!(
                    "expected a final message, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            verifier.verify(data).unwrap();

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::Authentication(
                    backend::Authentication::Ok
                )),
            );

            let status = loop {
                let frame = connection.receive().await.unwrap();

                match backend::Message::decode(&frame).unwrap() {
                    backend::Message::ReadyForQuery(status) => break status,
                    backend::Message::ParameterStatus(_) => (),
                    backend::Message::BackendKeyData(_) => (),
                    other => panic!("unexpected message during startup: {other:?}"),
                }
            };

            assert_eq!(status, backend::TransactionStatus::Idle);

            terminate(connection).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn rejects_a_proof_bound_to_another_server() {
    let backend = ociman::test_backend_setup!();

    let elsewhere = tls_definition_named(backend.clone(), "elsewhere");
    let genuine = tls_definition_named(backend, "genuine");

    elsewhere
        .with_container(async |elsewhere| {
            let stolen = channel_binding(
                &connect_tls(&elsewhere.session_config(), CERTIFICATE_NAME)
                    .await
                    .unwrap(),
            );

            genuine
                .with_container(async |genuine| {
                    let config = &genuine.session_config();
                    let stream = connect_tls(config, CERTIFICATE_NAME).await.unwrap();

                    assert_ne!(stolen, channel_binding(&stream));

                    let mut connection =
                        connection::Connection::new(stream, protocol::ReceiveBufferSize::DEFAULT);

                    connection
                        .queue_send(frontend::StartupMessage {
                            version: protocol::Version::V3_2,
                            user: &config.authentication.user,
                            database: Some(&config.database),
                            replication: None,
                            parameters: &parameter::Map::new(),
                        })
                        .unwrap();

                    connection.flush_send().await.unwrap();

                    let frame = connection.receive().await.unwrap();

                    let Ok(backend::Message::Authentication(backend::Authentication::Sasl {
                        ..
                    })) = backend::Message::decode(&frame)
                    else {
                        panic!(
                            "expected a SASL request, got {:?}",
                            backend::Message::decode(&frame)
                        );
                    };

                    let password = password(config);

                    let (scram, client_first) = authentication::Scram::new(
                        password.as_str(),
                        authentication::Scram::nonce(),
                        authentication::ClientBinding::Bound(stolen.clone()),
                    );

                    connection
                        .queue_send(frontend::SaslInitialResponse {
                            mechanism: authentication::Mechanism::ScramSha256Plus,
                            initial: Some(&client_first),
                        })
                        .unwrap();

                    connection.flush_send().await.unwrap();

                    let frame = connection.receive().await.unwrap();

                    let Ok(backend::Message::Authentication(
                        backend::Authentication::SaslContinue { data },
                    )) = backend::Message::decode(&frame)
                    else {
                        panic!(
                            "expected a challenge, got {:?}",
                            backend::Message::decode(&frame)
                        );
                    };

                    let (_, client_final) = scram.respond(data).unwrap();

                    connection
                        .queue_send(frontend::SaslResponse {
                            payload: &client_final,
                        })
                        .unwrap();

                    connection.flush_send().await.unwrap();

                    let frame = connection.receive().await.unwrap();

                    let Ok(backend::Message::ErrorResponse(backend::ErrorResponse { diagnostic })) =
                        backend::Message::decode(&frame)
                    else {
                        panic!(
                            "expected the binding check to fail, got {:?}",
                            backend::Message::decode(&frame)
                        );
                    };

                    assert_eq!(diagnostic.severity(), backend::Severity::Fatal);
                    assert_eq!(
                        diagnostic.code(),
                        backend::SqlState::InvalidAuthorizationSpecification,
                    );
                    assert_eq!(diagnostic.message(), "SCRAM channel binding check failed");

                    match connection.receive().await {
                        Err(protocol::ReceiveError::Closed) => (),
                        other => panic!("expected the server to close, got {other:?}"),
                    }
                })
                .await
                .unwrap();
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn describes_a_statement_that_returns_no_rows() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .with_container(async |container| {
            let (mut connection, _) = authenticate(&container.session_config()).await;

            run(
                &mut connection,
                &pg_client::Sql::from_static_or_panic("CREATE TABLE example (id int, word text)"),
            )
            .await;

            let statement = frontend::StatementName::Unnamed;

            connection
                .queue_send(frontend::Parse {
                    statement: &statement,
                    sql: &pg_client::Sql::from_static_or_panic(
                        "INSERT INTO example (id, word) VALUES ($1, $2)",
                    ),
                    parameter_types: &[],
                })
                .unwrap();

            connection
                .queue_send(frontend::Describe {
                    target: &frontend::Target::Statement(statement.clone()),
                })
                .unwrap();

            connection.queue_send(frontend::Sync).unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::ParseComplete),
            );

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::ParameterDescription(
                    backend::ParameterDescription {
                        types: vec![protocol::Oid::new(23), protocol::Oid::new(25)],
                    }
                )),
            );

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::NoData)
            );

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::ReadyForQuery(
                    backend::TransactionStatus::Idle
                )),
            );

            terminate(connection).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn answers_an_empty_query_without_a_command_tag() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .with_container(async |container| {
            let (mut connection, _) = authenticate(&container.session_config()).await;

            connection
                .queue_send(frontend::Query {
                    sql: &pg_client::Sql::EMPTY,
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::EmptyQueryResponse),
            );

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::ReadyForQuery(
                    backend::TransactionStatus::Idle
                )),
            );

            let statement = frontend::StatementName::Unnamed;
            let portal = frontend::PortalName::Unnamed;

            // Not an empty string this time: a comment is not a statement, so
            // the emptiness is of the query, not of the text.
            connection
                .queue_send(frontend::Parse {
                    statement: &statement,
                    sql: &pg_client::Sql::from_static_or_panic("-- nothing to run"),
                    parameter_types: &[],
                })
                .unwrap();

            connection
                .queue_send(frontend::Bind::<&[u8]> {
                    portal: &portal,
                    statement: &statement,
                    parameter_formats: frontend::Formats::Default,
                    parameters: &[],
                    result_formats: frontend::Formats::Default,
                })
                .unwrap();

            connection
                .queue_send(frontend::Execute {
                    portal: &portal,
                    max_rows: None,
                })
                .unwrap();

            connection.queue_send(frontend::Sync).unwrap();

            connection.flush_send().await.unwrap();

            let mut seen = Vec::new();

            let status = loop {
                let frame = connection.receive().await.unwrap();

                match backend::Message::decode(&frame).unwrap() {
                    backend::Message::ReadyForQuery(status) => break status,
                    backend::Message::ParseComplete => seen.push("ParseComplete"),
                    backend::Message::BindComplete => seen.push("BindComplete"),
                    backend::Message::EmptyQueryResponse => seen.push("EmptyQueryResponse"),
                    other => panic!("unexpected message: {other:?}"),
                }
            };

            assert_eq!(
                seen,
                vec!["ParseComplete", "BindComplete", "EmptyQueryResponse"],
            );

            assert_eq!(status, backend::TransactionStatus::Idle);

            terminate(connection).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn confirms_a_parse_before_ending_the_batch() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .with_container(async |container| {
            let (mut connection, _) = authenticate(&container.session_config()).await;

            let statement = frontend::StatementName::Unnamed;

            connection
                .queue_send(frontend::Parse {
                    statement: &statement,
                    sql: &pg_client::Sql::from_static_or_panic("SELECT $1::int"),
                    parameter_types: &[],
                })
                .unwrap();

            connection.queue_send(frontend::Flush).unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::ParseComplete),
            );

            connection.queue_send(frontend::Sync).unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::ReadyForQuery(
                    backend::TransactionStatus::Idle
                )),
            );

            terminate(connection).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn pages_a_portal_with_a_row_limit() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .with_container(async |container| {
            let (mut connection, _) = authenticate(&container.session_config()).await;

            let statement = frontend::StatementName::Unnamed;
            let portal = frontend::PortalName::Unnamed;
            let two = core::num::NonZeroU32::new(2).and_then(frontend::RowLimit::new);

            connection
                .queue_send(frontend::Parse {
                    statement: &statement,
                    sql: &pg_client::Sql::from_static_or_panic(
                        "SELECT * FROM generate_series(1, 5)",
                    ),
                    parameter_types: &[],
                })
                .unwrap();

            connection
                .queue_send(frontend::Bind::<&[u8]> {
                    portal: &portal,
                    statement: &statement,
                    parameter_formats: frontend::Formats::Default,
                    parameters: &[],
                    result_formats: frontend::Formats::Default,
                })
                .unwrap();

            connection
                .queue_send(frontend::Execute {
                    portal: &portal,
                    max_rows: two,
                })
                .unwrap();

            connection.queue_send(frontend::Flush).unwrap();

            connection.flush_send().await.unwrap();

            let mut seen = Vec::new();
            let mut rows: Vec<Vec<u8>> = Vec::new();

            loop {
                let frame = connection.receive().await.unwrap();

                match backend::Message::decode(&frame).unwrap() {
                    backend::Message::ParseComplete => seen.push("ParseComplete".to_owned()),
                    backend::Message::BindComplete => seen.push("BindComplete".to_owned()),
                    backend::Message::DataRow(row) => {
                        rows.push(row.into_iter().next().unwrap().unwrap().to_vec());
                    }
                    backend::Message::PortalSuspended => break,
                    other => panic!("unexpected message: {other:?}"),
                }
            }

            assert_eq!(seen, vec!["ParseComplete", "BindComplete"]);
            assert_eq!(rows, vec![b"1".to_vec(), b"2".to_vec()]);

            connection
                .queue_send(frontend::Execute {
                    portal: &portal,
                    max_rows: two,
                })
                .unwrap();

            connection.queue_send(frontend::Flush).unwrap();

            connection.flush_send().await.unwrap();

            rows.clear();

            loop {
                let frame = connection.receive().await.unwrap();

                match backend::Message::decode(&frame).unwrap() {
                    backend::Message::DataRow(row) => {
                        rows.push(row.into_iter().next().unwrap().unwrap().to_vec());
                    }
                    backend::Message::PortalSuspended => break,
                    other => panic!("unexpected message: {other:?}"),
                }
            }

            assert_eq!(rows, vec![b"3".to_vec(), b"4".to_vec()]);

            connection
                .queue_send(frontend::Execute {
                    portal: &portal,
                    max_rows: two,
                })
                .unwrap();

            connection.queue_send(frontend::Flush).unwrap();

            connection.flush_send().await.unwrap();

            rows.clear();

            let counted = loop {
                let frame = connection.receive().await.unwrap();

                match backend::Message::decode(&frame).unwrap() {
                    backend::Message::DataRow(row) => {
                        rows.push(row.into_iter().next().unwrap().unwrap().to_vec());
                    }
                    backend::Message::CommandComplete(backend::CommandComplete::Select {
                        rows,
                    }) => break rows,
                    other => panic!("unexpected message: {other:?}"),
                }
            };

            assert_eq!(rows, vec![b"5".to_vec()]);

            // One, not the portal's five: the tag counts what this `Execute`
            // produced. Anything paging a portal keeps its own total.
            assert_eq!(counted, 1);

            connection
                .queue_send(frontend::Close {
                    target: &frontend::Target::Portal(portal.clone()),
                })
                .unwrap();

            connection.queue_send(frontend::Sync).unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::CloseComplete),
            );

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::ReadyForQuery(
                    backend::TransactionStatus::Idle
                )),
            );

            terminate(connection).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn delivers_a_notification() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .with_container(async |container| {
            let (mut connection, key) = authenticate(&container.session_config()).await;

            run(
                &mut connection,
                &pg_client::Sql::from_static_or_panic("LISTEN example"),
            )
            .await;

            connection
                .queue_send(frontend::Query {
                    sql: &pg_client::Sql::from_static_or_panic("NOTIFY example, 'payload'"),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let mut order = Vec::new();

            loop {
                let frame = connection.receive().await.unwrap();

                match backend::Message::decode(&frame).unwrap() {
                    backend::Message::NotificationResponse(notification) => {
                        // Our own backend: `NOTIFY` reaches a session listening
                        // on the channel even when that session sent it.
                        assert_eq!(notification.process_id, key.process_id());
                        assert_eq!(notification.channel.as_str(), "example");
                        assert_eq!(notification.payload, "payload");

                        order.push("NotificationResponse");
                    }
                    backend::Message::CommandComplete(_) => order.push("CommandComplete"),
                    backend::Message::ReadyForQuery(_) => {
                        order.push("ReadyForQuery");

                        break;
                    }
                    other => panic!("unexpected message: {other:?}"),
                }
            }

            assert_eq!(
                order,
                vec!["CommandComplete", "NotificationResponse", "ReadyForQuery"],
            );

            terminate(connection).await;
        })
        .await
        .unwrap()
}

/// Depends on the server's default `client_min_messages` of `notice`.
#[tokio::test]
async fn reports_a_truncated_identifier_as_a_notice() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .with_container(async |container| {
            let (mut connection, _) = authenticate(&container.session_config()).await;

            connection.queue_send(frontend::Query {
                sql: &pg_client::Sql::from_static_or_panic("SELECT 1 AS aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
            }).unwrap();

            connection.flush_send().await.unwrap();

            let mut order = Vec::new();

            loop {
                let frame = connection.receive().await.unwrap();

                match backend::Message::decode(&frame).unwrap() {
                    backend::Message::NoticeResponse(backend::NoticeResponse { diagnostic }) => {
                        assert_eq!(diagnostic.severity(), backend::Severity::Notice);
                        assert_eq!(diagnostic.code(), backend::SqlState::NameTooLong);

                        order.push("NoticeResponse");
                    }
                    backend::Message::RowDescription(description) => {
                        let name = description.into_iter().next().unwrap().name;

                        assert_eq!(name.as_str(), "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
                        assert_eq!(name.as_str().len(), 63);

                        order.push("RowDescription");
                    }
                    backend::Message::DataRow(_) => order.push("DataRow"),
                    backend::Message::CommandComplete(_) => order.push("CommandComplete"),
                    backend::Message::ReadyForQuery(_) => {
                        order.push("ReadyForQuery");

                        break;
                    }
                    other => panic!("unexpected message: {other:?}"),
                }
            }

            assert_eq!(
                order,
                vec![
                    "NoticeResponse",
                    "RowDescription",
                    "DataRow",
                    "CommandComplete",
                    "ReadyForQuery",
                ],
            );

            terminate(connection).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn offers_a_downgrade_to_3_0() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .image(pg_ephemeral::Image::OfficialRelease {
            major: pg_ephemeral::image::Major::new(17),
            minor: pg_ephemeral::image::Minor::Latest,
            os: pg_ephemeral::image::OS::Default,
            digest: None,
        })
        .with_container(async |container| {
            let config = &container.session_config();

            let mut connection = connection::Connection::new(
                connection::Stream::Tcp(connect(config).await),
                protocol::ReceiveBufferSize::DEFAULT,
            );

            connection
                .queue_send(frontend::StartupMessage {
                    version: protocol::Version::V3_2,
                    user: &config.authentication.user,
                    database: Some(&config.database),
                    replication: None,
                    parameters: &parameter::Map::new(),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::NegotiateProtocolVersion(
                    backend::NegotiateProtocolVersion {
                        version: protocol::Version::V3_0,
                    },
                )),
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn abandons_a_copy_in() {
    let backend = ociman::test_backend_setup!();

    definition(backend)
        .with_container(async |container| {
            let (mut connection, _) = authenticate(&container.session_config()).await;

            run(
                &mut connection,
                &pg_client::Sql::from_static_or_panic("CREATE TABLE example (id int, word text)"),
            )
            .await;

            connection
                .queue_send(frontend::Query {
                    sql: &pg_client::Sql::from_static_or_panic("COPY example FROM STDIN"),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::CopyInResponse(_)) = backend::Message::decode(&frame) else {
                panic!(
                    "expected a copy-in response, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            connection
                .queue_send(frontend::CopyData { data: b"1\tone\n" })
                .unwrap();

            connection
                .queue_send(frontend::CopyFail {
                    message: &pg_client::protocol::String::from_static_or_panic("changed my mind"),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let frame = connection.receive().await.unwrap();

            let Ok(backend::Message::ErrorResponse(backend::ErrorResponse { diagnostic })) =
                backend::Message::decode(&frame)
            else {
                panic!(
                    "expected the copy to fail, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            assert_eq!(diagnostic.severity(), backend::Severity::Error);
            assert_eq!(diagnostic.code(), backend::SqlState::QueryCanceled);
            assert_eq!(
                diagnostic.message(),
                "COPY from stdin failed: changed my mind",
            );

            let frame = connection.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame),
                Ok(backend::Message::ReadyForQuery(
                    backend::TransactionStatus::Idle
                )),
            );

            // The row that was already sent is gone with it: `CopyFail` fails
            // the statement rather than stopping it partway.
            connection
                .queue_send(frontend::Query {
                    sql: &pg_client::Sql::from_static_or_panic("SELECT count(*) FROM example"),
                })
                .unwrap();

            connection.flush_send().await.unwrap();

            let mut counted = None;

            loop {
                let frame = connection.receive().await.unwrap();

                match backend::Message::decode(&frame).unwrap() {
                    backend::Message::DataRow(row) => {
                        counted = row
                            .into_iter()
                            .next()
                            .unwrap()
                            .map(|value| String::from_utf8(value.to_vec()).unwrap());
                    }
                    backend::Message::RowDescription(_) => (),
                    backend::Message::CommandComplete(_) => (),
                    backend::Message::ReadyForQuery(_) => break,
                    other => panic!("unexpected message: {other:?}"),
                }
            }

            assert_eq!(counted.as_deref(), Some("0"));

            terminate(connection).await;
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_a_handshake_that_did_not_select_the_protocol() {
    let backend = ociman::test_backend_setup!();

    tls_definition(backend)
        .with_container(async |container| {
            let config = &container.session_config();

            let mut client = trust(config);

            client.alpn_protocols.clear();

            let error = connect_tls_with(config, CERTIFICATE_NAME, client)
                .await
                .unwrap_err();

            assert!(
                matches!(&error, connection::TlsError::Alpn { selected } if selected.is_none()),
                "expected no protocol to have been selected, got {error:?}",
            );
        })
        .await
        .unwrap()
}
