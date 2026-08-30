use super::{CopyDataError, ReceiveError, ServerError, Session, statement};
use crate::protocol::{self, backend};

#[derive(Debug)]
pub struct Run<'s> {
    session: &'s mut Session,
    ended: bool,
}

impl<'s> Run<'s> {
    pub(super) const fn new(session: &'s mut Session) -> Self {
        Self {
            session,
            ended: false,
        }
    }
}

impl Run<'_> {
    pub async fn write(&mut self, data: &[u8]) -> Result<(), statement::Error> {
        self.session
            .send_copy_data(data)
            .await
            .map_err(|error| match error {
                CopyDataError::Queue(error) => statement::Error::Queue(error),
                CopyDataError::Send(error) => statement::Error::Send(error),
            })
    }

    pub async fn finish(mut self) -> Result<u64, statement::Error> {
        self.session.queue_copy_done()?;
        self.ended = true;

        let mut rows = None;
        let mut error = None;

        loop {
            let frame = self.session.receive().await?;

            let failure = match backend::Message::decode(&frame).map_err(ReceiveError::Decode)? {
                backend::Message::ReadyForQuery(_) => break,
                backend::Message::CommandComplete(backend::CommandComplete::Copy {
                    rows: copied,
                }) => {
                    rows = Some(copied);
                    None
                }
                backend::Message::ErrorResponse(response) => {
                    let failure = statement::Error::Server(ServerError(frame.clone()));

                    if response.diagnostic.severity().ends_session() {
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

        self.session.unfinished = false;

        match error {
            Some(error) => Err(error),
            None => Ok(rows.expect("a completed COPY reports its row count")),
        }
    }

    pub async fn abort(mut self, reason: &protocol::String<'_>) -> Result<(), statement::Error> {
        self.session.queue_copy_fail(reason)?;
        self.ended = true;

        self.session.read_to_ready().await?;
        self.session.unfinished = false;

        Ok(())
    }
}

impl Drop for Run<'_> {
    fn drop(&mut self) {
        const ABANDONED: protocol::String<'static> =
            protocol::String::from_static_or_panic("COPY abandoned by the client");

        if !self.ended {
            self.session
                .queue_copy_fail(&ABANDONED)
                .expect("a COPY that has not ended is still in CopyIn mode");
        }
    }
}
