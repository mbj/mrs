use core::ops::DerefMut;

use super::{Mode, ReceiveError, ServerError, Session, statement};
use crate::protocol::{self, backend};

// One CopyData message as the server sent it. It shares the receive buffer's
// storage, so holding on to it costs a reference count, not a copy.
#[derive(Clone, Debug)]
pub struct Data(protocol::Frame);

impl Data {
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.0.body().as_slice()
    }
}

#[derive(Debug)]
pub struct Builder<'q, S: DerefMut<Target = Session>> {
    session: S,
    sql: crate::Sql<'q>,
}

impl<'q, S: DerefMut<Target = Session>> Builder<'q, S> {
    pub fn new(session: S, sql: &crate::Sql<'q>) -> Self {
        Self {
            session,
            sql: sql.clone(),
        }
    }

    pub fn run(mut self) -> Result<Run<S>, statement::Error> {
        self.session.queue_query(&self.sql)?;

        Ok(Run {
            session: self.session,
            response: None,
            state: State::Queued,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Queued,
    Streaming,
    // CopyDone is in, the completion is not.
    Completing,
    Done,
}

#[derive(Debug)]
pub struct Run<S: DerefMut<Target = Session>> {
    session: S,
    response: Option<backend::CopyResponse>,
    state: State,
}

impl<S: DerefMut<Target = Session>> Run<S> {
    // None once the run ended without the copy starting.
    pub async fn response(&mut self) -> Result<Option<&backend::CopyResponse>, statement::Error> {
        self.start().await?;

        Ok(self.response.as_ref())
    }

    pub async fn next(&mut self) -> Result<Option<Data>, statement::Error> {
        self.start().await?;

        if self.state != State::Streaming {
            return Ok(None);
        }

        loop {
            let frame = self.session.receive().await?;

            match backend::Message::decode(&frame).map_err(ReceiveError::Decode)? {
                backend::Message::CopyData(_) => return Ok(Some(Data(frame))),
                backend::Message::CopyDone => {
                    self.state = State::Completing;

                    return Ok(None);
                }
                backend::Message::NoticeResponse(_)
                | backend::Message::NotificationResponse(_)
                | backend::Message::ParameterStatus(_) => {}
                backend::Message::ErrorResponse(response) => {
                    return Err(self
                        .fail(
                            statement::Error::Server(ServerError(frame.clone())),
                            response.diagnostic.severity().ends_session(),
                        )
                        .await);
                }
                _ => {
                    return Err(self
                        .session
                        .violation(super::Violation::UnexpectedMessage(frame))
                        .into());
                }
            }
        }
    }

    // Reads whatever data is left, then the completion, and returns the row
    // count the server reports; none when the copy never ran.
    pub async fn finish(mut self) -> Result<Option<u64>, statement::Error> {
        while self.next().await?.is_some() {}

        if self.state == State::Done {
            return Ok(None);
        }

        let mut rows = None;
        let mut error = None;

        loop {
            let frame = self.session.receive().await?;

            let failure = match backend::Message::decode(&frame).map_err(ReceiveError::Decode)? {
                backend::Message::ReadyForQuery(_) => {
                    self.state = State::Done;

                    return match (error, rows) {
                        (Some(error), _) => Err(error),
                        (None, Some(rows)) => Ok(Some(rows)),
                        (None, None) => Err(self
                            .session
                            .violation(super::Violation::UnexpectedMessage(frame))
                            .into()),
                    };
                }
                backend::Message::CommandComplete(backend::CommandComplete::Copy {
                    rows: copied,
                }) => {
                    rows = Some(copied);
                    None
                }
                backend::Message::ErrorResponse(response) => {
                    let failure = statement::Error::Server(ServerError(frame.clone()));

                    if response.diagnostic.severity().ends_session() {
                        return Err(error.unwrap_or(failure));
                    }

                    Some(failure)
                }
                backend::Message::NoticeResponse(_)
                | backend::Message::NotificationResponse(_)
                | backend::Message::ParameterStatus(_) => None,
                message if super::answers_a_query(&message) => {
                    Some(statement::Error::WrongStatement(frame.clone()))
                }
                _ => {
                    return Err(self
                        .session
                        .violation(super::Violation::UnexpectedMessage(frame))
                        .into());
                }
            };

            if error.is_none() {
                error = failure;
            }
        }
    }

    // Reads up to the CopyOutResponse.
    async fn start(&mut self) -> Result<(), statement::Error> {
        if self.state != State::Queued {
            return Ok(());
        }

        self.session
            .drain_abandoned(&crate::trace::Span::current())
            .await?;

        let mut error = None;

        loop {
            let frame = self.session.receive().await?;

            if self.session.mode == Mode::CopyIn {
                self.session.refuse_copy_in()?;
            }

            let failure = match backend::Message::decode(&frame).map_err(ReceiveError::Decode)? {
                backend::Message::CopyOutResponse(response) if error.is_none() => {
                    self.response = Some(response);
                    self.state = State::Streaming;

                    return Ok(());
                }
                backend::Message::ReadyForQuery(_) => {
                    self.state = State::Done;

                    return Err(match error {
                        Some(error) => error,
                        None => self
                            .session
                            .violation(super::Violation::UnexpectedMessage(frame))
                            .into(),
                    });
                }
                backend::Message::ErrorResponse(response) => {
                    let failure = statement::Error::Server(ServerError(frame.clone()));

                    if response.diagnostic.severity().ends_session() {
                        self.state = State::Done;

                        return Err(error.unwrap_or(failure));
                    }

                    Some(failure)
                }
                backend::Message::NoticeResponse(_)
                | backend::Message::NotificationResponse(_)
                | backend::Message::ParameterStatus(_) => None,
                message if super::answers_a_query(&message) => {
                    Some(statement::Error::WrongStatement(frame.clone()))
                }
                _ => {
                    return Err(self
                        .session
                        .violation(super::Violation::UnexpectedMessage(frame))
                        .into());
                }
            };

            if error.is_none() {
                error = failure;
            }
        }
    }

    async fn fail(&mut self, error: statement::Error, ends_session: bool) -> statement::Error {
        self.state = State::Done;

        if ends_session {
            return error;
        }

        match self.session.read_to_ready().await {
            Ok(()) => error,
            Err(receive) => statement::Error::Receive(receive),
        }
    }
}
