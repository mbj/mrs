use core::marker::PhantomData;
use core::ops::DerefMut;

use crate::protocol;
use crate::protocol::{backend, frontend};
use crate::{name, value};

use super::{Mode, QueueError, ReceiveError, Session};

#[derive(Debug)]
pub struct Builder<'q, S: DerefMut<Target = Session>> {
    session: S,
    sql: crate::Sql<'q>,
    types: smallvec::SmallVec<[protocol::Oid; 8]>,
    values: Vec<u8>,
    ranges: smallvec::SmallVec<[Option<core::ops::Range<usize>>; 8]>,
}

impl<'q, S: DerefMut<Target = Session>> Builder<'q, S> {
    pub fn new(session: S, sql: &crate::Sql<'q>) -> Self {
        Self {
            session,
            sql: sql.clone(),
            types: smallvec::SmallVec::new(),
            values: Vec::new(),
            ranges: smallvec::SmallVec::new(),
        }
    }

    pub fn bind<T: value::Encode>(
        mut self,
        value: T,
    ) -> <T::Outcome as value::Outcome>::Checked<Self, Error> {
        let start = self.values.len();
        let null = value.is_null();

        let written = if null {
            <T::Outcome as value::Outcome>::done()
        } else {
            value.write_to(&mut frontend::Writer::new(&mut self.values))
        };

        value::Outcome::check(
            written,
            self,
            |mut builder| {
                let end = builder.values.len();

                builder.record(T::BUILTIN, (!null).then_some(start..end));

                builder
            },
            |result, mut builder| {
                builder
                    .settle(T::BUILTIN, start, null, result)
                    .map(|()| builder)
            },
        )
    }

    pub fn bind_value(mut self, value: &value::Value) -> Result<Self, Error> {
        let start = self.values.len();

        let written = value.write_to(&mut frontend::Writer::new(&mut self.values));

        self.settle(value.builtin(), start, false, written)?;

        Ok(self)
    }

    fn settle(
        &mut self,
        builtin: crate::catalog::Builtin,
        start: usize,
        null: bool,
        written: Result<(), value::EncodeError>,
    ) -> Result<(), Error> {
        let checked = written.and_then(|()| {
            let length = self.values.len() - start;

            i32::try_from(length)
                .map(|_| ())
                .map_err(|_| frontend::TooLong { length }.into())
        });

        if let Err(source) = checked {
            self.values.truncate(start);

            return Err(Error::Bind {
                index: self.types.len(),
                source,
            });
        }

        let end = self.values.len();

        self.record(builtin, (!null).then_some(start..end));

        Ok(())
    }

    fn record(&mut self, builtin: crate::catalog::Builtin, range: Option<core::ops::Range<usize>>) {
        self.types.push(builtin.oid());
        self.ranges.push(range);
    }

    pub fn execute(self) -> Result<Run<S>, Error> {
        let Self {
            mut session,
            sql,
            types,
            values,
            ranges,
        } = self;

        let span = crate::trace::debug_span!(
            "pg.statement",
            otel.kind = "client",
            otel.status_code = crate::trace::field::Empty,
            db.system.name = "postgresql",
            db.query.text = sql.as_str(),
            db.operation.name = crate::trace::field::Empty,
            db.response.returned_rows = crate::trace::field::Empty,
            db.response.status_code = crate::trace::field::Empty,
            error.type = crate::trace::field::Empty,
            pg.abandoned = crate::trace::field::Empty,
        );

        if i16::try_from(types.len()).is_err() {
            return Err(Error::TooManyParameters { count: types.len() });
        }

        let parameters: smallvec::SmallVec<[Option<&[u8]>; 8]> = ranges
            .into_iter()
            .map(|range| range.map(|range| &values[range]))
            .collect();

        let statement = session.queue_parse(frontend::StatementName::Unnamed, &sql, &types)?;

        let portal = session.queue_bind(
            &statement,
            frontend::PortalName::Unnamed,
            frontend::Formats::All(protocol::Format::Binary),
            &parameters,
            frontend::Formats::All(protocol::Format::Binary),
        )?;

        session.queue_describe_portal(&portal)?;
        session.queue_execute(&portal, None)?;
        session.queue_sync()?;

        Ok(Run::new(session, span))
    }
}

