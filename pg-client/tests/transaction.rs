use pg_client::sqlx::sqlstate::{SqlState, sqlstate};
use pg_client::sqlx::transaction::{
    AccessMode, IsolationLevel, Options, TransactionError, with_transaction,
};
use pretty_assertions::assert_eq;
use sqlx::Row as _;

fn definition(backend: ociman::Backend) -> pg_ephemeral::Definition {
    pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        "test".parse().unwrap(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30))
}

async fn create_table(connection: &mut sqlx::postgres::PgConnection) {
    sqlx::raw_sql("CREATE TABLE tx_test (id int8 PRIMARY KEY)")
        .execute(connection)
        .await
        .unwrap();
}

async fn row_count(connection: &mut sqlx::postgres::PgConnection) -> i64 {
    let row = sqlx::query("SELECT count(*) FROM tx_test")
        .fetch_one(&mut *connection)
        .await
        .unwrap();
    row.try_get::<i64, _>(0).unwrap()
}

#[derive(Debug, thiserror::Error)]
enum TestError {
    #[error("synthetic action error")]
    Custom,
    #[error(transparent)]
    Sql(#[from] sqlx::Error),
}

#[tokio::test]
async fn commit_persists_writes() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            container
                .client_config()
                .with_sqlx_connection(async |connection| {
                    create_table(connection).await;

                    let result: Result<(), TransactionError<TestError>> = with_transaction(
                        connection,
                        IsolationLevel::Serializable,
                        async |connection| {
                            sqlx::raw_sql("INSERT INTO tx_test VALUES (1)")
                                .execute(&mut *connection)
                                .await?;
                            Ok::<(), TestError>(())
                        },
                    )
                    .await;

                    result.unwrap();

                    assert_eq!(1, row_count(connection).await);
                })
                .await
                .unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn action_error_rolls_back_writes() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            container
                .client_config()
                .with_sqlx_connection(async |connection| {
                    create_table(connection).await;

                    let result: Result<(), TransactionError<TestError>> = with_transaction(
                        connection,
                        IsolationLevel::Serializable,
                        async |connection| {
                            sqlx::raw_sql("INSERT INTO tx_test VALUES (1)")
                                .execute(&mut *connection)
                                .await?;
                            Err::<(), TestError>(TestError::Custom)
                        },
                    )
                    .await;

                    assert!(matches!(
                        result,
                        Err(TransactionError::Action(TestError::Custom))
                    ));

                    assert_eq!(0, row_count(connection).await);
                })
                .await
                .unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn sql_error_inside_action_surfaces_as_action_and_rolls_back() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            container
                .client_config()
                .with_sqlx_connection(async |connection| {
                    create_table(connection).await;

                    let result: Result<(), TransactionError<TestError>> = with_transaction(
                        connection,
                        IsolationLevel::Serializable,
                        async |connection| {
                            sqlx::raw_sql("INSERT INTO tx_test VALUES (1)")
                                .execute(&mut *connection)
                                .await?;
                            sqlx::raw_sql("INSERT INTO tx_test VALUES (1)")
                                .execute(&mut *connection)
                                .await?;
                            Ok::<(), TestError>(())
                        },
                    )
                    .await;

                    assert!(matches!(
                        result,
                        Err(TransactionError::Action(TestError::Sql(_)))
                    ));

                    assert_eq!(0, row_count(connection).await);
                })
                .await
                .unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn isolation_level_is_applied() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    let cases = [
        (IsolationLevel::ReadCommitted, "read committed"),
        (IsolationLevel::RepeatableRead, "repeatable read"),
        (IsolationLevel::Serializable, "serializable"),
    ];

