use crate::{
    error::{AdapterReadError, Result},
    query::Query,
    value::Value,
};
use async_trait::async_trait;
use std::ops::Deref;
use tokio_stream::Stream;

pub mod once_value;
#[cfg(feature = "serde")]
pub mod serde_adapter;

/// A value that can initialize a new [`Adapter`]. Used to dynamically construct adapters from an
/// adapter impl registry based on the given database schema.
#[async_trait]
pub trait Init: Send + Sync {
    /// Initialize a new adapter impl, returning the associated header information for this.
    async fn init_adapter(&self, config: Value) -> Result<Box<dyn Adapter>>;
}

// TODO: Return header/metadata about the name, capabilities, consistency of schema and etc of an
// Adapter.

#[async_trait]
pub trait Adapter: Send + Sync {
    // TODO: impl a mechanism to indicate if the query select, filter and ord were honored. It would
    // be nice to let them optionally not honor those values.
    fn read(&self, query: Query) -> Box<dyn Stream<Item = Result<Value, AdapterReadError>> + '_>;

    // TODO: impl.
    // async fn create();
    // async fn update();
    // async fn delete();
    // async fn schema()
}

#[async_trait]
impl Adapter for Box<dyn Adapter> {
    fn read(&self, query: Query) -> Box<dyn Stream<Item = Result<Value, AdapterReadError>> + '_> {
        self.deref().read(query)
    }
}

type _EnsureObjectSafeInit = Box<dyn Init>;
type _EnsureObjectSafeAdapter = Box<dyn Adapter>;

// #[test]
// fn dyn_compat() {
//     let _: Box<dyn AdapterInit> = Box::new();
// }