#[derive(Clone, Debug)]
pub struct Completion(protocol::Frame);

impl Completion {
    #[must_use]
    pub fn command(&self) -> backend::CommandComplete<'_> {
        let Ok(backend::Message::CommandComplete(command)) = backend::Message::decode(&self.0)
        else {
            unreachable!("a Completion is only built from a decoded CommandComplete");
        };

        command
    }
}

#[derive(Clone, Debug)]
pub enum Outcome {
    Complete(Completion),
    Empty,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Queue(#[from] QueueError),
    #[error("writing a message: {0}")]
    Send(#[from] protocol::SendError),
    #[error(transparent)]
    Receive(#[from] ReceiveError),
    #[error("server rejected the statement: {0}")]
    Server(super::ServerError),
    #[error("unexpected message: {}", describe(.0))]
    Unexpected(protocol::Frame),
    #[error("row has {actual} columns, the description has {expected}")]
    ColumnCount { expected: usize, actual: usize },
    #[error(transparent)]
    Column(#[from] ColumnError),
    #[error(transparent)]
    Decode(#[from] DecodeError),
    #[error("statement returns {columns} columns, a scalar needs exactly one")]
    NotOneColumn { columns: usize },
    #[error("statement returned no rows")]
    NoRows,
    #[error("statement returned {actual} rows, at most one was expected")]
    TooManyRows { actual: usize },
    #[error("{count} parameters exceed the protocol's limit of 32767")]
    TooManyParameters { count: usize },
    #[error("parameter {index} cannot be encoded")]
    Bind {
        index: usize,
        #[source]
        source: value::EncodeError,
    },
}

fn describe(frame: &protocol::Frame) -> String {
    match backend::Message::decode(frame) {
        Ok(message) => format!("{message:?}"),
        Err(error) => format!("undecodable frame: {error}"),
    }
}

#[derive(Clone, Debug)]
pub struct Description(Option<protocol::Frame>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ColumnReference<'a> {
    Index(usize),
    Name(name::Column<'a>),
}

impl From<usize> for ColumnReference<'_> {
    fn from(index: usize) -> Self {
        Self::Index(index)
    }
}

impl<'a> From<name::Column<'a>> for ColumnReference<'a> {
    fn from(name: name::Column<'a>) -> Self {
        Self::Name(name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ColumnError {
    #[error("the statement returns no rows")]
    NoColumns,
    #[error("column {index} is out of range, the description has {columns}")]
    IndexOutOfRange { index: usize, columns: usize },
    #[error("no column is named {name}")]
    NoSuchColumn { name: String },
    #[error("{count} columns are named {name}")]
    AmbiguousColumn { name: String, count: usize },
    #[error(
        "column {index} has type {data_type:?} in {format:?} format, which the requested type does not decode"
    )]
    Type {
        index: usize,
        data_type: protocol::Oid,
        format: protocol::Format,
    },
    #[error("the statement returns {actual} columns, the row type takes {expected}")]
    Count { expected: usize, actual: usize },
}

impl Description {
    pub fn columns<T: FromRow>(&self) -> Result<Columns<T>, ColumnError> {
        T::columns(self).map(Columns)
    }

    #[must_use]
    pub fn column_count(&self) -> usize {
        self.0
            .as_ref()
            .map_or(0, |frame| row_description(frame).len())
    }

    pub fn column<'c, T: value::Decode>(
        &self,
        reference: impl Into<ColumnReference<'c>>,
    ) -> Result<Column<T>, ColumnError> {
        let Some(frame) = &self.0 else {
            return Err(ColumnError::NoColumns);
        };

        let columns = row_description(frame);

        let (index, column) = match reference.into() {
            ColumnReference::Index(index) => columns
                .into_iter()
                .nth(index)
                .map(|column| (index, column))
                .ok_or(ColumnError::IndexOutOfRange {
                    index,
                    columns: columns.len(),
                })?,
            ColumnReference::Name(name) => {
                let mut matches = columns
                    .into_iter()
                    .enumerate()
                    .filter(|(_, column)| column.name.as_str() == name.as_str());

                let found = matches.next().ok_or_else(|| ColumnError::NoSuchColumn {
                    name: name.to_string(),
                })?;

                let others = matches.count();

                if others > 0 {
                    return Err(ColumnError::AmbiguousColumn {
                        name: name.to_string(),
                        count: others + 1,
                    });
                }

                found
            }
        };

        if !T::accepts(&column) {
            return Err(ColumnError::Type {
                index,
                data_type: column.data_type,
                format: column.format,
            });
        }

        Ok(Column {
            index,
            description: frame.clone(),
            decoded: PhantomData,
        })
    }
}

fn row_description(frame: &protocol::Frame) -> backend::RowDescription<'_> {
    let Ok(backend::Message::RowDescription(description)) = backend::Message::decode(frame) else {
        unreachable!("a description frame is only kept once decoded as a RowDescription");
    };

    description
}

pub trait FromRow {
    type Columns;
    type Output<'r>;

    fn columns(description: &Description) -> Result<Self::Columns, ColumnError>;

    fn from_row<'r>(
        columns: &Self::Columns,
        row: &Row<'r>,
    ) -> Result<Self::Output<'r>, DecodeError>;
}

pub struct Columns<T: FromRow>(T::Columns);

macro_rules! tuple_from_row {
    ($count:literal; $($element:ident $index:tt),+) => {
        impl<$($element: value::Decode),+> FromRow for ($($element,)+) {
            type Columns = ($(Column<$element>,)+);
            type Output<'r> = ($($element::Output<'r>,)+);

            fn columns(description: &Description) -> Result<Self::Columns, ColumnError> {
                let actual = description.column_count();

                if actual != $count {
                    return Err(ColumnError::Count {
                        expected: $count,
                        actual,
                    });
                }

                Ok(($(description.column::<$element>($index)?,)+))
            }

            fn from_row<'r>(
                columns: &Self::Columns,
                row: &Row<'r>,
            ) -> Result<Self::Output<'r>, DecodeError> {
                Ok(($(row.decode(&columns.$index)?,)+))
            }
        }
    };
}

