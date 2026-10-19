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
                    &pg_client::Sql::from_static_or_panic("SELECT $1::int4 + 1"),
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
                    &pg_client::Sql::from_static_or_panic("SELECT 1"),
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
                    &pg_client::Sql::from_static_or_panic("SELECT $1"),
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
                .queue_query(&pg_client::Sql::from_static_or_panic("SELECT 1; SELECT 2"))
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
async fn round_trips_a_uuid() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "uuid")
        .with_session(async |session| {
            let uuid = uuid::Uuid::from_u128(0xa0ee_bc99_9c0b_4ef8_bb6d_6bb9_bd38_0a11);

            let text = session
                .statement(&pg_client::Sql::from_static_or_panic("SELECT $1::text"))
                .bind(uuid)
                .execute()
                .unwrap()
                .scalar::<String>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(text, "a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11");

            let decoded = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'::uuid",
                ))
                .execute()
                .unwrap()
                .scalar::<uuid::Uuid>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(decoded, uuid);
        })
        .await
        .unwrap()
}

type Shared = std::sync::Arc<tokio::sync::Mutex<session::Session>>;

async fn count_from<S: session::Source>(source: &S, count: i32) -> i64
where
    S::Error: core::fmt::Debug,
{
    session::execute::Builder::new(
        source.acquire().await.unwrap(),
        &pg_client::Sql::from_static_or_panic("SELECT count(*) FROM generate_series(1, $1)"),
    )
    .bind(count)
    .execute()
    .unwrap()
    .scalar::<i64>()
    .await
    .unwrap()
    .one()
    .await
    .unwrap()
}

#[tokio::test]
async fn runs_statements_against_any_session_source() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "source")
        .with_container(async |container| {
            let factory = container.session_config();

            let shared: Shared = std::sync::Arc::new(tokio::sync::Mutex::new(
                session::start(std::sync::Arc::clone(&factory))
                    .await
                    .unwrap(),
            ));

            assert_eq!(count_from(&factory, 3).await, 3);
            assert_eq!(count_from(&shared, 5).await, 5);
            assert_eq!(count_from(&shared, 7).await, 7);
        })
        .await
        .unwrap()
}

async fn numbers(
    shared: &Shared,
    count: i32,
) -> session::execute::Reader<tokio::sync::OwnedMutexGuard<session::Session>> {
    session::execute::Builder::new(
        std::sync::Arc::clone(shared).lock_owned().await,
        &pg_client::Sql::from_static_or_panic("SELECT generate_series(1, $1)"),
    )
    .bind(count)
    .execute()
    .unwrap()
}

#[tokio::test]
async fn shares_one_session_between_tasks() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "shared")
        .with_container(async |container| {
            let shared: Shared = std::sync::Arc::new(tokio::sync::Mutex::new(
                session::start(container.session_config()).await.unwrap(),
            ));

            let mut abandoned = numbers(&shared, 1000).await;
            let description = abandoned.describe().await.unwrap();
            let column = description.column::<i32>(0).unwrap();
            let first = abandoned
                .next()
                .await
                .unwrap()
                .unwrap()
                .decode(&column)
                .unwrap();
            drop(abandoned);

            assert_eq!(first, 1);

            let tasks: Vec<_> = (1..=8)
                .map(|count| {
                    let shared = std::sync::Arc::clone(&shared);

                    tokio::spawn(async move {
                        numbers(&shared, count)
                            .await
                            .scalar::<i32>()
                            .await
                            .unwrap()
                            .into_vec()
                            .await
                            .unwrap()
                    })
                })
                .collect();

            for (count, task) in (1..=8).zip(tasks) {
                assert_eq!(task.await.unwrap(), (1..=count).collect::<Vec<_>>());
            }

            assert_eq!(
                shared.lock().await.transaction_status(),
                backend::TransactionStatus::Idle
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn decodes_a_scalar_only_from_exactly_one_column() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "scalar")
        .with_session(async |session| {
            let numbers = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT generate_series(1, $1)",
                ))
                .bind(3i32)
                .execute()
                .unwrap()
                .scalar::<i32>()
                .await
                .unwrap()
                .into_vec()
                .await
                .unwrap();

            assert_eq!(numbers, [1, 2, 3]);

            for (sql, columns) in [("SELECT 1, 2", 2), ("CREATE TEMPORARY TABLE t (id int)", 0)] {
                let error = session
                    .statement(&pg_client::Sql::from_static_or_panic(sql))
                    .execute()
                    .unwrap()
                    .scalar::<i32>()
                    .await
                    .err()
                    .unwrap();

                let session::execute::Error::NotOneColumn { columns: actual } = error else {
                    panic!("expected {sql} to be refused as a scalar, got {error:?}");
                };

                assert_eq!(actual, columns, "{sql}");
            }

            let one = session
                .statement(&pg_client::Sql::from_static_or_panic("SELECT 1"))
                .execute()
                .unwrap()
                .scalar::<i32>()
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
async fn decodes_rows_into_tuples_up_to_eight_columns() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "from-row-tuple")
        .with_session(async |session| {
            let rows = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT n, n::int8, n::int2, n % 2 = 0, n::float8, n::text, NULL::int4, \
                     n::text::bytea FROM generate_series(1, 2) AS n",
                ))
                .execute()
                .unwrap()
                .rows::<(i32, i64, i16, bool, f64, String, Option<i32>, Vec<u8>)>()
                .await
                .unwrap()
                .into_vec()
                .await
                .unwrap();

            assert_eq!(
                rows,
                [
                    (1, 1, 1, false, 1.0, "1".to_owned(), None, b"1".to_vec()),
                    (2, 2, 2, true, 2.0, "2".to_owned(), None, b"2".to_vec()),
                ]
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn decodes_borrowed_rows_through_columns_resolved_once() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "from-row-borrowed")
        .with_session(async |session| {
            let mut reader = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT n, repeat('x', n) FROM generate_series(1, 3) AS n",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let columns = description.columns::<(i32, &str)>().unwrap();

            let mut rows = Vec::new();

            while let Some(row) = reader.next().await.unwrap() {
                let (n, text): (i32, &str) = row.decode_row(&columns).unwrap();

                rows.push((n, text.to_owned()));
            }

            assert_eq!(
                rows,
                [
                    (1, "x".to_owned()),
                    (2, "xx".to_owned()),
                    (3, "xxx".to_owned())
                ]
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_a_row_type_with_another_column_count() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "from-row-count")
        .with_session(async |session| {
            let error = session
                .statement(&pg_client::Sql::from_static_or_panic("SELECT 1, 2, 3"))
                .execute()
                .unwrap()
                .rows::<(i32, i32)>()
                .await
                .err()
                .unwrap();

            let session::execute::Error::Column(session::execute::ColumnError::Count {
                expected: 2,
                actual: 3,
            }) = error
            else {
                panic!("expected the column count to be refused, got {error:?}");
            };
        })
        .await
        .unwrap()
}

#[derive(Debug, PartialEq)]
struct Account {
    name: String,
    balance: i64,
}

impl session::execute::FromRow for Account {
    type Columns = (
        session::execute::Column<String>,
        session::execute::Column<i64>,
    );
    type Output<'r> = Self;

    fn columns(
        description: &session::execute::Description,
    ) -> Result<Self::Columns, session::execute::ColumnError> {
        Ok((
            description.column(pg_client::name::Column::from_static_or_panic("name"))?,
            description.column(pg_client::name::Column::from_static_or_panic("balance"))?,
        ))
    }

    fn from_row<'r>(
        (name, balance): &Self::Columns,
        row: &session::execute::Row<'r>,
    ) -> Result<Self, session::execute::DecodeError> {
        Ok(Self {
            name: row.decode(name)?,
            balance: row.decode(balance)?,
        })
    }
}

struct AccountByPosition;

impl session::execute::FromRow for AccountByPosition {
    type Columns = <(String, i64) as session::execute::FromRow>::Columns;
    type Output<'r> = Account;

    fn columns(
        description: &session::execute::Description,
    ) -> Result<Self::Columns, session::execute::ColumnError> {
        <(String, i64)>::columns(description)
    }

    fn from_row<'r>(
        columns: &Self::Columns,
        row: &session::execute::Row<'r>,
    ) -> Result<Account, session::execute::DecodeError> {
        let (name, balance) = <(String, i64)>::from_row(columns, row)?;

        Ok(Account { name, balance })
    }
}

