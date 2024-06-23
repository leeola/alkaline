//! Backend impls for the [`crate::Alkaline`] client where each backend impls the [`Connection`]
//! trait.
use crate::{adapter, error::Result};
use async_trait::async_trait;
use std::ops::{Deref, DerefMut};

// pub mod server {}
// pub mod conn {}
pub mod ephemeral_reference;
pub mod persistent_reference;

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

type _EnsureObjectSafeConnection = Box<dyn Connection>;

#[async_trait]
impl<T> Connection for Box<T>
where
    T: Connection,
{
    async fn databases(&self) -> Result<Vec<String>> {
        self.deref().databases().await
    }
}