tuple_from_row!(1; A 0);
tuple_from_row!(2; A 0, B 1);
tuple_from_row!(3; A 0, B 1, C 2);
tuple_from_row!(4; A 0, B 1, C 2, D 3);
tuple_from_row!(5; A 0, B 1, C 2, D 3, E 4);
tuple_from_row!(6; A 0, B 1, C 2, D 3, E 4, F 5);
tuple_from_row!(7; A 0, B 1, C 2, D 3, E 4, F 5, G 6);
tuple_from_row!(8; A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
tuple_from_row!(9; A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
tuple_from_row!(10; A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
tuple_from_row!(11; A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
tuple_from_row!(12; A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);
tuple_from_row!(13; A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12);
tuple_from_row!(14; A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13);
tuple_from_row!(15; A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14);
tuple_from_row!(16; A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15);

#[derive(Debug)]
pub struct Column<T> {
    index: usize,
    description: protocol::Frame,
    decoded: PhantomData<fn() -> T>,
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("the column was resolved against another statement's description")]
    ForeignColumn,
    #[error("column {index} could not be decoded")]
    Value {
        index: usize,
        #[source]
        source: value::DecodeError,
    },
}

// Located once per row: a value's position depends on every length before it.
#[derive(Clone, Debug)]
pub struct Row<'r> {
    description: &'r protocol::Frame,
    values: smallvec::SmallVec<[Option<&'r [u8]>; 16]>,
}

impl<'r> Row<'r> {
    pub fn decode<T: value::Decode>(
        &self,
        column: &Column<T>,
    ) -> Result<T::Output<'r>, DecodeError> {
        if !core::ptr::eq(
            column.description.body().as_slice(),
            self.description.body().as_slice(),
        ) {
            return Err(DecodeError::ForeignColumn);
        }

        self.decode_at::<T>(column.index)
    }

    pub fn decode_row<T: FromRow>(
        &self,
        columns: &Columns<T>,
    ) -> Result<T::Output<'r>, DecodeError> {
        T::from_row(&columns.0, self)
    }

    fn decode_at<T: value::Decode>(&self, index: usize) -> Result<T::Output<'r>, DecodeError> {
        let value = *self
            .values
            .get(index)
            .expect("the row's column count was checked against the description");

        match value {
            None => T::decode_null(),
            Some(bytes) => T::decode(bytes),
        }
        .map_err(|source| DecodeError::Value { index, source })
    }
}

