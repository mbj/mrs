use core::marker::PhantomData;

use crate::protocol;
use crate::protocol::{backend, frontend};
use crate::{name, value};

use super::{Mode, ModeError, ReceiveError, Session};

#[derive(Debug)]
pub struct Builder<'s> {
    session: &'s mut Session,
    sql: frontend::Sql,
    types: Vec<protocol::Oid>,
    values: Vec<u8>,
    ranges: Vec<Option<core::ops::Range<usize>>>,
}

impl<'s> Builder<'s> {
    pub(super) fn new(session: &'s mut Session, sql: &frontend::Sql) -> Self {
        Self {
            session,
            sql: sql.clone(),
            types: Vec::new(),
            values: Vec::new(),
            ranges: Vec::new(),
        }
    }

    #[must_use]
    pub fn bind<T: value::Encode>(mut self, value: T) -> Self {
        self.types.push(T::BUILTIN.oid());

        if value.is_null() {
            self.ranges.push(None);
        } else {
            let start = self.values.len();

            value.write_to(&mut frontend::Writer::new(&mut self.values));

            self.ranges.push(Some(start..self.values.len()));
        }

        self
    }

    #[must_use]
    pub fn bind_value(mut self, value: &value::Value) -> Self {
        self.types.push(value.builtin().oid());

        let start = self.values.len();

        frontend::Parameter::write_to(value, &mut frontend::Writer::new(&mut self.values));

        self.ranges.push(Some(start..self.values.len()));

        self
    }

    pub fn execute(self) -> Result<Reader<'s>, Error> {
        let Self {
            session,
            sql,
            types,
            values,
            ranges,
        } = self;

        if i16::try_from(types.len()).is_err() {
            return Err(Error::TooManyParameters { count: types.len() });
        }

        let parameters: Vec<Option<&[u8]>> = ranges
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

        Ok(Reader::new(session))
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
    Mode(#[from] ModeError),
    #[error("writing a message: {0}")]
    Send(#[from] protocol::SendError),
    #[error(transparent)]
    Receive(#[from] ReceiveError),
    #[error("server rejected the statement: {message}")]
    Server {
        severity: backend::Severity,
        code: backend::SqlState,
        message: String,
    },
    #[error("unexpected message: {}", describe(.0))]
    Unexpected(protocol::Frame),
    #[error("row has {actual} columns, the description has {expected}")]
    ColumnCount { expected: usize, actual: usize },
    #[error(transparent)]
    Column(#[from] ColumnError),
    #[error(transparent)]
    Decode(#[from] DecodeError),
    #[error("statement returned no rows")]
    NoRows,
    #[error("statement returned {actual} rows, at most one was expected")]
    TooManyRows { actual: usize },
    #[error("{count} parameters exceed the protocol's limit of 32767")]
    TooManyParameters { count: usize },
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
}

impl Description {
    pub fn column<'c, T: for<'r> value::Decode<'r>>(
        &self,
        reference: impl Into<ColumnReference<'c>>,
    ) -> Result<Column<'_, T>, ColumnError> {
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

        if !<T as value::Decode<'_>>::accepts(&column) {
            return Err(ColumnError::Type {
                index,
                data_type: column.data_type,
                format: column.format,
            });
        }

        Ok(Column {
            index,
            description: frame,
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

#[derive(Debug)]
pub struct Column<'d, T> {
    index: usize,
    description: &'d protocol::Frame,
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

#[derive(Clone, Copy, Debug)]
pub struct Row<'r> {
    description: &'r protocol::Frame,
    values: backend::DataRow<'r>,
}

impl<'r> Row<'r> {
    pub fn decode<T: value::Decode<'r>>(&self, column: &Column<'_, T>) -> Result<T, DecodeError> {
        if !core::ptr::eq(
            column.description.body().as_slice(),
            self.description.body().as_slice(),
        ) {
            return Err(DecodeError::ForeignColumn);
        }

        self.decode_at(column.index)
    }

    fn decode_at<T: value::Decode<'r>>(&self, index: usize) -> Result<T, DecodeError> {
        let value = self
            .values
            .into_iter()
            .nth(index)
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
    Described {
        columns: usize,
    },
    NoData,
    DataRow {
        columns: usize,
    },
    Complete,
    Empty,
    Ready,
    Rejected {
        severity: backend::Severity,
        code: backend::SqlState,
        message: String,
    },
    Other,
}

enum Step {
    Progress,
    Row(protocol::Frame),
    Done,
}

#[derive(Debug)]
pub struct Reader<'s> {
    session: &'s mut Session,
    state: State,
    description: Option<protocol::Frame>,
    columns: usize,
    current: Option<protocol::Frame>,
}

impl<'s> Reader<'s> {
    pub(super) const fn new(session: &'s mut Session) -> Self {
        Self {
            session,
            state: State::Parsing,
            description: None,
            columns: 0,
            current: None,
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
            values,
        }))
    }

    pub async fn discard_rows(mut self) -> Result<(), Error> {
        while self.next().await?.is_some() {}

        Ok(())
    }

    pub async fn decode_column<T: for<'r> value::Decode<'r>>(
        mut self,
        reference: impl Into<ColumnReference<'_>>,
    ) -> Result<Decoder<'s, impl FnMut(Row<'_>) -> Result<T, DecodeError>>, Error> {
        let index = self.describe().await?.column::<T>(reference)?.index;

        Ok(self.decode(move |row| row.decode_at(index)))
    }

    pub fn decode<T, F>(self, decode: F) -> Decoder<'s, F>
    where
        F: FnMut(Row<'_>) -> Result<T, DecodeError>,
    {
        Decoder {
            reader: self,
            decode,
        }
    }

    #[must_use]
    pub const fn outcome(&self) -> Option<&Outcome> {
        match &self.state {
            State::Completed(outcome) | State::Done(Some(outcome)) => Some(outcome),
            _ => None,
        }
    }

    async fn step(&mut self) -> Result<Step, Error> {
        if matches!(self.state, State::Done(_)) {
            return Ok(Step::Done);
        }

        if self.session.unfinished {
            self.session.read_to_ready().await?;
            self.session.unfinished = false;
        }

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
            backend::Message::CommandComplete(_) => Event::Complete,
            backend::Message::EmptyQueryResponse => Event::Empty,
            backend::Message::ReadyForQuery(_) => Event::Ready,
            backend::Message::ErrorResponse(error) => Event::Rejected {
                severity: error.diagnostic.severity(),
                code: error.diagnostic.code(),
                message: error.diagnostic.message().to_owned(),
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
            (
                _,
                Event::Rejected {
                    severity,
                    code,
                    message,
                },
            ) => (
                Error::Server {
                    severity,
                    code,
                    message,
                },
                severity.ends_session(),
            ),
            (_, event) => {
                if self.session.mode == Mode::CopyIn {
                    self.session.refuse_copy_in();
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

pub struct Decoder<'s, F> {
    reader: Reader<'s>,
    decode: F,
}

impl<T, F> Decoder<'_, F>
where
    F: FnMut(Row<'_>) -> Result<T, DecodeError>,
{
    pub async fn next(&mut self) -> Result<Option<T>, Error> {
        match self.reader.next().await? {
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

        while self.reader.next().await?.is_some() {
            actual = actual.saturating_add(1);
        }

        if actual == 1 {
            Ok(Some(value))
        } else {
            Err(Error::TooManyRows { actual })
        }
    }
}

impl Drop for Reader<'_> {
    fn drop(&mut self) {
        if !matches!(self.state, State::Done(_)) {
            self.session.unfinished = true;
        }
    }
}
