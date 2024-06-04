use crate::{
    adapter,
    storage::{memory::MemoryPersist, Persist},
};
use async_trait::async_trait;
use std::ops::DerefMut;

// pub mod server {}
// pub mod conn {}
pub mod local {
    use super::{Connection, Registry};
    use crate::{adapter, storage::DatabaseStorage};
    use async_trait::async_trait;

    pub struct Local {
        #[allow(unused)]
        storage: Box<dyn DatabaseStorage>,
    }
    impl Local {
        pub fn new(storage: Box<dyn DatabaseStorage>) -> Self {
            Self { storage }
        }
        #[cfg(test)]
        pub fn test() -> Self {
            Self::new(Box::<crate::storage::memory::MemoryDb>::default())
        }
        /// Register an adapter.
        pub fn register<T>(&mut self) {
            todo!()
        }
    }
    #[async_trait]
    impl Registry for Local {
        fn register<T: adapter::Init>(&mut self) {
            todo!()
        }
    }
    #[async_trait]
    impl Connection for Local {}
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

#[async_trait]
pub trait Connection {}
#[async_trait]
impl<T> Connection for Box<T> where T: Connection {}

pub struct Alkaline<C = Box<dyn Connection>, P = Box<dyn Persist>> {
    conn: C,
    persist: P,
}
impl<C> Alkaline<C> {
    pub fn new(conn: C) -> Self {
        Self {
            conn,
            persist: Box::<MemoryPersist>::default(),
        }
    }
}
#[cfg(test)]
impl Alkaline<local::Local, MemoryPersist> {
    pub fn test() -> Self {
        Self {
            conn: local::Local::test(),
            persist: MemoryPersist::default(),
        }
    }
}

#[async_trait]
impl<C, S> Registry for Alkaline<C, S>
where
    C: Registry,
{
    fn register<T: adapter::Init>(&mut self) {
        self.conn.register::<T>()
    }
}
