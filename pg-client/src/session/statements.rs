use core::ops::DerefMut;

use super::{Session, statement};

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
        })
    }
}

impl<'q, S> core::future::IntoFuture for Builder<'q, S>
where
    S: DerefMut<Target = Session> + Send + 'q,
{
    type Output = Result<(), statement::Error>;
    type IntoFuture =
        core::pin::Pin<Box<dyn core::future::Future<Output = Self::Output> + Send + 'q>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.run()?.finish().await })
    }
}

#[derive(Debug)]
pub struct Run<S: DerefMut<Target = Session>> {
    session: S,
}

impl<S: DerefMut<Target = Session>> Run<S> {
    // Reads the reply up to its ReadyForQuery.
    pub async fn finish(mut self) -> Result<(), statement::Error> {
        self.session.drain_abandoned().await?;

        self.session.read_statements_reply(|_| ()).await
    }
}