#[tokio::test]
async fn decodes_rows_through_a_marker_into_another_type() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "from-row-marker")
        .with_session(async |session| {
            let account = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT 'alice', 10::int8",
                ))
                .execute()
                .unwrap()
                .rows_as::<AccountByPosition, _>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(
                account,
                Account {
                    name: "alice".to_owned(),
                    balance: 10
                }
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn decodes_rows_into_a_type_resolving_columns_by_name() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "from-row-struct")
        .with_session(async |session| {
            let accounts = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT 10::int8 AS balance, 'alice' AS name UNION ALL SELECT 20, 'bob'",
                ))
                .execute()
                .unwrap()
                .rows::<Account>()
                .await
                .unwrap()
                .into_vec()
                .await
                .unwrap();

            assert_eq!(
                accounts,
                [
                    Account {
                        name: "alice".to_owned(),
                        balance: 10
                    },
                    Account {
                        name: "bob".to_owned(),
                        balance: 20
                    },
                ]
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn decodes_arrays() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "decode-array")
        .with_session(async |session| {
            let mut reader = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT ARRAY[1, NULL, 3]::int4[], '[0:1][1:2]={{1,2},{3,4}}'::int4[], \
                     ARRAY['a', 'bc']::text[], '{}'::int4[]",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let flat = description
                .column::<pg_client::value::Array<i32>>(0)
                .unwrap();
            let grid = description
                .column::<pg_client::value::Array<i32>>(1)
                .unwrap();
            let words = description
                .column::<pg_client::value::Array<&str>>(2)
                .unwrap();
            let empty = description
                .column::<pg_client::value::Array<i32>>(3)
                .unwrap();

            let row = reader.next().await.unwrap().unwrap();

            let flat = row.decode(&flat).unwrap();
            assert_eq!(flat.elements(), [Some(1), None, Some(3)]);

            let grid = row.decode(&grid).unwrap();
            assert_eq!(
                grid.dimensions(),
                [
                    pg_client::value::Dimension {
                        length: 2,
                        lower_bound: 0
                    },
                    pg_client::value::Dimension {
                        length: 2,
                        lower_bound: 1
                    },
                ]
            );
            assert_eq!(grid.get(&[1, 1]), Some(Some(&3)));

            let words = row.decode(&words).unwrap();
            assert_eq!(words.elements(), [Some("a"), Some("bc")]);

            let empty = row.decode(&empty).unwrap();
            assert_eq!(empty.dimensions(), []);
            assert_eq!(empty.elements(), []);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_to_decode_an_array_of_another_element_type() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "decode-array-type")
        .with_session(async |session| {
            let mut reader = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT ARRAY[1]::int4[], 1::int4",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();

            for index in [0, 1] {
                let error = description
                    .column::<pg_client::value::Array<String>>(index)
                    .unwrap_err();

                assert!(
                    matches!(error, session::execute::ColumnError::Type { index: actual, .. } if actual == index),
                    "{error:?}"
                );
            }
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
                .statement(&pg_client::Sql::from_static_or_panic(
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
async fn round_trips_a_timestamptz() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "timestamptz")
        .with_session(async |session| {
            let timestamp = pg_client::value::TimestampTz::Finite(
                pg_client::value::FiniteTimestampTz::truncate_precision(
                    "2024-02-29T12:34:56.123456Z".parse().unwrap(),
                )
                .unwrap(),
            );

            let text = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT ($1 AT TIME ZONE 'UTC')::text",
                ))
                .bind(timestamp)
                .execute()
                .unwrap()
                .scalar::<String>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(text, "2024-02-29 12:34:56.123456");

            let decoded = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT '2024-02-29 12:34:56.123456+00'::timestamptz",
                ))
                .execute()
                .unwrap()
                .scalar::<pg_client::value::TimestampTz>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(decoded, timestamp);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn round_trips_a_timestamp() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "timestamp")
        .with_session(async |session| {
            let timestamp = pg_client::value::Timestamp::Finite(
                pg_client::value::FiniteTimestamp::truncate_precision(
                    "2024-02-29T12:34:56.123456".parse().unwrap(),
                )
                .unwrap(),
            );

            let text = session
                .statement(&pg_client::Sql::from_static_or_panic("SELECT $1::text"))
                .bind(timestamp)
                .execute()
                .unwrap()
                .scalar::<String>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(text, "2024-02-29 12:34:56.123456");

            let decoded = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT '2024-02-29 12:34:56.123456'::timestamp",
                ))
                .execute()
                .unwrap()
                .scalar::<pg_client::value::Timestamp>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(decoded, timestamp);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn round_trips_the_timestamp_infinities() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "timestamp-infinity")
        .with_session(async |session| {
            for (timestamp, expected) in [
                (pg_client::value::TimestampTz::NegativeInfinity, "-infinity"),
                (pg_client::value::TimestampTz::Infinity, "infinity"),
            ] {
                let text = session
                    .statement(&pg_client::Sql::from_static_or_panic("SELECT $1::text"))
                    .bind(timestamp)
                    .execute()
                    .unwrap()
                    .scalar::<String>()
                    .await
                    .unwrap()
                    .one()
                    .await
                    .unwrap();

                assert_eq!(text, expected);
            }

            let decoded = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT '-infinity'::timestamp",
                ))
                .execute()
                .unwrap()
                .scalar::<pg_client::value::Timestamp>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(decoded, pg_client::value::Timestamp::NegativeInfinity);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn decodes_the_earliest_timestamp_postgresql_stores() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "timestamp-earliest")
        .with_session(async |session| {
            let decoded = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT '4714-11-24 00:00:00 BC'::timestamp",
                ))
                .execute()
                .unwrap()
                .scalar::<pg_client::value::Timestamp>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            let pg_client::value::Timestamp::Finite(finite) = decoded else {
                panic!("expected a finite timestamp, got {decoded:?}");
            };

            assert_eq!(
                finite.get(),
                chrono::NaiveDate::from_ymd_opt(-4713, 11, 24)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_a_timestamp_chrono_cannot_represent() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "timestamp-unrepresentable")
        .with_session(async |session| {
            let mut reader = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT '262144-01-01 00:00:00'::timestamp",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let column = description
                .column::<pg_client::value::Timestamp>(0)
                .unwrap();

            let row = reader.next().await.unwrap().unwrap();

            let error = row.decode(&column).unwrap_err();

            let session::execute::DecodeError::Value {
                index: 0,
                source: pg_client::value::DecodeError::TimestampUnrepresentable { .. },
            } = error
            else {
                panic!("expected an unrepresentable timestamp on column 0, got {error:?}");
            };
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn decodes_a_timestamptz_into_a_date_time() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "timestamptz-date-time")
        .with_session(async |session| {
            let decoded = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT '2024-02-29 12:34:56.123456+00'::timestamptz",
                ))
                .execute()
                .unwrap()
                .scalar::<chrono::DateTime<chrono::Utc>>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(
                decoded,
                "2024-02-29T12:34:56.123456Z"
                    .parse::<chrono::DateTime<chrono::Utc>>()
                    .unwrap()
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn binds_timestamp_array_values() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "timestamp-array")
        .with_session(async |session| {
            let finite = pg_client::value::FiniteTimestamp::truncate_precision(
                "2024-02-29T12:34:56.123456".parse().unwrap(),
            )
            .unwrap();

            let timestamps = pg_client::value::Value::TimestampArray(
                vec![
                    Some(pg_client::value::Timestamp::NegativeInfinity),
                    Some(pg_client::value::Timestamp::Finite(finite)),
                    None,
                ]
                .into(),
            );

            let text = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT pg_typeof($1)::text || ' ' || $1::text",
                ))
                .bind_value(&timestamps)
                .unwrap()
                .execute()
                .unwrap()
                .scalar::<String>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(
                text,
                "timestamp without time zone[] {-infinity,\"2024-02-29 12:34:56.123456\",NULL}"
            );

            let timestamps_tz = pg_client::value::Value::TimestampTzArray(
                vec![pg_client::value::TimestampTz::Infinity].into(),
            );

            let text = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT pg_typeof($1)::text || ' ' || $1::text",
                ))
                .bind_value(&timestamps_tz)
                .unwrap()
                .execute()
                .unwrap()
                .scalar::<String>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(text, "timestamp with time zone[] {infinity}");
        })
        .await
        .unwrap()
}

