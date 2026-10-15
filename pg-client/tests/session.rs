use pg_client::protocol::{backend, frontend};
use pg_client::{protocol, session};

const CERTIFICATE_NAME: &str = "pg-ephemeral.test";

fn definition(backend: ociman::Backend, instance: &str) -> pg_ephemeral::Definition {
    pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        instance.parse().unwrap(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30))
}

#[tokio::test]
async fn connects_in_cleartext() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "cleartext")
        .with_container(async |container| {
            session::start(container.session_config()).await.unwrap();
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn connects_with_required_channel_binding() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "bound")
        .ssl_config(pg_ephemeral::definition::SslConfig::Generated {
            hostname: CERTIFICATE_NAME.parse().unwrap(),
        })
        .with_container(async |container| {
            session::start(container.session_config()).await.unwrap();
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn runs_a_statement_with_a_bound_parameter() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "statement")
        .with_session(async |session| {
            let statement = session
                .queue_parse(
                    frontend::StatementName::Unnamed,
                    &frontend::Sql::from_static_or_panic("SELECT $1::int4 + 1"),
                    &[],
                )
                .unwrap();

            let portal = session
                .queue_bind(
                    &statement,
                    frontend::PortalName::Unnamed,
                    frontend::Formats::Default,
                    &[Some(b"41".as_slice())],
                    frontend::Formats::Default,
                )
                .unwrap();

            session.queue_execute(&portal, None).unwrap();
            session.queue_sync().unwrap();

            let mut seen = Vec::new();

            loop {
                let frame = session.receive().await.unwrap();

                match backend::Message::decode(&frame).unwrap() {
                    backend::Message::ParseComplete => seen.push("ParseComplete"),
                    backend::Message::BindComplete => seen.push("BindComplete"),
                    backend::Message::DataRow(row) => {
                        assert_eq!(
                            row.into_iter().collect::<Vec<_>>(),
                            vec![Some(b"42".as_slice())]
                        );
                        seen.push("DataRow");
                    }
                    backend::Message::CommandComplete(tag) => {
                        assert_eq!(tag, backend::CommandComplete::Select { rows: 1 });
                        seen.push("CommandComplete");
                    }
                    backend::Message::ReadyForQuery(status) => {
                        assert_eq!(status, backend::TransactionStatus::Idle);
                        seen.push("ReadyForQuery");
                        break;
                    }
                    other => panic!("unexpected {other:?}"),
                }
            }

            assert_eq!(
                seen,
                [
                    "ParseComplete",
                    "BindComplete",
                    "DataRow",
                    "CommandComplete",
                    "ReadyForQuery"
                ]
            );
            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );
        })
        .await
        .unwrap()
}