#[derive(Debug)]
enum State {
    Parsing,
    Binding,
    Describing,
    Rows,
    Completed(Outcome),
    Done(Option<Outcome>),
}

enum Event {
    Ignore,
    ParseComplete,
    BindComplete,
    Described { columns: usize },
    NoData,
    DataRow { columns: usize },
    Complete,
    Empty,
    Ready,
    Rejected { severity: backend::Severity },
    Other,
}

enum Step {
    Progress,
    Row(protocol::Frame),
    Done,
}

#[derive(Debug)]
pub struct Run<S: DerefMut<Target = Session>> {
    session: S,
    state: State,
    description: Option<protocol::Frame>,
    columns: usize,
    current: Option<protocol::Frame>,
    rows: u64,
    span: crate::trace::Span,
}

impl<S: DerefMut<Target = Session>> Run<S> {
    const fn new(session: S, span: crate::trace::Span) -> Self {
        Self {
            session,
            state: State::Parsing,
            description: None,
            columns: 0,
            current: None,
            rows: 0,
            span,
        }
    }

    pub async fn describe(&mut self) -> Result<Description, Error> {
        while matches!(
            self.state,
            State::Parsing | State::Binding | State::Describing
        ) {
            match self.step().await? {
                Step::Progress => (),
                Step::Row(_) => unreachable!("rows only arrive after the description"),
                Step::Done => break,
            }
        }

        Ok(Description(self.description.clone()))
    }

    pub async fn next(&mut self) -> Result<Option<Row<'_>>, Error> {
        loop {
            match self.step().await? {
                Step::Progress => (),
                Step::Row(frame) => {
                    self.current = Some(frame);
                    break;
                }
                Step::Done => return Ok(None),
            }
        }

        let (Some(description), Some(current)) = (&self.description, &self.current) else {
            unreachable!("a row is only accepted after a RowDescription");
        };

        let Ok(backend::Message::DataRow(values)) = backend::Message::decode(current) else {
            unreachable!("a row frame is only kept once decoded as a DataRow");
        };

