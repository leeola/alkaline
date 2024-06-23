use super::{Connection, Registry};
use crate::{adapter, error::Result};
use async_trait::async_trait;

/// A simplified direct backend impl without persistence, serving as an initial alkaline backend
/// reference impl.
///
/// The sole focus being a correct impl and being very understandable.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EphemeralReferenceBackend {}
impl EphemeralReferenceBackend {
    pub fn new() -> Self {
        Self::default()
    }
}
#[async_trait]
impl Connection for EphemeralReferenceBackend {
    async fn databases(&self) -> Result<Vec<String>> {
        todo!()
    }
}
impl Registry for EphemeralReferenceBackend {
    fn register<T: adapter::Init>(&mut self) {
        todo!()
    }
}