    definition
        .with_container(async |container| {
            container
                .client_config()
                .with_sqlx_connection(async |connection| {
                    for (level, expected) in cases {
                        let observed: String = with_transaction::<_, sqlx::Error, _, _>(
                            connection,
                            level,
                            async |connection| {
                                let row =
                                    sqlx::query("SELECT current_setting('transaction_isolation')")
                                        .fetch_one(&mut *connection)
                                        .await?;
                                row.try_get::<String, _>(0)
                            },
                        )
                        .await
                        .unwrap();

                        assert_eq!(expected, observed, "isolation level {level:?}");
                    }
                })
                .await
                .unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn serializable_read_only_deferrable_options_are_applied() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            container
                .client_config()
                .with_sqlx_connection(async |connection| {
                    let options = Options {
                        isolation: IsolationLevel::Serializable,
                        access_mode: AccessMode::ReadOnly,
                        deferrable: true,
                    };

                    let (isolation, read_only, deferrable): (String, String, String) =
                        with_transaction::<_, sqlx::Error, _, _>(
                            connection,
                            options,
                            async |connection| {
                                let row = sqlx::query(
                                    "SELECT \
                                       current_setting('transaction_isolation') AS isolation, \
                                       current_setting('transaction_read_only') AS read_only, \
                                       current_setting('transaction_deferrable') AS deferrable",
                                )
                                .fetch_one(&mut *connection)
                                .await?;
                                Ok::<_, sqlx::Error>((
                                    row.try_get::<String, _>("isolation")?,
                                    row.try_get::<String, _>("read_only")?,
                                    row.try_get::<String, _>("deferrable")?,
                                ))
                            },
                        )
                        .await
                        .unwrap();

                    assert_eq!("serializable", isolation);
                    assert_eq!("on", read_only);
                    assert_eq!("on", deferrable);
                })
                .await
                .unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn read_only_access_mode_forbids_writes() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            container
                .client_config()
                .with_sqlx_connection(async |connection| {
                    create_table(connection).await;

                    let options = Options {
                        isolation: IsolationLevel::Serializable,
                        access_mode: AccessMode::ReadOnly,
                        deferrable: false,
                    };

                    let result: Result<(), TransactionError<TestError>> =
                        with_transaction(connection, options, async |connection| {
                            sqlx::raw_sql("INSERT INTO tx_test VALUES (1)")
                                .execute(&mut *connection)
                                .await?;
                            Ok::<(), TestError>(())
                        })
                        .await;

                    assert!(matches!(
                        result,
                        Err(TransactionError::Action(TestError::Sql(_)))
                    ));

                    assert_eq!(0, row_count(connection).await);
                })
                .await
                .unwrap();
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn commit_failure_surfaces_as_commit_variant() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            container
                .client_config()
                .with_sqlx_connection(async |connection| {
                    sqlx::raw_sql(
                        "CREATE TABLE tx_test (\
                           id int8 PRIMARY KEY, \
                           parent_id int8 REFERENCES tx_test(id) \
                             DEFERRABLE INITIALLY DEFERRED\
                         )",
                    )
                    .execute(&mut *connection)
                    .await
                    .unwrap();

                    let result: Result<(), TransactionError<TestError>> = with_transaction(
                        connection,
                        IsolationLevel::Serializable,
                        async |connection| {
                            sqlx::raw_sql("INSERT INTO tx_test (id, parent_id) VALUES (1, 999)")
                                .execute(&mut *connection)
                                .await?;
                            Ok::<(), TestError>(())
                        },
                    )
                    .await;

                    assert!(
                        matches!(result, Err(TransactionError::Commit(_))),
                        "expected Commit variant, got: {result:?}"
                    );

                    assert_eq!(0, row_count(connection).await);
                })
                .await
                .unwrap();
        })
        .await
        .unwrap();
}

// Each transaction reads the *other* row then writes its *own* row, forming a
// read-write antidependency cycle that SSI detects at COMMIT. The barrier forces
// both snapshots to be taken before either writes, so the conflict is guaranteed.
async fn cross_update(
    config: &pg_client::Config,
    barrier: &tokio::sync::Barrier,
    read_id: i64,
    write_id: i64,
) -> Result<(), TransactionError<TestError>> {
    config
        .with_sqlx_connection(async |connection| {
            with_transaction(
                connection,
                IsolationLevel::Serializable,
                async |connection| {
                    let row = sqlx::query("SELECT value FROM skew WHERE id = $1")
                        .bind(read_id)
                        .fetch_one(&mut *connection)
                        .await?;
                    let value: i64 = row.try_get(0)?;

                    barrier.wait().await;

                    sqlx::query("UPDATE skew SET value = $1 WHERE id = $2")
                        .bind(value)
                        .bind(write_id)
                        .execute(&mut *connection)
                        .await?;
                    Ok::<(), TestError>(())
                },
            )
            .await
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn serializable_conflict_commit_carries_serialization_failure_sqlstate() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let config = container.client_config();

            config
                .with_sqlx_connection(async |connection| {
                    sqlx::raw_sql("CREATE TABLE skew (id int8 PRIMARY KEY, value int8 NOT NULL)")
                        .execute(&mut *connection)
                        .await?;
                    sqlx::raw_sql("INSERT INTO skew VALUES (1, 10), (2, 20)")
                        .execute(&mut *connection)
                        .await?;
                    Ok::<(), sqlx::Error>(())
                })
                .await
                .unwrap()
                .unwrap();

            let barrier = tokio::sync::Barrier::new(2);

            let (left, right) = tokio::join!(
                cross_update(config, &barrier, 2, 1),
                cross_update(config, &barrier, 1, 2),
            );

            let results = [left, right];

            let committed = results.iter().filter(|result| result.is_ok()).count();

            let serialization_failures = results
                .iter()
                .filter(|result| {
                    matches!(
                        result,
                        Err(TransactionError::Commit(error))
                            if sqlstate(error) == Some(SqlState::SERIALIZATION_FAILURE)
                    )
                })
                .count();

            assert_eq!(
                1, committed,
                "exactly one transaction should commit: {results:?}"
            );
            assert_eq!(
                1, serialization_failures,
                "exactly one transaction should fail with SQLSTATE 40001: {results:?}"
            );
        })
        .await
        .unwrap();
}