        Ok(Some(Row {
            description,
            values: values.into_iter().collect(),
        }))
    }

    pub async fn discard_rows(mut self) -> Result<(), Error> {
        while self.next().await?.is_some() {}

        Ok(())
    }

    pub async fn decode_column<T: for<'r> value::Decode<Output<'r> = T>>(
        mut self,
        reference: impl Into<ColumnReference<'_>>,
    ) -> Result<Decoder<S, impl FnMut(Row<'_>) -> Result<T, DecodeError>>, Error> {
        let index = self.describe().await?.column::<T>(reference)?.index;

        Ok(self.decode(move |row| row.decode_at::<T>(index)))
    }

    pub async fn rows_as<T, O>(
        mut self,
    ) -> Result<Decoder<S, impl FnMut(Row<'_>) -> Result<O, DecodeError>>, Error>
    where
        T: for<'r> FromRow<Output<'r> = O>,
    {
        let columns = self.describe().await?.columns::<T>()?;

        Ok(self.decode(move |row| row.decode_row(&columns)))
    }

    pub async fn rows<T: for<'r> FromRow<Output<'r> = T>>(
        self,
    ) -> Result<Decoder<S, impl FnMut(Row<'_>) -> Result<T, DecodeError>>, Error> {
        self.rows_as::<T, T>().await
    }

    pub async fn scalar<T: for<'r> value::Decode<Output<'r> = T>>(
        mut self,
    ) -> Result<Decoder<S, impl FnMut(Row<'_>) -> Result<T, DecodeError>>, Error> {
        let description = self.describe().await?;
        let columns = description.column_count();

        if columns != 1 {
            return Err(Error::NotOneColumn { columns });
        }

        let index = description.column::<T>(0)?.index;

        Ok(self.decode(move |row| row.decode_at::<T>(index)))
    }

    pub fn decode<T, F>(self, decode: F) -> Decoder<S, F>
    where
        F: FnMut(Row<'_>) -> Result<T, DecodeError>,
    {
        Decoder { run: self, decode }
    }

    #[must_use]
    pub const fn outcome(&self) -> Option<&Outcome> {
        match &self.state {
            State::Completed(outcome) | State::Done(Some(outcome)) => Some(outcome),
            _ => None,
        }
    }

    async fn step(&mut self) -> Result<Step, Error> {
        let result = self.advance().await;

        if let Err(error) = &result {
            record_error(&self.span, error);
        }

        if matches!(self.state, State::Done(_)) {
            self.span.record("db.response.returned_rows", self.rows);
            self.span = crate::trace::Span::none();
        }

        result
    }

    async fn advance(&mut self) -> Result<Step, Error> {
        if matches!(self.state, State::Done(_)) {
            return Ok(Step::Done);
        }

        self.session.drain_unfinished(&self.span).await?;

        let frame = self.session.receive().await?;

        let event = match backend::Message::decode(&frame).map_err(ReceiveError::Decode)? {
            backend::Message::NoticeResponse(_) | backend::Message::ParameterStatus(_) => {
                Event::Ignore
            }
            backend::Message::ParseComplete => Event::ParseComplete,
            backend::Message::BindComplete => Event::BindComplete,
            backend::Message::RowDescription(description) => Event::Described {
                columns: description.len(),
            },
            backend::Message::NoData => Event::NoData,
            backend::Message::DataRow(row) => Event::DataRow { columns: row.len() },
            backend::Message::CommandComplete(command) => {
                self.span.record("db.operation.name", operation(&command));
                Event::Complete
            }
            backend::Message::EmptyQueryResponse => Event::Empty,
            backend::Message::ReadyForQuery(_) => Event::Ready,
            backend::Message::ErrorResponse(error) => Event::Rejected {
                severity: error.diagnostic.severity(),
            },
            _ => Event::Other,
        };

        let state = core::mem::replace(&mut self.state, State::Done(None));

        let (error, ready) = match (state, event) {
            (state, Event::Ignore) => {
                self.state = state;
                return Ok(Step::Progress);
            }
            (State::Parsing, Event::ParseComplete) => {
                self.state = State::Binding;
                return Ok(Step::Progress);
            }
            (State::Binding, Event::BindComplete) => {
                self.state = State::Describing;
                return Ok(Step::Progress);
            }
            (State::Describing, Event::Described { columns }) => {
                self.description = Some(frame);
                self.columns = columns;
                self.state = State::Rows;
                return Ok(Step::Progress);
            }
            (State::Describing, Event::NoData) => {
                self.state = State::Rows;
                return Ok(Step::Progress);
            }
            (State::Rows, Event::DataRow { columns }) if self.description.is_some() => {
                self.state = State::Rows;

                if columns == self.columns {
                    self.rows += 1;
                    return Ok(Step::Row(frame));
                }

                (
                    Error::ColumnCount {
                        expected: self.columns,
                        actual: columns,
                    },
                    false,
                )
            }
            (State::Rows, Event::Complete) => {
                self.state = State::Completed(Outcome::Complete(Completion(frame)));
                return Ok(Step::Progress);
            }
            (State::Rows, Event::Empty) if self.description.is_none() => {
                self.state = State::Completed(Outcome::Empty);
                return Ok(Step::Progress);
            }
            (State::Completed(outcome), Event::Ready) => {
                self.state = State::Done(Some(outcome));
                return Ok(Step::Done);
            }
            (_, Event::Rejected { severity }) => (
                Error::Server(super::ServerError(frame)),
                severity.ends_session(),
            ),
            (_, event) => {
                if self.session.mode == Mode::CopyIn {
                    self.session.refuse_copy_in()?;
                }

                (Error::Unexpected(frame), matches!(event, Event::Ready))
            }
        };

        if !ready {
            self.session.read_to_ready().await?;
        }

        Err(error)
    }
}

fn operation<'a>(command: &backend::CommandComplete<'a>) -> &'a str {
    match command {
        backend::CommandComplete::Insert { .. } => "INSERT",
        backend::CommandComplete::Delete { .. } => "DELETE",
        backend::CommandComplete::Update { .. } => "UPDATE",
        backend::CommandComplete::Merge { .. } => "MERGE",
        backend::CommandComplete::Select { .. } => "SELECT",
        backend::CommandComplete::Move { .. } => "MOVE",
        backend::CommandComplete::Fetch { .. } => "FETCH",
        backend::CommandComplete::Copy { .. } => "COPY",
        backend::CommandComplete::Other(tag) => tag,
    }
}

