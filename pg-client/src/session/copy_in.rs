use core::ops::DerefMut;

use super::{CopyDataError, Mode, ReceiveError, ServerError, Session, statement};
use crate::protocol::{self, backend};

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
            state: State::Queued,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Queued,
    Copying,
    Done,
}

#[derive(Debug)]
pub struct Run<S: DerefMut<Target = Session>> {
    session: S,
    state: State,
}

impl<S: DerefMut<Target = Session>> Run<S> {
    pub async fn write(&mut self, data: &[u8]) -> Result<(), statement::Error> {
        self.start().await?;

        self.session
            .send_copy_data(data)
            .await
            .map_err(|error| match error {
                CopyDataError::Queue(error) => statement::Error::Queue(error),
                CopyDataError::Send(error) => statement::Error::Send(error),
            })
    }

    // The row count the server reports; none when the copy never ran.
    pub async fn finish(mut self) -> Result<Option<u64>, statement::Error> {
        self.start().await?;

        if self.state == State::Done {
            return Ok(None);
        }

        self.session.queue_copy_done()?;

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

    pub async fn abort(mut self, reason: &protocol::String<'_>) -> Result<(), statement::Error> {
        self.start().await?;

        if self.state == State::Done {
            return Ok(());
        }

        self.session.queue_copy_fail(reason)?;
        self.session.read_to_ready().await?;
        self.state = State::Done;

        Ok(())
    }

    // Reads up to the server's go-ahead, the CopyInResponse.
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
                if error.is_none() {
                    self.state = State::Copying;

                    return Ok(());
                }

                self.session.refuse_copy_in()?;

                continue;
            }

            let failure = match backend::Message::decode(&frame).map_err(ReceiveError::Decode)? {
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
}

// An unfinished copy-in keeps its snapshot, and with it the server's xmin
// and its locks, until something ends it. A CopyFail queued here would only
// go out on the session's next use, so the connection is closed instead.
impl<S: DerefMut<Target = Session>> Drop for Run<S> {
    fn drop(&mut self) {
        if self.state != State::Done {
            self.session.connection.close();
        }
    }
}
