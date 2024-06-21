use crate::{
    adapter,
    error::{Error, Result},
    statement::Statement,
};
use async_trait::async_trait;
use std::ops::{Deref, DerefMut};

// pub mod server {}
// pub mod conn {}
pub mod local {
    use super::{Connection, Registry};
    use crate::{adapter, error::Result};
    use async_trait::async_trait;

    pub struct Local<S> {
        #[allow(unused)]
        storage: S,
    }
    impl<S> Local<S> {
        pub fn new(storage: S) -> Self {
            Self { storage }
        }
    }
    #[cfg(test)]
    impl Local<crate::storage::memory::MemoryDb> {
        pub fn test() -> Self {
            #[allow(clippy::default_constructed_unit_structs)]
            Self::new(crate::storage::memory::MemoryDb::default())
        }
    }
    #[async_trait]
    impl<S> Registry for Local<S> {
        fn register<T: adapter::Init>(&mut self) {
            todo!()
        }
    }
    #[async_trait]
    impl<S> Connection for Local<S>
    where
        S: Send + Sync,
    {
        async fn databases(&self) -> Result<Vec<String>> {
            todo!()
        }
    }
}

#[async_trait]
pub trait Registry {
    fn register<T: adapter::Init>(&mut self);
}
impl<R> Registry for Box<R>
where
    R: Registry,
{
    fn register<T: adapter::Init>(&mut self) {
        self.deref_mut().register::<T>()
    }
}

/// The primary behavior impl of local or remote alkaline impls.
///
/// This impl is largely stateless,
#[async_trait]
pub trait Connection: Send + Sync {
    async fn databases(&self) -> Result<Vec<String>>;
}
#[async_trait]
impl<T> Connection for Box<T>
where
    T: Connection,
{
    async fn databases(&self) -> Result<Vec<String>> {
        self.deref().databases().await
    }
}

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
impl Alkaline<local::Local<crate::storage::memory::MemoryDb>> {
    pub fn test() -> Self {
        Self::new(local::Local::test())
    }
}

#[async_trait]
impl<C> Registry for Alkaline<C>
where
    C: Registry,
{
    fn register<T: adapter::Init>(&mut self) {
        self.conn.register::<T>()
    }
}
