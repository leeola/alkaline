use super::{ephemeral_reference::EphemeralReferenceBackend, Connection, Registry};
use crate::{adapter, error::Result};
use async_trait::async_trait;

/// A wrapper around [`EphemeralReferenceBackend`] which serializes and deserializes the state
/// to provide basic persistence.
///
/// This impl is highly inefficient, by design, and merely provides some persistence to the
/// underlying reference impl.
pub struct PersistentReferenceBackend {
    backend: EphemeralReferenceBackend,
}
impl PersistentReferenceBackend {
    pub async fn new(backend: EphemeralReferenceBackend) -> Result<Self> {
        Ok(Self { backend })
    }
}
#[async_trait]
impl Connection for PersistentReferenceBackend {
    async fn databases(&self) -> Result<Vec<String>> {
        self.backend.databases().await
    }
}
impl Registry for PersistentReferenceBackend {
    fn register<T: adapter::Init>(&mut self) {
        self.backend.register::<T>()
    }
}