// Each transaction locks its `first_id` row, waits for the other to do the same,
// then reaches for the other's row. The crossed lock requests form a cycle that
// Postgres's deadlock detector breaks by cancelling one victim's statement.
async fn lock_in_order(
    config: &pg_client::Config,
    barrier: &tokio::sync::Barrier,
    first_id: i64,
    second_id: i64,
) -> Result<(), TransactionError<TestError>> {
    config
        .with_sqlx_connection(async |connection| {
            with_transaction(
                connection,
                IsolationLevel::ReadCommitted,
                async |connection| {
                    sqlx::query("UPDATE deadlock SET value = value + 1 WHERE id = $1")
                        .bind(first_id)
                        .execute(&mut *connection)
                        .await?;

                    barrier.wait().await;

                    sqlx::query("UPDATE deadlock SET value = value + 1 WHERE id = $1")
                        .bind(second_id)
                        .execute(&mut *connection)
                        .await?;
                    Ok::<(), TestError>(())
                },
            )
            .await
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn deadlock_inside_action_carries_deadlock_detected_sqlstate() {
    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            let config = container.client_config();

            config
                .with_sqlx_connection(async |connection| {
                    sqlx::raw_sql(
                        "CREATE TABLE deadlock (id int8 PRIMARY KEY, value int8 NOT NULL)",
                    )
                    .execute(&mut *connection)
                    .await?;
                    sqlx::raw_sql("INSERT INTO deadlock VALUES (1, 0), (2, 0)")
                        .execute(&mut *connection)
                        .await?;
                    Ok::<(), sqlx::Error>(())
                })
                .await
                .unwrap()
                .unwrap();

            let barrier = tokio::sync::Barrier::new(2);

            let (left, right) = tokio::join!(
                lock_in_order(config, &barrier, 1, 2),
                lock_in_order(config, &barrier, 2, 1),
            );

            let results = [left, right];

            let committed = results.iter().filter(|result| result.is_ok()).count();

            let deadlocks = results
                .iter()
                .filter(|result| {
                    matches!(
                        result,
                        Err(TransactionError::Action(TestError::Sql(error)))
                            if sqlstate(error) == Some(SqlState::DEADLOCK_DETECTED)
                    )
                })
                .count();

            assert_eq!(
                1, committed,
                "exactly one transaction should commit: {results:?}"
            );
            assert_eq!(
                1, deadlocks,
                "exactly one transaction should be the deadlock victim: {results:?}"
            );
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn rollback_failure_surfaces_both_action_and_rollback_errors() {
    use sqlx::ConnectOptions as _;

    let backend = ociman::test_backend_setup!();
    let definition = definition(backend);

    definition
        .with_container(async |container| {
            // Connect directly: with_transaction operates on a &mut PgConnection, and
            // a direct connection lets us kill the backend mid-action so the ROLLBACK
            // that with_transaction issues next also fails.
            let options = container.client_config().to_sqlx_connect_options().unwrap();
            let mut connection = options.connect().await.unwrap();

            let result: Result<(), TransactionError<TestError>> = with_transaction(
                &mut connection,
                IsolationLevel::ReadCommitted,
                async |connection| {
                    // Terminate our own backend; the connection dies, so the ROLLBACK
                    // with_transaction attempts after this action error also fails.
                    let _ = sqlx::raw_sql("SELECT pg_terminate_backend(pg_backend_pid())")
                        .execute(&mut *connection)
                        .await;
                    Err::<(), TestError>(TestError::Custom)
                },
            )
            .await;

            assert!(
                matches!(
                    result,
                    Err(TransactionError::ActionRollback {
                        action: TestError::Custom,
                        ..
                    })
                ),
                "expected ActionRollback carrying the action error, got: {result:?}"
            );
        })
        .await
        .unwrap();
}
