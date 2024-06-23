use crate::{
    adapter,
    backend::{Connection, Registry},
    error::{Error, Result},
    statement::Statement,
};

/// A stateful client over an underlying alkaline [`Connection`], tracking active database, partial
/// adapter, etc.
pub struct Alkaline<C = Box<dyn Connection>> {
    #[allow(unused)]
    active_database: Option<String>,
    conn: C,
}
impl<C> Alkaline<C> {
    pub fn new(conn: C) -> Self {
        Self {
            conn,
            active_database: Default::default(),
        }
    }
    pub async fn statement(&mut self, _stmt: impl Into<Statement>) -> Result<(), Error> {
        todo!()
    }
    pub async fn query(&mut self, _query: impl Into<Statement>) -> Result<(), Error> {
        todo!()
    }
}
impl<C> Alkaline<C>
where
    C: Connection,
{
    pub async fn databases(&self) -> Result<Vec<String>> {
        self.conn.databases().await
    }
}
#[cfg(test)]
impl Alkaline<crate::backend::ephemeral_reference::EphemeralReferenceBackend> {
    pub fn test() -> Self {
        Self::new(crate::backend::ephemeral_reference::EphemeralReferenceBackend::new())
    }
}

impl<C> Registry for Alkaline<C>
where
    C: Registry,
{
    fn register<T: adapter::Init>(&mut self) {
        self.conn.register::<T>()
    }
}