async fn echo<T>(session: &mut session::Session, value: T) -> T
where
    T: pg_client::value::Encode + for<'r> pg_client::value::Decode<Output<'r> = T>,
{
    let builder = session
        .statement(&pg_client::Sql::from_static_or_panic("SELECT $1"))
        .bind(value);

    <T::Outcome as pg_client::value::Outcome>::into_checked_result(builder)
        .unwrap()
        .execute()
        .unwrap()
        .scalar::<T>()
        .await
        .unwrap()
        .one()
        .await
        .unwrap()
}

#[tokio::test]
async fn round_trips_the_scalar_types() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "scalars")
        .with_session(async |session| {
            assert_eq!(echo(session, -7i8).await, -7);
            assert_eq!(echo(session, i16::MIN).await, i16::MIN);
            assert_eq!(echo(session, 1.5f32).await, 1.5);
            assert_eq!(echo(session, -0.25f64).await, -0.25);
            assert_eq!(
                echo(session, pg_client::protocol::Oid::new(u32::MAX)).await,
                pg_client::protocol::Oid::new(u32::MAX)
            );
            assert_eq!(echo(session, vec![0u8, 0xff, 0x00]).await, [0, 0xff, 0]);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn round_trips_a_date() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "date")
        .with_session(async |session| {
            let date = pg_client::value::Date::Finite(
                pg_client::value::FiniteDate::new("2024-02-29".parse().unwrap()).unwrap(),
            );

            let text = session
                .statement(&pg_client::Sql::from_static_or_panic("SELECT $1::text"))
                .bind(date)
                .execute()
                .unwrap()
                .scalar::<String>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(text, "2024-02-29");

            assert_eq!(echo(session, date).await, date);
            assert_eq!(
                echo(session, pg_client::value::Date::Infinity).await,
                pg_client::value::Date::Infinity
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn decodes_the_earliest_date_postgresql_stores() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "date-earliest")
        .with_session(async |session| {
            let decoded = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT '4714-11-24 BC'::date",
                ))
                .execute()
                .unwrap()
                .scalar::<chrono::NaiveDate>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(
                decoded,
                chrono::NaiveDate::from_ymd_opt(-4713, 11, 24).unwrap()
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_a_date_chrono_cannot_represent() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "date-unrepresentable")
        .with_session(async |session| {
            let mut reader = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT '262144-01-01'::date",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let column = description.column::<pg_client::value::Date>(0).unwrap();

            let row = reader.next().await.unwrap().unwrap();

            let error = row.decode(&column).unwrap_err();

            let session::execute::DecodeError::Value {
                index: 0,
                source: pg_client::value::DecodeError::DateUnrepresentable { .. },
            } = error
            else {
                panic!("expected an unrepresentable date on column 0, got {error:?}");
            };
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn binds_a_date_array_value() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "date-array")
        .with_session(async |session| {
            let dates = pg_client::value::Value::DateArray(
                vec![
                    Some(pg_client::value::Date::Finite(
                        pg_client::value::FiniteDate::new("2024-02-29".parse().unwrap()).unwrap(),
                    )),
                    Some(pg_client::value::Date::NegativeInfinity),
                    None,
                ]
                .into(),
            );

            let text = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT pg_typeof($1)::text || ' ' || $1::text",
                ))
                .bind_value(&dates)
                .unwrap()
                .execute()
                .unwrap()
                .scalar::<String>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(text, "date[] {2024-02-29,-infinity,NULL}");
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn round_trips_numerics() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "numeric")
        .with_session(async |session| {
            for (input, expected) in [
                ("0", "0"),
                ("0.00", "0.00"),
                ("1.5", "1.5"),
                ("1.50", "1.50"),
                ("-123.456", "-123.456"),
                ("10000", "10000"),
                ("0.0001", "0.0001"),
                ("-0.00001", "-0.00001"),
                ("1E+3", "1000"),
                (
                    "12345678901234567890.123456789",
                    "12345678901234567890.123456789",
                ),
            ] {
                let numeric = pg_client::value::Numeric::Finite(
                    pg_client::value::FiniteNumeric::new(input.parse().unwrap()).unwrap(),
                );

                let text = session
                    .statement(&pg_client::Sql::from_static_or_panic("SELECT $1::text"))
                    .bind(&numeric)
                    .execute()
                    .unwrap()
                    .scalar::<String>()
                    .await
                    .unwrap()
                    .one()
                    .await
                    .unwrap();

                assert_eq!(text, expected, "{input}");

                let pg_client::value::Numeric::Finite(decoded) = echo(session, numeric).await
                else {
                    panic!("expected a finite numeric for {input}");
                };

                assert_eq!(decoded.get().to_plain_string(), expected, "{input}");
            }
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn decodes_numerics_the_server_writes() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "numeric-decode")
        .with_session(async |session| {
            for literal in [
                "-0.000000000000000000000000000001",
                "100000000000000000000000000000000000000000000000000.5",
                "99999999.99990000",
            ] {
                let decoded = session
                    .statement(&pg_client::Sql::from_static_or_panic("SELECT $1::numeric"))
                    .bind(literal)
                    .unwrap()
                    .execute()
                    .unwrap()
                    .scalar::<bigdecimal::BigDecimal>()
                    .await
                    .unwrap()
                    .one()
                    .await
                    .unwrap();

                assert_eq!(decoded.to_plain_string(), literal);
            }

            for (literal, expected) in [
                ("NaN", pg_client::value::Numeric::NaN),
                ("Infinity", pg_client::value::Numeric::Infinity),
                ("-Infinity", pg_client::value::Numeric::NegativeInfinity),
            ] {
                let decoded = session
                    .statement(&pg_client::Sql::from_static_or_panic("SELECT $1::numeric"))
                    .bind(literal)
                    .unwrap()
                    .execute()
                    .unwrap()
                    .scalar::<pg_client::value::Numeric>()
                    .await
                    .unwrap()
                    .one()
                    .await
                    .unwrap();

                assert_eq!(decoded, expected);
                assert_eq!(echo(session, expected.clone()).await, expected);
            }
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn binds_a_numeric_array_value() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "numeric-array")
        .with_session(async |session| {
            let numerics = pg_client::value::Value::NumericArray(
                vec![
                    Some(pg_client::value::Numeric::Finite(
                        pg_client::value::FiniteNumeric::new("1.50".parse().unwrap()).unwrap(),
                    )),
                    Some(pg_client::value::Numeric::NaN),
                    None,
                ]
                .into(),
            );

            let text = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT pg_typeof($1)::text || ' ' || $1::text",
                ))
                .bind_value(&numerics)
                .unwrap()
                .execute()
                .unwrap()
                .scalar::<String>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(text, "numeric[] {1.50,NaN,NULL}");
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn decodes_borrowed_text_and_bytes_from_each_row() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "borrowed")
        .with_session(async |session| {
            let mut reader = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT * FROM (VALUES ('a', '\\x00ff'::bytea, NULL), ('b', '\\x'::bytea, 'c')) AS rows (word, bytes, nullable)",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let word = description.column::<&str>(0).unwrap();
            let bytes = description.column::<&[u8]>(1).unwrap();
            let nullable = description.column::<Option<&str>>(2).unwrap();

            let mut rows = Vec::new();

            while let Some(row) = reader.next().await.unwrap() {
                let word: &str = row.decode(&word).unwrap();
                let bytes: &[u8] = row.decode(&bytes).unwrap();
                let nullable: Option<&str> = row.decode(&nullable).unwrap();

                rows.push((word.to_owned(), bytes.to_vec(), nullable.map(str::to_owned)));
            }

            assert_eq!(
                rows,
                [
                    ("a".to_owned(), vec![0x00, 0xff], None),
                    ("b".to_owned(), Vec::new(), Some("c".to_owned())),
                ]
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn binds_a_uuid_array_value() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "uuid-array")
        .with_session(async |session| {
            let uuids = pg_client::value::Value::UuidArray(
                vec![
                    Some(uuid::Uuid::from_u128(
                        0xa0ee_bc99_9c0b_4ef8_bb6d_6bb9_bd38_0a11,
                    )),
                    None,
                ]
                .into(),
            );

            let text = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT pg_typeof($1)::text || ' ' || $1::text",
                ))
                .bind_value(&uuids)
                .unwrap()
                .execute()
                .unwrap()
                .scalar::<String>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(text, "uuid[] {a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11,NULL}");
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
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT array_length($1, 1), array_position($1, 'c')",
                ))
                .bind(&names)
                .unwrap()
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
                .statement(&pg_client::Sql::from_static_or_panic(
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
                .statement(&pg_client::Sql::from_static_or_panic(
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
                .statement(&pg_client::Sql::from_static_or_panic("SELECT ''::name"))
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
                .statement(&pg_client::Sql::from_static_or_panic(
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
async fn binds_json_and_jsonb() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "encode-json")
        .with_session(async |session| {
            let text = |sql| pg_client::Sql::from_static_or_panic(sql);

            let json = session
                .statement(&text("SELECT pg_typeof($1)::text || ' ' || $1::text"))
                .bind(pg_client::value::Json(serde_json::json!({"a": [1, 2]})))
                .unwrap()
                .execute()
                .unwrap()
                .scalar::<String>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(json, "json {\"a\":[1,2]}");

            let jsonb = session
                .statement(&text("SELECT pg_typeof($1)::text || ' ' || $1::text"))
                .bind(pg_client::value::Jsonb(
                    serde_json::json!({"b": true, "a": 1}),
                ))
                .unwrap()
                .execute()
                .unwrap()
                .scalar::<String>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(jsonb, "jsonb {\"a\": 1, \"b\": true}");

            let raw = session
                .statement(&text("SELECT $1::text"))
                .bind(pg_client::value::JsonText::new("[1, 2, 3]"))
                .unwrap()
                .execute()
                .unwrap()
                .scalar::<String>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(raw, "[1, 2, 3]");

            let raw = session
                .statement(&text("SELECT $1::text"))
                .bind(pg_client::value::JsonbText::new("[1, 2, 3]"))
                .unwrap()
                .execute()
                .unwrap()
                .scalar::<String>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(raw, "[1, 2, 3]");
        })
        .await
        .unwrap()
}

#[cfg(feature = "serde")]
#[tokio::test]
async fn deserializes_json_borrowed_from_the_row() {
    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct Greeting<'a> {
        word: &'a str,
    }

    let backend = ociman::test_backend_setup!();

    definition(backend, "decode-json-borrowed")
        .with_session(async |session| {
            let mut reader = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT '{\"word\":\"hi\"}'::json, '{\"word\":\"hi\"}'::jsonb",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let json = description.column::<pg_client::value::JsonText>(0).unwrap();
            let jsonb = description
                .column::<pg_client::value::JsonbText>(1)
                .unwrap();

            let row = reader.next().await.unwrap().unwrap();

            let json = row.decode(&json).unwrap();
            let jsonb = row.decode(&jsonb).unwrap();

            assert_eq!(json.as_str(), "{\"word\":\"hi\"}");
            assert_eq!(jsonb.as_str(), "{\"word\": \"hi\"}");

            assert_eq!(
                serde_json::from_str::<Greeting<'_>>(json.as_str()).unwrap(),
                Greeting { word: "hi" }
            );
            assert_eq!(
                serde_json::from_str::<Greeting<'_>>(jsonb.as_str()).unwrap(),
                Greeting { word: "hi" }
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn runs_sql_borrowed_from_another_sessions_row() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "decode-borrowed-sql")
        .with_container(async |container| {
            let config = container.session_config();

            let mut generator = session::start(std::sync::Arc::clone(&config))
                .await
                .unwrap();
            let mut runner = session::start(config).await.unwrap();

            let mut reader = generator
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT 'SELECT 41 + 1'::text",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let column = description.column::<&pg_client::Sql>(0).unwrap();

            let row = reader.next().await.unwrap().unwrap();
            let sql: pg_client::Sql<'_> = row.decode(&column).unwrap();

            assert_eq!(sql.as_str(), "SELECT 41 + 1");

            let answer = runner
                .statement(&sql)
                .execute()
                .unwrap()
                .scalar::<i32>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(answer, 42);
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn decodes_a_borrowed_name() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "decode-borrowed-name")
        .with_session(async |session| {
            let mut reader = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT 'example'::name",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let table = description.column::<&pg_client::name::Table>(0).unwrap();

            let row = reader.next().await.unwrap().unwrap();

            let table: pg_client::name::Table<'_> = row.decode(&table).unwrap();

            assert_eq!(table.as_str(), "example");
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
                .statement(&pg_client::Sql::from_static_or_panic("SELECT '[1]'::jsonb"))
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
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT format('SELECT count(*) FROM %I', 'pg_namespace')",
                ))
                .execute()
                .unwrap();

            let description = reader.describe().await.unwrap();
            let column = description.column::<pg_client::Sql>(0).unwrap();

            let generated: pg_client::Sql = reader
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
                .statement(&pg_client::Sql::from_static_or_panic(
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
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT 1 WHERE false",
                ))
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
                .statement(&pg_client::Sql::from_static_or_panic(
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
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT 1 WHERE false",
                ))
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
                .statement(&pg_client::Sql::from_static_or_panic("SELECT 42::int4"))
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
                .statement(&pg_client::Sql::from_static_or_panic(
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
                .statement(&pg_client::Sql::from_static_or_panic(
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
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT 1 / (n - 2) FROM generate_series(1, 3) AS n",
                ))
                .execute()
                .unwrap()
                .discard_rows()
                .await
                .unwrap_err();

            let session::execute::Error::Server(server) = error else {
                panic!("expected a server error, got {error:?}");
            };

            assert_eq!(
                server.diagnostic().code(),
                backend::SqlState::DivisionByZero
            );
            assert_eq!(
                session.transaction_status(),
                backend::TransactionStatus::Idle
            );

            session
                .statement(&pg_client::Sql::from_static_or_panic("SELECT 1"))
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
                .statements(&pg_client::Sql::from_static_or_panic(
                    "CREATE TABLE t (x int); INSERT INTO t VALUES (1), (2); SELECT * FROM t",
                ))
                .await
                .unwrap();

            let mut reader = session
                .statement(&pg_client::Sql::from_static_or_panic(
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
                .statements(&pg_client::Sql::from_static_or_panic(
                    "CREATE TABLE t (x int); SELECT 1 / 0; INSERT INTO t VALUES (1)",
                ))
                .await
                .unwrap_err();

            let session::execute::Error::Server(server) = error else {
                panic!("expected a server error, got {error:?}");
            };

            assert_eq!(
                server.diagnostic().code(),
                backend::SqlState::DivisionByZero
            );

            let mut reader = session
                .statement(&pg_client::Sql::from_static_or_panic(
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
                .statements(&pg_client::Sql::from_static_or_panic(
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
                .statements(&pg_client::Sql::from_static_or_panic("SELECT 1"))
                .await
                .unwrap();
        })
        .await
        .unwrap()
}

async fn count_rows(session: &mut session::Session) -> i64 {
    let mut reader = session
        .statement(&pg_client::Sql::from_static_or_panic(
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
                .statements(&pg_client::Sql::from_static_or_panic(
                    "CREATE TABLE t (x int)",
                ))
                .await
                .unwrap();

            let mut copy = session
                .copy_in(&pg_client::Sql::from_static_or_panic(
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
                .statements(&pg_client::Sql::from_static_or_panic(
                    "CREATE TABLE t (x int)",
                ))
                .await
                .unwrap();

            let mut copy = session
                .copy_in(&pg_client::Sql::from_static_or_panic(
                    "COPY t FROM STDIN (FORMAT csv)",
                ))
                .await
                .unwrap();

            copy.write(b"1\nnot a number\n3\n").await.unwrap();

            let error = copy.finish().await.unwrap_err();

            let session::execute::Error::Server(server) = error else {
                panic!("expected a server error, got {error:?}");
            };

            assert_eq!(
                server.diagnostic().code(),
                backend::SqlState::InvalidTextRepresentation
            );
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
                .statements(&pg_client::Sql::from_static_or_panic(
                    "CREATE TABLE t (x int)",
                ))
                .await
                .unwrap();

            let mut copy = session
                .copy_in(&pg_client::Sql::from_static_or_panic(
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
                .statements(&pg_client::Sql::from_static_or_panic(
                    "CREATE TABLE t (x int)",
                ))
                .await
                .unwrap();

            let mut copy = session
                .copy_in(&pg_client::Sql::from_static_or_panic(
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
                .copy_in(&pg_client::Sql::from_static_or_panic("SELECT 1"))
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
                .statements(&pg_client::Sql::from_static_or_panic("SELECT 1"))
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
                .statement(&pg_client::Sql::from_static_or_panic(
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
                .statement(&pg_client::Sql::from_static_or_panic("SELECT 'text'::text"))
                .execute()
                .unwrap()
                .scalar::<i32>()
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
            let pid = victim.backend_key().process_id().get();

            let sleeping = tokio::spawn(async move {
                victim
                    .statements(&pg_client::Sql::from_static_or_panic(
                        "SELECT pg_sleep(3600)",
                    ))
                    .await
            });

            let mut observer = session::start(config).await.unwrap();

            loop {
                let active = observer
                    .statement(&pg_client::Sql::from_static_or_panic(
                        "SELECT count(*) FROM pg_stat_activity WHERE pid = $1 AND state = 'active'",
                    ))
                    .bind(pid)
                    .execute()
                    .unwrap()
                    .scalar::<i64>()
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
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT pg_terminate_backend($1)",
                ))
                .bind(pid)
                .execute()
                .unwrap()
                .discard_rows()
                .await
                .unwrap();

            let error = sleeping.await.unwrap().unwrap_err();

            let session::execute::Error::Server(server) = error else {
                panic!("expected the server's fatal error, got {error:?}");
            };

            assert_eq!(
                (server.diagnostic().severity(), server.diagnostic().code()),
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
            assert_eq!(session.backend_key().cancel_key().as_slice().len(), 4);

            let one = session
                .statement(&pg_client::Sql::from_static_or_panic("SELECT 1::int4"))
                .execute()
                .unwrap()
                .scalar::<i32>()
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

async fn fake_server(reply: Vec<u8>) -> (u16, tokio::task::JoinHandle<()>) {
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

        socket.write_all(&reply).await.unwrap();
    });

    (port, server)
}

fn fake_config(port: u16, methods: pg_client::authentication::Methods) -> session::Config {
    session::Config {
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
            methods,
        },
        database: pg_client::Database::POSTGRES,
        parameters: session::Parameters::new(),
    }
}

fn scram_only() -> pg_client::authentication::Methods {
    pg_client::authentication::Methods {
        accept_unchallenged: false,
        scram: Some(pg_client::authentication::method::Scram {
            password: pg_client::authentication::Password::Static("unused".parse().unwrap()),
            channel_binding: pg_client::config::ChannelBinding::Prefer,
        }),
        cleartext_password: None,
    }
}

#[tokio::test]
async fn refuses_a_protocol_version_it_does_not_support() {
    let (port, server) = fake_server(
        [
            b"v".as_slice(),
            &12u32.to_be_bytes(),
            &0x0003_0003u32.to_be_bytes(),
            &0u32.to_be_bytes(),
        ]
        .concat(),
    )
    .await;

    let error = session::start(std::sync::Arc::new(fake_config(port, scram_only())))
        .await
        .unwrap_err();

    let session::StartError::UnsupportedProtocol { code } = error else {
        panic!("expected an unsupported protocol error, got {error:?}");
    };

    assert_eq!(code, 0x0003_0003);

    server.await.unwrap();
}

#[tokio::test]
async fn refuses_a_server_that_skips_authentication() {
    let (port, server) =
        fake_server([b"R".as_slice(), &8u32.to_be_bytes(), &0u32.to_be_bytes()].concat()).await;

    let error = session::start(std::sync::Arc::new(fake_config(port, scram_only())))
        .await
        .unwrap_err();

    let session::StartError::MethodNotConfigured {
        requested: pg_client::protocol::backend::AuthenticationRequest::Ok,
    } = error
    else {
        panic!("expected an unchallenged startup to be refused, got {error:?}");
    };

    server.await.unwrap();
}

#[tokio::test]
async fn refuses_md5_it_is_not_configured_for() {
    let (port, server) = fake_server(
        [
            b"R".as_slice(),
            &12u32.to_be_bytes(),
            &5u32.to_be_bytes(),
            &[1, 2, 3, 4],
        ]
        .concat(),
    )
    .await;

    let error = session::start(std::sync::Arc::new(fake_config(port, scram_only())))
        .await
        .unwrap_err();

    let session::StartError::MethodNotConfigured {
        requested: pg_client::protocol::backend::AuthenticationRequest::Md5Password,
    } = error
    else {
        panic!("expected MD5 to be refused, got {error:?}");
    };

    server.await.unwrap();
}

fn cleartext_only(password: pg_client::config::Password) -> pg_client::authentication::Methods {
    pg_client::authentication::Methods {
        accept_unchallenged: false,
        scram: None,
        cleartext_password: Some(pg_client::authentication::method::CleartextPassword {
            password: pg_client::authentication::Password::Static(password),
        }),
    }
}

#[tokio::test]
async fn refuses_a_cleartext_password_it_is_not_configured_for() {
    let (port, server) =
        fake_server([b"R".as_slice(), &8u32.to_be_bytes(), &3u32.to_be_bytes()].concat()).await;

    let error = session::start(std::sync::Arc::new(fake_config(port, scram_only())))
        .await
        .unwrap_err();

    let session::StartError::MethodNotConfigured {
        requested: pg_client::protocol::backend::AuthenticationRequest::CleartextPassword,
    } = error
    else {
        panic!("expected the cleartext request to be refused, got {error:?}");
    };

    server.await.unwrap();
}

#[tokio::test]
async fn refuses_to_send_a_cleartext_password_over_unencrypted_tcp() {
    let (port, server) =
        fake_server([b"R".as_slice(), &8u32.to_be_bytes(), &3u32.to_be_bytes()].concat()).await;

    let error = session::start(std::sync::Arc::new(fake_config(
        port,
        cleartext_only("secret".parse().unwrap()),
    )))
    .await
    .unwrap_err();

    let session::StartError::CleartextOverUnencryptedTcp = error else {
        panic!("expected cleartext over unencrypted TCP to be refused, got {error:?}");
    };

    server.await.unwrap();
}

#[tokio::test]
async fn sends_a_cleartext_password_over_a_unix_socket() {
    use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};

    let path =
        std::env::temp_dir().join(format!("pg-client-cleartext-{}.sock", std::process::id()));
    let listener = tokio::net::UnixListener::bind(&path).unwrap();

    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();

        let mut length = [0; 4];
        socket.read_exact(&mut length).await.unwrap();

        let mut startup = vec![0; usize::try_from(u32::from_be_bytes(length)).unwrap() - 4];
        socket.read_exact(&mut startup).await.unwrap();

        socket
            .write_all(&[b"R".as_slice(), &8u32.to_be_bytes(), &3u32.to_be_bytes()].concat())
            .await
            .unwrap();

        let mut header = [0; 5];
        socket.read_exact(&mut header).await.unwrap();

        let mut password = vec![
            0;
            usize::try_from(u32::from_be_bytes([
                header[1], header[2], header[3], header[4]
            ]))
            .unwrap()
                - 4
        ];
        socket.read_exact(&mut password).await.unwrap();

        let fields = b"SFATAL\0VFATAL\0C28P01\0Mpassword authentication failed\0\0";

        socket
            .write_all(
                &[
                    b"E".as_slice(),
                    &u32::try_from(fields.len() + 4).unwrap().to_be_bytes(),
                    fields,
                ]
                .concat(),
            )
            .await
            .unwrap();

        (header[0], password)
    });

    let mut config = fake_config(0, cleartext_only("secret".parse().unwrap()));
    config.connection.endpoint = pg_client::connection::Endpoint::Unix { path: path.clone() };

    let error = session::start(std::sync::Arc::new(config))
        .await
        .unwrap_err();

    let (tag, password) = server.await.unwrap();
    std::fs::remove_file(&path).unwrap();

    assert_eq!((tag, password.as_slice()), (b'p', b"secret\0".as_slice()));

    let session::StartError::Rejected(rejection) = error else {
        panic!("expected the server's rejection, got {error:?}");
    };

    assert_eq!(
        rejection.diagnostic().code(),
        pg_client::protocol::backend::SqlState::InvalidPassword
    );
}

async fn require_cleartext_passwords_over_tls(session: &mut session::Session) {
    session
        .statements(&pg_client::Sql::from_static_or_panic(
            r"
            CREATE TEMPORARY TABLE hba AS
              SELECT ordinality AS position, line
              FROM regexp_split_to_table(pg_read_file(current_setting('hba_file')), E'\n')
                WITH ORDINALITY AS rules (line, ordinality)
              WHERE line !~ '^\s*(#|$)';

            INSERT INTO hba VALUES (0, 'hostssl all all all password');

            DO $$
            BEGIN
              EXECUTE format(
                'COPY (SELECT line FROM hba ORDER BY position) TO %L WITH (FORMAT csv)',
                current_setting('hba_file')
              );
            END
            $$;

            SELECT pg_reload_conf();
            ",
        ))
        .await
        .unwrap();
}

#[tokio::test]
async fn authenticates_with_a_cleartext_password_over_tls() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "auth-cleartext")
        .ssl_config(pg_ephemeral::definition::SslConfig::Generated {
            hostname: CERTIFICATE_NAME.parse().unwrap(),
        })
        .with_container(async |container| {
            let superuser = container.session_config();

            require_cleartext_passwords_over_tls(
                &mut session::start(std::sync::Arc::clone(&superuser))
                    .await
                    .unwrap(),
            )
            .await;

            let Some(pg_client::authentication::method::Scram {
                password: pg_client::authentication::Password::Static(password),
                ..
            }) = &superuser.authentication.methods.scram
            else {
                panic!("expected SCRAM with a static password");
            };

            let mut config = (*superuser).clone();
            config.authentication.methods = cleartext_only(password.clone());

            let mut session = session::start(std::sync::Arc::new(config)).await.unwrap();

            let method = session
                .statement(&pg_client::Sql::from_static_or_panic(
                    "SELECT auth_method FROM pg_hba_file_rules WHERE rule_number = 1",
                ))
                .execute()
                .unwrap()
                .scalar::<String>()
                .await
                .unwrap()
                .one()
                .await
                .unwrap();

            assert_eq!(method, "password");

            let mut config = (*superuser).clone();
            config.authentication.methods = cleartext_only("wrong".parse().unwrap());

            let error = session::start(std::sync::Arc::new(config))
                .await
                .unwrap_err();

            let session::StartError::Rejected(rejection) = error else {
                panic!("expected the wrong password to be rejected, got {error:?}");
            };

            assert_eq!(
                rejection.diagnostic().code(),
                pg_client::protocol::backend::SqlState::InvalidPassword
            );
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_scram_when_only_unchallenged_is_accepted() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "auth-unchallenged-only")
        .with_container(async |container| {
            let mut config = (*container.session_config()).clone();
            config.authentication.methods = pg_client::authentication::Methods {
                accept_unchallenged: true,
                scram: None,
                cleartext_password: None,
            };

            let error = session::start(std::sync::Arc::new(config))
                .await
                .unwrap_err();

            let session::StartError::MethodNotConfigured {
                requested: pg_client::protocol::backend::AuthenticationRequest::Sasl,
            } = error
            else {
                panic!("expected SASL to be refused, got {error:?}");
            };
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_required_channel_binding_without_tls() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "auth-binding-cleartext")
        .with_container(async |container| {
            let mut config = (*container.session_config()).clone();

            let Some(scram) = &mut config.authentication.methods.scram else {
                panic!("expected SCRAM to be configured");
            };
            scram.channel_binding = pg_client::config::ChannelBinding::Require;

            let error = session::start(std::sync::Arc::new(config))
                .await
                .unwrap_err();

            let session::StartError::ChannelBindingWithoutTls = error else {
                panic!("expected channel binding to need TLS, got {error:?}");
            };
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn refuses_a_notification_as_an_unexpected_message() {
    let backend = ociman::test_backend_setup!();

    definition(backend, "notification")
        .with_session(async |session| {
            run(session, "LISTEN example").await;

            let mut reader = session
                .statement(&pg_client::Sql::from_static_or_panic(
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
            &pg_client::Sql::from_static_or_panic(sql),
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
                    &pg_client::Sql::from_static_or_panic("COPY t FROM STDIN"),
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
                    &pg_client::Sql::from_static_or_panic("SELECT 1"),
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
            &pg_client::Sql::from_static_or_panic(
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
                session::QueueError::Mode(session::ModeError {
                    mode: session::Mode::CommandProcessing
                })
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
            let pid = doomed.backend_key().process_id().get();

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
