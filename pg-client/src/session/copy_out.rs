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
pub struct Run<S: DerefMut<Target = Session>> {
    session: S,
    response: backend::CopyResponse,
    state: State,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Streaming,
    Drained,
    Finished,
}

impl<S: DerefMut<Target = Session>> Run<S> {
    pub async fn start(mut session: S, sql: &crate::Sql<'_>) -> Result<Self, statement::Error> {
        session
            .drain_unfinished(&crate::trace::Span::current())
            .await?;

        session.queue_query(sql)?;
        session.unfinished = true;

        let mut error = None;

        loop {
            let frame = session.receive().await?;

            if session.mode == Mode::CopyIn {
                session.refuse_copy_in()?;
            }

            let failure = match backend::Message::decode(&frame).map_err(ReceiveError::Decode)? {
                backend::Message::CopyOutResponse(response) if error.is_none() => {
                    return Ok(Self {
                        session,
                        response,
                        state: State::Streaming,
                    });
                }
                backend::Message::ReadyForQuery(_) => {
                    session.unfinished = false;

                    return Err(error.unwrap_or(statement::Error::Unexpected(frame)));
                }
                backend::Message::ErrorResponse(response) => {
                    let failure = statement::Error::Server(ServerError(frame.clone()));

                    if response.diagnostic.severity().ends_session() {
                        session.unfinished = false;

                        return Err(error.unwrap_or(failure));
                    }

                    Some(failure)
                }
                backend::Message::NoticeResponse(_) | backend::Message::ParameterStatus(_) => None,
                _ => Some(statement::Error::Unexpected(frame.clone())),
            };

            if error.is_none() {
                error = failure;
            }
        }
    }

    #[must_use]
    pub const fn response(&self) -> &backend::CopyResponse {
        &self.response
    }

    pub async fn next(&mut self) -> Result<Option<Data>, statement::Error> {
        if self.state != State::Streaming {
            return Ok(None);
        }

        loop {
            let frame = self.session.receive().await?;

            match backend::Message::decode(&frame).map_err(ReceiveError::Decode)? {
                backend::Message::CopyData(_) => return Ok(Some(Data(frame))),
                backend::Message::CopyDone => {
                    self.state = State::Drained;

                    return Ok(None);
                }
                backend::Message::NoticeResponse(_) | backend::Message::ParameterStatus(_) => {}
                backend::Message::ErrorResponse(response) => {
                    return Err(self
                        .fail(
                            statement::Error::Server(ServerError(frame.clone())),
                            response.diagnostic.severity().ends_session(),
                        )
                        .await);
                }
                _ => return Err(self.fail(statement::Error::Unexpected(frame), false).await),
            }
        }
    }

    // Reads whatever data is left, then the completion, and returns the row
    // count the server reports.
    pub async fn finish(mut self) -> Result<u64, statement::Error> {
        while self.next().await?.is_some() {}

        let mut rows = None;
        let mut error = None;

        loop {
            let frame = self.session.receive().await?;

            let failure = match backend::Message::decode(&frame).map_err(ReceiveError::Decode)? {
                backend::Message::ReadyForQuery(_) => {
                    self.state = State::Finished;
                    self.session.unfinished = false;

                    return match (error, rows) {
                        (Some(error), _) => Err(error),
                        (None, Some(rows)) => Ok(rows),
                        (None, None) => Err(statement::Error::Unexpected(frame)),
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
                        self.state = State::Finished;
                        self.session.unfinished = false;

                        return Err(error.unwrap_or(failure));
                    }

                    Some(failure)
                }
                backend::Message::NoticeResponse(_) | backend::Message::ParameterStatus(_) => None,
                _ => Some(statement::Error::Unexpected(frame.clone())),
            };

            if error.is_none() {
                error = failure;
            }
        }
    }

    async fn fail(&mut self, error: statement::Error, ends_session: bool) -> statement::Error {
        self.state = State::Finished;

        if ends_session {
            self.session.unfinished = false;

            return error;
        }

        match self.session.read_to_ready().await {
            Ok(()) => {
                self.session.unfinished = false;

                error
            }
            Err(receive) => statement::Error::Receive(receive),
        }
    }
}

impl<S: DerefMut<Target = Session>> Drop for Run<S> {
    fn drop(&mut self) {
        // Left to the next statement to drain; a client cannot stop a COPY TO
        // short of cancelling it, so that reads whatever output remains.
        if self.state != State::Finished {
            self.session.unfinished = true;
        }
    }
}
