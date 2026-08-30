#![cfg(feature = "tracing")]

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
struct Span {
    name: &'static str,
    parent: Option<u64>,
    fields: BTreeMap<&'static str, String>,
    handles: usize,
}

#[derive(Clone, Default)]
struct Collector(Arc<Mutex<Spans>>);

#[derive(Default)]
struct Spans {
    next: u64,
    spans: BTreeMap<u64, Span>,
}

impl Collector {
    fn named(&self, name: &str) -> Vec<(u64, Span)> {
        self.0
            .lock()
            .unwrap()
            .spans
            .iter()
            .filter(|(_, span)| span.name == name)
            .map(|(id, span)| (*id, span.clone()))
            .collect()
    }

    fn statement(&self, sql: &str) -> (u64, Span) {
        let mut matching = self
            .named("pg.statement")
            .into_iter()
            .filter(|(_, span)| span.fields.get("db.query.text").map(String::as_str) == Some(sql));

        let statement = matching.next().unwrap();

        assert!(matching.next().is_none());

        statement
    }
}

struct Fields<'a>(&'a mut BTreeMap<&'static str, String>);

impl tracing::field::Visit for Fields<'_> {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.0.insert(field.name(), value.to_owned());
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn core::fmt::Debug) {
        self.0.insert(field.name(), format!("{value:?}"));
    }
}

impl tracing::Subscriber for Collector {
    fn enabled(&self, _metadata: &tracing::Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, attributes: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        let mut spans = self.0.lock().unwrap();

        spans.next += 1;

        let id = spans.next;
        let mut fields = BTreeMap::new();

        attributes.record(&mut Fields(&mut fields));

        spans.spans.insert(
            id,
            Span {
                name: attributes.metadata().name(),
                parent: attributes.parent().map(tracing::span::Id::into_u64),
                fields,
                handles: 1,
            },
        );

        tracing::span::Id::from_u64(id)
    }

    fn record(&self, span: &tracing::span::Id, values: &tracing::span::Record<'_>) {
        let mut spans = self.0.lock().unwrap();
        let span = spans.spans.get_mut(&span.into_u64()).unwrap();

        values.record(&mut Fields(&mut span.fields));
    }

    fn record_follows_from(&self, _span: &tracing::span::Id, _follows: &tracing::span::Id) {}

    fn event(&self, _event: &tracing::Event<'_>) {}

    fn enter(&self, _span: &tracing::span::Id) {}

    fn exit(&self, _span: &tracing::span::Id) {}

    fn clone_span(&self, span: &tracing::span::Id) -> tracing::span::Id {
        self.0
            .lock()
            .unwrap()
            .spans
            .get_mut(&span.into_u64())
            .unwrap()
            .handles += 1;

        span.clone()
    }

    fn try_close(&self, span: tracing::span::Id) -> bool {
        let mut spans = self.0.lock().unwrap();
        let span = spans.spans.get_mut(&span.into_u64()).unwrap();

        span.handles -= 1;

        span.handles == 0
    }
}

fn definition(backend: ociman::Backend, instance: &str) -> pg_ephemeral::Definition {
    pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        instance.parse().unwrap(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30))
}

const SERIES: &str = "SELECT generate_series(1, 3)";

#[tokio::test]
async fn records_a_completed_statement() {
    let backend = ociman::test_backend_setup!();
    let collector = Collector::default();
    let _default = tracing::subscriber::set_default(collector.clone());

    definition(backend, "statement")
        .with_session(async |session| {
            let rows = session
                .statement(&pg_client::Sql::from_static_or_panic(SERIES))
                .execute()
                .unwrap()
                .scalar::<i32>()
                .await
                .unwrap()
                .into_vec()
                .await
                .unwrap();

            assert_eq!(rows, vec![1, 2, 3]);
        })
        .await
        .unwrap();

    let (_, statement) = collector.statement(SERIES);

    assert_eq!(statement.handles, 0);
    assert_eq!(
        statement.fields,
        BTreeMap::from([
            ("db.operation.name", "SELECT".to_owned()),
            ("db.query.text", SERIES.to_owned()),
            ("db.response.returned_rows", "3".to_owned()),
            ("db.system.name", "postgresql".to_owned()),
            ("otel.kind", "client".to_owned()),
        ]),
    );
}

const DIVISION: &str = "SELECT 1 / 0";

#[tokio::test]
async fn records_the_sqlstate_of_a_rejected_statement() {
    let backend = ociman::test_backend_setup!();
    let collector = Collector::default();
    let _default = tracing::subscriber::set_default(collector.clone());

    definition(backend, "rejected")
        .with_session(async |session| {
            let Err(error) = session
                .statement(&pg_client::Sql::from_static_or_panic(DIVISION))
                .execute()
                .unwrap()
                .scalar::<i32>()
                .await
            else {
                panic!("expected the server to reject the statement");
            };

            assert!(matches!(
                error,
                pg_client::session::statement::Error::Server(_)
            ));
        })
        .await
        .unwrap();

    let (_, statement) = collector.statement(DIVISION);

    assert_eq!(statement.handles, 0);
    assert_eq!(
        statement.fields,
        BTreeMap::from([
            ("db.query.text", DIVISION.to_owned()),
            ("db.response.returned_rows", "0".to_owned()),
            ("db.response.status_code", "22012".to_owned()),
            ("db.system.name", "postgresql".to_owned()),
            ("error.type", "22012".to_owned()),
            ("otel.kind", "client".to_owned()),
            ("otel.status_code", "ERROR".to_owned()),
        ]),
    );
}

const ABANDONED: &str = "SELECT generate_series(1, 2)";
const FOLLOWING: &str = "SELECT 1";

#[tokio::test]
async fn drains_an_abandoned_statement_under_the_next_one() {
    let backend = ociman::test_backend_setup!();
    let collector = Collector::default();
    let _default = tracing::subscriber::set_default(collector.clone());

    definition(backend, "drain")
        .with_session(async |session| {
            drop(
                session
                    .statement(&pg_client::Sql::from_static_or_panic(ABANDONED))
                    .execute()
                    .unwrap(),
            );

            let one = session
                .statement(&pg_client::Sql::from_static_or_panic(FOLLOWING))
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
        .unwrap();

    let (_, abandoned) = collector.statement(ABANDONED);
    let (following, _) = collector.statement(FOLLOWING);

    assert_eq!(abandoned.handles, 0);
    assert_eq!(
        abandoned.fields.get("pg.abandoned").map(String::as_str),
        Some("true")
    );

    let drains = collector.named("pg.drain");

    assert_eq!(drains.len(), 1);
    assert_eq!(drains[0].1.parent, Some(following));
    assert_eq!(drains[0].1.handles, 0);
}