fn record_error(span: &crate::trace::Span, error: &Error) {
    if span.is_disabled() {
        return;
    }

    span.record("otel.status_code", "ERROR");

    let error_type = match error {
        Error::Server(server) => {
            let code = server.diagnostic().code();

            span.record("db.response.status_code", code.as_str());
            span.record("error.type", code.as_str());

            return;
        }
        Error::Queue(_) => "queue",
        Error::Send(_) => "send",
        Error::Receive(_) => "receive",
        Error::Unexpected(_) => "unexpected",
        Error::ColumnCount { .. } => "column_count",
        Error::Column(_) => "column",
        Error::Decode(_) => "decode",
        Error::NotOneColumn { .. } => "not_one_column",
        Error::NoRows => "no_rows",
        Error::TooManyRows { .. } => "too_many_rows",
        Error::TooManyParameters { .. } => "too_many_parameters",
        Error::Bind { .. } => "bind",
    };

    span.record("error.type", error_type);
}

pub struct Decoder<S: DerefMut<Target = Session>, F> {
    run: Run<S>,
    decode: F,
}

impl<T, S: DerefMut<Target = Session>, F> Decoder<S, F>
where
    F: FnMut(Row<'_>) -> Result<T, DecodeError>,
{
    pub async fn next(&mut self) -> Result<Option<T>, Error> {
        match self.run.next().await? {
            Some(row) => Ok(Some((self.decode)(row)?)),
            None => Ok(None),
        }
    }

    pub async fn into_vec(mut self) -> Result<Vec<T>, Error> {
        let mut values = Vec::new();

        while let Some(value) = self.next().await? {
            values.push(value);
        }

        Ok(values)
    }

    pub async fn one(self) -> Result<T, Error> {
        self.max_one().await?.ok_or(Error::NoRows)
    }

    pub async fn max_one(mut self) -> Result<Option<T>, Error> {
        let Some(value) = self.next().await? else {
            return Ok(None);
        };

        let mut actual: usize = 1;

        while self.run.next().await?.is_some() {
            actual = actual.saturating_add(1);
        }

        if actual == 1 {
            Ok(Some(value))
        } else {
            Err(Error::TooManyRows { actual })
        }
    }
}

impl<S: DerefMut<Target = Session>> Drop for Run<S> {
    fn drop(&mut self) {
        if !matches!(self.state, State::Done(_)) {
            self.session.unfinished = true;
            self.span.record("pg.abandoned", true);
            self.span.record("db.response.returned_rows", self.rows);
        }
    }
}