/// The backend combines replies "into the minimum possible number of packets",
/// so without a `Flush` this `ParseComplete` would not arrive until `Sync`.
#[tokio::test]
async fn sees_a_reply_without_ending_the_cycle() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "flush")
        .with_session(async |session| {
            session
                .queue_parse(
                    frontend::StatementName::Unnamed,
                    &frontend::Sql::from_static_or_panic("SELECT 1"),
                    &[],
                )
                .unwrap();

            session.queue_flush().unwrap();

            let frame = session.receive().await.unwrap();

            assert_eq!(
                backend::Message::decode(&frame).unwrap(),
                backend::Message::ParseComplete
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn describes_the_parameters_and_rows_of_a_statement() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "describe")
        .with_session(async |session| {
            let statement = session
                .queue_parse(
                    frontend::StatementName::Unnamed,
                    &frontend::Sql::from_static_or_panic("SELECT $1"),
                    &[protocol::Oid::new(23)],
                )
                .unwrap();

            session.queue_describe_statement(&statement).unwrap();
            session.queue_flush().unwrap();

            let mut seen = Vec::new();

            for _ in 0..3 {
                let frame = session.receive().await.unwrap();

                match backend::Message::decode(&frame).unwrap() {
                    backend::Message::ParseComplete => seen.push("ParseComplete"),
                    backend::Message::ParameterDescription(description) => {
                        assert_eq!(description.types, vec![protocol::Oid::new(23)]);
                        seen.push("ParameterDescription");
                    }
                    backend::Message::RowDescription(description) => {
                        let columns: Vec<_> = description.into_iter().collect();

                        assert_eq!(columns.len(), 1);
                        assert_eq!(columns[0].data_type, protocol::Oid::new(23));
                        seen.push("RowDescription");
                    }
                    other => panic!("unexpected {other:?}"),
                }
            }

            assert_eq!(
                seen,
                ["ParseComplete", "ParameterDescription", "RowDescription"]
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn runs_a_simple_query_with_two_statements() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "query")
        .with_session(async |session| {
            session
                .queue_query(&frontend::Sql::from_static_or_panic("SELECT 1; SELECT 2"))
                .unwrap();

            let mut seen = Vec::new();

            loop {
                let frame = session.receive().await.unwrap();

                match backend::Message::decode(&frame).unwrap() {
                    backend::Message::RowDescription(_) => seen.push("RowDescription"),
                    backend::Message::DataRow(row) => {
                        match row.into_iter().collect::<Vec<_>>().as_slice() {
                            [Some(b"1")] => seen.push("DataRow 1"),
                            [Some(b"2")] => seen.push("DataRow 2"),
                            other => panic!("unexpected row {other:?}"),
                        }
                    }
                    backend::Message::CommandComplete(tag) => {
                        assert_eq!(tag, backend::CommandComplete::Select { rows: 1 });
                        seen.push("CommandComplete");
                    }
                    backend::Message::ReadyForQuery(status) => {
                        assert_eq!(status, backend::TransactionStatus::Idle);
                        seen.push("ReadyForQuery");
                        break;
                    }
                    other => panic!("unexpected {other:?}"),
                }
            }

            assert_eq!(
                seen,
                [
                    "RowDescription",
                    "DataRow 1",
                    "CommandComplete",
                    "RowDescription",
                    "DataRow 2",
                    "CommandComplete",
                    "ReadyForQuery"
                ]
            );
            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn executes_a_statement_and_reads_its_rows() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "execute")
        .with_session(async |session| {
            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT generate_series(1, $1)",
                ))
                .bind(2i32)
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let column = description.column::<i32>(0).unwrap();

            let mut values = Vec::new();

            while let Some(row) = reader.next().await.unwrap() {
                values.push(row.decode(&column).unwrap());
            }

            assert_eq!(values, [1, 2]);

            let Some(session::execute::Outcome::Complete(completion)) = reader.outcome() else {
                panic!("expected a completed command, got {:?}", reader.outcome());
            };

            assert_eq!(
                completion.command(),
                backend::CommandComplete::Select { rows: 2 }
            );

            drop(reader);

            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn binds_an_array_parameter() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "array")
        .with_session(async |session| {
            let names = pg_client::value::Array::new(
                vec![pg_client::value::Dimension {
                    length: 3,
                    lower_bound: 1,
                }],
                vec![Some("a".to_owned()), None, Some("c".to_owned())],
            )
            .unwrap();

            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT array_length($1, 1), array_position($1, 'c')",
                ))
                .bind(&names)
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let length = description.column::<i32>(0).unwrap();
            let position = description.column::<i32>(1).unwrap();

            let row = reader.next().await.unwrap().unwrap();

            assert_eq!(
                (row.decode(&length).unwrap(), row.decode(&position).unwrap()),
                (3, 3)
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn binds_a_typed_null() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "null")
        .with_session(async |session| {
            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT $1 IS NULL, pg_typeof($1) = 'bigint'::regtype",
                ))
                .bind(None::<i64>)
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let null = description.column::<bool>(0).unwrap();
            let bigint = description.column::<bool>(1).unwrap();

            let row = reader.next().await.unwrap().unwrap();

            assert_eq!(
                (row.decode(&null).unwrap(), row.decode(&bigint).unwrap()),
                (true, true)
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn decodes_text_name_and_int8() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "decode")
        .with_session(async |session| {
            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT 'café'::text, 'pg_class'::name, 9000000000::int8",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let text = description.column::<String>(0).unwrap();
            let name = description.column::<String>(1).unwrap();
            let int8 = description.column::<i64>(2).unwrap();

            let row = reader.next().await.unwrap().unwrap();

            assert_eq!(row.decode(&text).unwrap(), "café");
            assert_eq!(row.decode(&name).unwrap(), "pg_class");
            assert_eq!(row.decode(&int8).unwrap(), 9_000_000_000);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_a_name_that_is_not_a_valid_identifier() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "decode-invalid-name")
        .with_session(async |session| {
            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic("SELECT ''::name"))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let table = description
                .column::<pg_client::name::Table<'static>>(0)
                .unwrap();

            let row = reader.next().await.unwrap().unwrap();

            let error = row.decode(&table).unwrap_err();

            let session::execute::DecodeError::Value {
                index: 0,
                source: pg_client::value::DecodeError::InvalidName(_),
            } = error
            else {
                panic!("expected an invalid name error on column 0, got {error:?}");
            };
        })
        .await
        .unwrap()
}

#[cfg(feature = "serde")]
#[tokio::test]
async fn decodes_json_and_jsonb() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "decode-json")
        .with_session(async |session| {
            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT '{\"a\":1}'::json, '{\"a\":1}'::jsonb",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let json = description
                .column::<pg_client::value::Json<serde_json::Value>>(0)
                .unwrap();
            let jsonb = description
                .column::<pg_client::value::Jsonb<serde_json::Value>>(1)
                .unwrap();

            let row = reader.next().await.unwrap().unwrap();

            assert_eq!(
                row.decode(&json).unwrap(),
                pg_client::value::Json(serde_json::json!({"a": 1}))
            );
            assert_eq!(
                row.decode(&jsonb).unwrap(),
                pg_client::value::Jsonb(serde_json::json!({"a": 1}))
            );
        })
        .await
        .unwrap()
}

#[cfg(feature = "serde")]
#[tokio::test]
async fn refuses_jsonb_that_does_not_match_the_target_type() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "decode-json-mismatch")
        .with_session(async |session| {
            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic("SELECT '[1]'::jsonb"))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let jsonb = description
                .column::<pg_client::value::Jsonb<std::collections::BTreeMap<String, i32>>>(0)
                .unwrap();

            let row = reader.next().await.unwrap().unwrap();

            let error = row.decode(&jsonb).unwrap_err();

            let session::execute::DecodeError::Value {
                index: 0,
                source: pg_client::value::DecodeError::Json(_),
            } = error
            else {
                panic!("expected a JSON decode error on column 0, got {error:?}");
            };
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn executes_a_statement_the_server_generated() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "generated")
        .with_session(async |session| {
            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT format('SELECT count(*) FROM %I', 'pg_namespace')",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let column = description.column::<frontend::Sql>(0).unwrap();

            let generated: frontend::Sql = reader
                .next()
                .await
                .unwrap()
                .unwrap()
                .decode(&column)
                .unwrap();

            drop(reader);

            assert_eq!(generated.as_str(), "SELECT count(*) FROM pg_namespace");

            let mut reader = session.statement(&generated).execute().unwrap();
            let description = reader.describe().await.unwrap();
            let count = description.column::<i64>(0).unwrap();

            let row = reader.next().await.unwrap().unwrap();

            assert!(row.decode(&count).unwrap() > 0);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn reads_a_column_by_name() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "column")
        .with_session(async |session| {
            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT n AS number, NULL::int4 AS nothing FROM generate_series(1, 3) AS n",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();

            let number = description
                .column::<i32>(pg_client::name::Column::from_static_or_panic("number"))
                .unwrap();
            let nothing = description
                .column::<Option<i32>>(pg_client::name::Column::from_static_or_panic("nothing"))
                .unwrap();

            let mut values = Vec::new();

            while let Some(row) = reader.next().await.unwrap() {
                values.push((row.decode(&number).unwrap(), row.decode(&nothing).unwrap()));
            }

            assert_eq!(values, [(1, None), (2, None), (3, None)]);

            drop(reader);

            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_one_row_when_there_are_none() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "one-of-none")
        .with_session(async |session| {
            let error = session
                .statement(&frontend::Sql::from_static_or_panic("SELECT 1 WHERE false"))
                .execute()
                .unwrap()
                .decode(|_row| Ok(()))
                .one()
                .await
                .unwrap_err();

            let session::execute::Error::NoRows = error else {
                panic!("expected a no rows error, got {error:?}");
            };

            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_one_row_when_there_are_several() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "one-of-several")
        .with_session(async |session| {
            let error = session
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT generate_series(1, 3)",
                ))
                .execute()
                .unwrap()
                .decode(|_row| Ok(()))
                .one()
                .await
                .unwrap_err();

            let session::execute::Error::TooManyRows { actual } = error else {
                panic!("expected a too many rows error, got {error:?}");
            };

            assert_eq!(actual, 3);
            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn reads_no_row_as_none() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "max-one-of-none")
        .with_session(async |session| {
            let value = session
                .statement(&frontend::Sql::from_static_or_panic("SELECT 1 WHERE false"))
                .execute()
                .unwrap()
                .decode(|_row| Ok(()))
                .max_one()
                .await
                .unwrap();

            assert_eq!(value, None);
            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn reads_a_single_row_as_some() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "max-one-of-one")
        .with_session(async |session| {
            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic("SELECT 42::int4"))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let number = description.column::<i32>(0).unwrap();

            let value = reader
                .decode(|row| row.decode(&number))
                .max_one()
                .await
                .unwrap();

            assert_eq!(value, Some(42));
            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_max_one_row_when_there_are_several() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "max-one-of-several")
        .with_session(async |session| {
            let error = session
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT generate_series(1, 2)",
                ))
                .execute()
                .unwrap()
                .decode(|_row| Ok(()))
                .max_one()
                .await
                .unwrap_err();

            let session::execute::Error::TooManyRows { actual } = error else {
                panic!("expected a too many rows error, got {error:?}");
            };

            assert_eq!(actual, 2);
            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn discards_rows() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "discard-rows")
        .with_session(async |session| {
            session
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT generate_series(1, 3)",
                ))
                .execute()
                .unwrap()
                .discard_rows()
                .await
                .unwrap();

            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn discarding_rows_reports_a_server_error() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "discard-rows-error")
        .with_session(async |session| {
            let error = session
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT 1 / (n - 2) FROM generate_series(1, 3) AS n",
                ))
                .execute()
                .unwrap()
                .discard_rows()
                .await
                .unwrap_err();

            let session::execute::Error::Server { code, .. } = error else {
                panic!("expected a server error, got {error:?}");
            };

            assert_eq!(code, backend::SqlState::DivisionByZero);
            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );

            session
                .statement(&frontend::Sql::from_static_or_panic("SELECT 1"))
                .execute()
                .unwrap()
                .discard_rows()
                .await
                .unwrap();
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn runs_several_statements_at_once() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "statements")
        .with_session(async |session| {
            session
                .statements(&frontend::Sql::from_static_or_panic(
                    "CREATE TABLE t (x int); INSERT INTO t VALUES (1), (2); SELECT * FROM t",
                ))
                .await
                .unwrap();

            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT count(*) FROM t",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let count = description.column::<i64>(0).unwrap();

            assert_eq!(
                reader.decode(|row| row.decode(&count)).one().await.unwrap(),
                2
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn stops_several_statements_at_the_first_error() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "statements-error")
        .with_session(async |session| {
            let error = session
                .statements(&frontend::Sql::from_static_or_panic(
                    "CREATE TABLE t (x int); SELECT 1 / 0; INSERT INTO t VALUES (1)",
                ))
                .await
                .unwrap_err();

            let session::execute::Error::Server { code, .. } = error else {
                panic!("expected a server error, got {error:?}");
            };

            assert_eq!(code, backend::SqlState::DivisionByZero);

            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT to_regclass('t') IS NULL",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let missing = description.column::<bool>(0).unwrap();

            let is_missing = reader
                .decode(|row| row.decode(&missing))
                .one()
                .await
                .unwrap();

            assert!(is_missing);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_copy_in_among_several_statements() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "statements-copy")
        .with_session(async |session| {
            let error = session
                .statements(&frontend::Sql::from_static_or_panic(
                    "CREATE TABLE t (x int); COPY t FROM STDIN",
                ))
                .await
                .unwrap_err();

            let session::execute::Error::Unexpected(frame) = error else {
                panic!("expected an unexpected message, got {error:?}");
            };

            let Ok(backend::Message::CopyInResponse(_)) = backend::Message::decode(&frame) else {
                panic!(
                    "expected a copy in response, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );

            session
                .statements(&frontend::Sql::from_static_or_panic("SELECT 1"))
                .await
                .unwrap();
        })
        .await
        .unwrap()
}

async fn count_rows(session: &mut session::Session) -> i64 {
    let mut reader = session
        .statement(&frontend::Sql::from_static_or_panic(
            "SELECT count(*) FROM t",
        ))
        .execute()
        .unwrap();

    let description = reader.describe().await.unwrap();
    let count = description.column::<i64>(0).unwrap();

    reader.decode(|row| row.decode(&count)).one().await.unwrap()
}

#[tokio::test]
async fn copies_rows_in_through_copy_in() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "copy-in")
        .with_session(async |session| {
            session
                .statements(&frontend::Sql::from_static_or_panic(
                    "CREATE TABLE t (x int)",
                ))
                .await
                .unwrap();

            let mut copy = session
                .copy_in(&frontend::Sql::from_static_or_panic(
                    "COPY t FROM STDIN (FORMAT csv)",
                ))
                .await
                .unwrap();

            copy.write(b"1\n2").await.unwrap();
            copy.write(b"\n3\n").await.unwrap();

            assert_eq!(copy.finish().await.unwrap(), 3);
            assert_eq!(count_rows(session).await, 3);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn reports_a_bad_row_when_finishing_a_copy() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "copy-in-bad-row")
        .with_session(async |session| {
            session
                .statements(&frontend::Sql::from_static_or_panic(
                    "CREATE TABLE t (x int)",
                ))
                .await
                .unwrap();

            let mut copy = session
                .copy_in(&frontend::Sql::from_static_or_panic(
                    "COPY t FROM STDIN (FORMAT csv)",
                ))
                .await
                .unwrap();

            copy.write(b"1\nnot a number\n3\n").await.unwrap();

            let error = copy.finish().await.unwrap_err();

            let session::execute::Error::Server { code, .. } = error else {
                panic!("expected a server error, got {error:?}");
            };

            assert_eq!(code, backend::SqlState::InvalidTextRepresentation);
            assert_eq!(count_rows(session).await, 0);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn aborts_a_copy() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "copy-in-abort")
        .with_session(async |session| {
            session
                .statements(&frontend::Sql::from_static_or_panic(
                    "CREATE TABLE t (x int)",
                ))
                .await
                .unwrap();

            let mut copy = session
                .copy_in(&frontend::Sql::from_static_or_panic(
                    "COPY t FROM STDIN (FORMAT csv)",
                ))
                .await
                .unwrap();

            copy.write(b"1\n2\n").await.unwrap();

            copy.abort(&protocol::String::from_static_or_panic("changed my mind"))
                .await
                .unwrap();

            assert_eq!(count_rows(session).await, 0);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn discards_a_dropped_copy() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "copy-in-drop")
        .with_session(async |session| {
            session
                .statements(&frontend::Sql::from_static_or_panic(
                    "CREATE TABLE t (x int)",
                ))
                .await
                .unwrap();

            let mut copy = session
                .copy_in(&frontend::Sql::from_static_or_panic(
                    "COPY t FROM STDIN (FORMAT csv)",
                ))
                .await
                .unwrap();

            copy.write(b"1\n2\n").await.unwrap();

            drop(copy);

            assert_eq!(count_rows(session).await, 0);
            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_to_copy_in_with_a_statement_that_is_no_copy() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "copy-in-no-copy")
        .with_session(async |session| {
            let error = session
                .copy_in(&frontend::Sql::from_static_or_panic("SELECT 1"))
                .await
                .unwrap_err();

            let session::execute::Error::Unexpected(frame) = error else {
                panic!("expected an unexpected message, got {error:?}");
            };

            let Ok(backend::Message::RowDescription(_)) = backend::Message::decode(&frame) else {
                panic!(
                    "expected a row description, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            session
                .statements(&frontend::Sql::from_static_or_panic("SELECT 1"))
                .await
                .unwrap();
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn decodes_a_single_column() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "decode-column")
        .with_session(async |session| {
            let names = session
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT n AS number, 'n' || n AS name FROM generate_series(1, 2) AS n",
                ))
                .execute()
                .unwrap()
                .decode_column::<String>(pg_client::name::Column::from_static_or_panic("name"))
                .await
                .unwrap()
                .into_vec()
                .await
                .unwrap();

            assert_eq!(names, ["n1", "n2"]);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_to_decode_a_column_as_another_type() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "decode-column-type")
        .with_session(async |session| {
            let Err(error) = session
                .statement(&frontend::Sql::from_static_or_panic("SELECT 'text'::text"))
                .execute()
                .unwrap()
                .decode_column::<i32>(0)
                .await
            else {
                panic!("expected a column error");
            };

            let session::execute::Error::Column(session::execute::ColumnError::Type {
                index: 0,
                ..
            }) = error
            else {
                panic!("expected a column type error, got {error:?}");
            };
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn reports_the_fatal_error_that_ends_the_session() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "fatal")
        .with_container(async |container| {
            let config = container.session_config();

            let mut victim = session::start(std::sync::Arc::clone(&config))
                .await
                .unwrap();
            let pid = victim.backend_key().process_id.get();

            let sleeping = tokio::spawn(async move {
                victim
                    .statements(&frontend::Sql::from_static_or_panic(
                        "SELECT pg_sleep(3600)",
                    ))
                    .await
            });

            let mut observer = session::start(config).await.unwrap();

            loop {
                let active = observer
                    .statement(&frontend::Sql::from_static_or_panic(
                        "SELECT count(*) FROM pg_stat_activity WHERE pid = $1 AND state = 'active'",
                    ))
                    .bind(pid)
                    .execute()
                    .unwrap()
                    .decode_column::<i64>(0)
                    .await
                    .unwrap()
                    .one()
                    .await
                    .unwrap();

                if active == 1 {
                    break;
                }
            }

            observer
                .statement(&frontend::Sql::from_static_or_panic(
                    "SELECT pg_terminate_backend($1)",
                ))
                .bind(pid)
                .execute()
                .unwrap()
                .discard_rows()
                .await
                .unwrap();

            let error = sleeping.await.unwrap().unwrap_err();

            let session::execute::Error::Server { severity, code, .. } = error else {
                panic!("expected the server's fatal error, got {error:?}");
            };

            assert_eq!(
                (severity, code),
                (backend::Severity::Fatal, backend::SqlState::AdminShutdown)
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn negotiates_down_to_3_0() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "protocol-3-0")
        .image(pg_ephemeral::Image::OfficialRelease {
            major: pg_ephemeral::image::Major::new(17),
            minor: pg_ephemeral::image::Minor::Latest,
            os: pg_ephemeral::image::OS::Default,
            digest: None,
        })
        .with_session(async |session| {
            assert_eq!(session.version(), protocol::Version::V3_0);
            assert_eq!(session.backend_key().cancel_key.as_slice().len(), 4);

            let one = session
                .statement(&frontend::Sql::from_static_or_panic("SELECT 1::int4"))
                .execute()
                .unwrap()
                .decode_column::<i32>(0)
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(one, 1);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_a_protocol_version_it_does_not_support() {
    use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};

    let listener = tokio::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0))
        .await
        .unwrap();
    let port = listener.local_addr().unwrap().port();

    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();

        let mut length = [0; 4];
        socket.read_exact(&mut length).await.unwrap();

        let mut startup = vec![0; usize::try_from(u32::from_be_bytes(length)).unwrap() - 4];
        socket.read_exact(&mut startup).await.unwrap();

        socket
            .write_all(
                &[
                    b"v".as_slice(),
                    &12u32.to_be_bytes(),
                    &0x0003_0003u32.to_be_bytes(),
                    &0u32.to_be_bytes(),
                ]
                .concat(),
            )
            .await
            .unwrap();
    });

    let config = session::Config {
        connection: pg_client::connection::Config {
            endpoint: pg_client::connection::Endpoint::Tcp {
                host: pg_client::config::Host::IpAddr(std::net::Ipv4Addr::LOCALHOST.into()),
                address: None,
                port: pg_client::config::Port::new(port),
                tls: None,
            },
            receive_buffer: protocol::ReceiveBufferSize::DEFAULT,
        },
        authentication: pg_client::authentication::Config {
            user: pg_client::User::POSTGRES,
            password: pg_client::authentication::Password::Static("unused".parse().unwrap()),
        },
        database: pg_client::Database::POSTGRES,
        parameters: session::Parameters::new(),
    };

    let error = session::start(std::sync::Arc::new(config))
        .await
        .unwrap_err();

    let session::StartError::UnsupportedProtocol { code } = error else {
        panic!("expected an unsupported protocol error, got {error:?}");
    };

    assert_eq!(code, 0x0003_0003);

    server.await.unwrap();
}

#[tokio::test]
async fn refuses_a_notification_as_an_unexpected_message() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "notification")
        .with_session(async |session| {
            run(session, "LISTEN example").await;

            let mut reader = session
                .statement(&frontend::Sql::from_static_or_panic(
                    "NOTIFY example, 'payload'",
                ))
                .execute()
                .unwrap();

            let error = reader.next().await.unwrap_err();

            drop(reader);

            let session::execute::Error::Unexpected(frame) = error else {
                panic!("expected an unexpected message, got {error:?}");
            };

            let Ok(backend::Message::NotificationResponse(notification)) =
                backend::Message::decode(&frame)
            else {
                panic!(
                    "expected a notification, got {:?}",
                    backend::Message::decode(&frame)
                );
            };

            assert_eq!(notification.channel.as_str(), "example");
            assert_eq!(notification.payload, "payload");
            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );
        })
        .await
        .unwrap()
}

async fn run(session: &mut session::Session, sql: &'static str) {
    let statement = session
        .queue_parse(
            frontend::StatementName::Unnamed,
            &frontend::Sql::from_static_or_panic(sql),
            &[],
        )
        .unwrap();

    let portal = session
        .queue_bind::<&[u8]>(
            &statement,
            frontend::PortalName::Unnamed,
            frontend::Formats::Default,
            &[],
            frontend::Formats::Default,
        )
        .unwrap();

    session.queue_execute(&portal, None).unwrap();
    session.queue_sync().unwrap();

    while !matches!(
        backend::Message::decode(&session.receive().await.unwrap()).unwrap(),
        backend::Message::ReadyForQuery(_)
    ) {}
}

#[tokio::test]
async fn copies_rows_in() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "copyin")
        .with_session(async |session| {
            run(session, "CREATE TABLE t (x int)").await;

            let statement = session
                .queue_parse(
                    frontend::StatementName::Unnamed,
                    &frontend::Sql::from_static_or_panic("COPY t FROM STDIN"),
                    &[],
                )
                .unwrap();

            let portal = session
                .queue_bind::<&[u8]>(
                    &statement,
                    frontend::PortalName::Unnamed,
                    frontend::Formats::Default,
                    &[],
                    frontend::Formats::Default,
                )
                .unwrap();

            session.queue_execute(&portal, None).unwrap();
            session.queue_flush().unwrap();

            while session.mode() == session::Mode::CommandProcessing {
                session.receive().await.unwrap();
            }

            assert_eq!(session.mode(), session::Mode::CopyIn);

            session.send_copy_data(b"1\n2\n3\n").await.unwrap();
            session.queue_copy_done().unwrap();
            session.queue_sync().unwrap();

            let mut copied = None;

            loop {
                match backend::Message::decode(&session.receive().await.unwrap()).unwrap() {
                    backend::Message::CommandComplete(backend::CommandComplete::Copy { rows }) => {
                        copied = Some(rows);
                    }
                    backend::Message::ReadyForQuery(_) => break,
                    _ => (),
                }
            }

            assert_eq!(copied, Some(3));
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn closes_a_statement_the_server_then_cannot_find() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "close")
        .with_session(async |session| {
            let statement = session
                .queue_parse(
                    frontend::StatementName::Named(
                        pg_client::name::Statement::from_static_or_panic("doomed"),
                    ),
                    &frontend::Sql::from_static_or_panic("SELECT 1"),
                    &[],
                )
                .unwrap();

            let stale = statement.clone();

            session.queue_close_statement(statement).unwrap();
            session.queue_sync().unwrap();

            while !matches!(
                backend::Message::decode(&session.receive().await.unwrap()).unwrap(),
                backend::Message::ReadyForQuery(_)
            ) {}

            session
                .queue_bind::<&[u8]>(
                    &stale,
                    frontend::PortalName::Unnamed,
                    frontend::Formats::Default,
                    &[],
                    frontend::Formats::Default,
                )
                .unwrap();

            session.queue_sync().unwrap();

            let mut refused = None;

            loop {
                match backend::Message::decode(&session.receive().await.unwrap()).unwrap() {
                    backend::Message::ErrorResponse(error) => {
                        refused = Some(error.diagnostic.code());
                    }
                    backend::Message::ReadyForQuery(_) => break,
                    _ => (),
                }
            }

            assert_eq!(refused, Some(backend::SqlState::InvalidSqlStatementName));
        })
        .await
        .unwrap()
}

async fn backend_count(session: &mut session::Session, pid: i32) -> u64 {
    let statement = session
        .queue_parse(
            frontend::StatementName::Unnamed,
            &frontend::Sql::from_static_or_panic(
                "SELECT count(*) FROM pg_stat_activity WHERE pid = $1::int4",
            ),
            &[],
        )
        .unwrap();

    let pid = pid.to_string();

    let portal = session
        .queue_bind(
            &statement,
            frontend::PortalName::Unnamed,
            frontend::Formats::Default,
            &[Some(pid.as_bytes())],
            frontend::Formats::Default,
        )
        .unwrap();

    session.queue_execute(&portal, None).unwrap();
    session.queue_sync().unwrap();

    let mut count = None;

    loop {
        match backend::Message::decode(&session.receive().await.unwrap()).unwrap() {
            backend::Message::DataRow(row) => {
                let values = row.into_iter().collect::<Vec<_>>();

                count = Some(
                    std::str::from_utf8(values[0].unwrap())
                        .unwrap()
                        .parse::<u64>()
                        .unwrap(),
                );
            }
            backend::Message::ReadyForQuery(_) => break count.unwrap(),
            _ => (),
        }
    }
}

#[tokio::test]
async fn refuses_a_message_the_mode_does_not_allow() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "mode")
        .with_session(async |session| {
            assert_eq!(
                session.queue_copy_done().unwrap_err(),
                session::ModeError {
                    mode: session::Mode::CommandProcessing
                }
            );

            run(session, "SELECT 1").await;

            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn terminate_ends_the_backend() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "terminate")
        .with_container(async |container| {
            let config = container.session_config();

            let doomed = session::start(std::sync::Arc::clone(&config))
                .await
                .unwrap();
            let pid = doomed.backend_key().process_id.get();

            let mut observer = session::start(config).await.unwrap();
            let before = backend_count(&mut observer, pid).await;

            assert_eq!(before, 1);

            doomed.send_terminate().await.unwrap();

            let mut remaining = before;

            for _ in 0..200 {
                remaining = backend_count(&mut observer, pid).await;

                if remaining == 0 {
                    break;
                }
            }

            assert_eq!(remaining, 0);
        })
        .await
        .unwrap()
}

#[test]
fn refuses_to_take_the_encoding_it_forces() {
    for name in ["client_encoding", "Client_Encoding"] {
        let name: pg_client::parameter::Name = name.parse().unwrap();

        assert_eq!(
            session::Parameters::new().insert(
                name.clone(),
                pg_client::parameter::Value::from_static_or_panic("LATIN1"),
            ),
            Err(session::ParametersError::Forced { name }),
        );
    }
}

#[test]
fn refuses_application_name_outside_its_setter() {
    for name in ["application_name", "APPLICATION_NAME"] {
        let name: pg_client::parameter::Name = name.parse().unwrap();

        assert_eq!(
            session::Parameters::new().insert(
                name.clone(),
                pg_client::parameter::Value::from_static_or_panic("app"),
            ),
            Err(session::ParametersError::Dedicated { name }),
        );
    }
}

#[test]
fn takes_other_settings() {
    let name = pg_client::parameter::Name::from_static_or_panic("search_path");
    let value = pg_client::parameter::Value::from_static_or_panic("app");
    let mut parameters = session::Parameters::new();

    assert_eq!(parameters.insert(name.clone(), value.clone()), Ok(None));
    assert_eq!(parameters.get(&name), Some(&value));
}

#[test]
fn sets_the_application_name() {
    let mut parameters = session::Parameters::new();

    parameters.set_application_name("some-app".parse().unwrap());

    assert_eq!(
        parameters.iter().collect::<Vec<_>>(),
        vec![(
            &pg_client::parameter::Name::from_static_or_panic("application_name"),
            &pg_client::parameter::Value::from_static_or_panic("some-app"),
        )],
    );
}
