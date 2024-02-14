use crate::{
    error::Result,
    query::Query,
    schema::Schema,
    value::{Map, Value},
};
use async_trait::async_trait;
use std::ops::Deref;

#[cfg(feature = "serde")]
pub mod serde_adapter;

/// A value that can initialize a new [`Adapter`]. Used to dynamically construct adapters from an
/// adapter impl registry based on the given database schema.
#[async_trait]
pub trait Init: Send + Sync {
    /// Initialize a new adapter impl.
    async fn init_adapter(&self, config: &Map) -> Result<Box<dyn Adapter>>;
}

#[async_trait]
pub trait Adapter: Send + Sync {
    // NIT: I may regret MutRef here. Owning and returning may be better long term to not deal
    // with lifecycle annoyances. We'll see.
    async fn read<'a>(&self, rows_buf: &mut Vec<Value>, query: Query<'a>) -> Result<()>;

    // TODO: impl.
    // async fn create();
    // async fn update();
    // async fn delete();

    // NIT: Probably better to return a paths of what is mutable?
    async fn mutable(&self) -> bool {
        false
    }
    async fn schema(&self) -> Option<Schema> {
        None
    }
}

#[async_trait]
impl Adapter for Box<dyn Adapter> {
    async fn read<'a>(&self, rows_buf: &mut Vec<Value>, query: Query<'a>) -> Result<()> {
        self.deref().read(rows_buf, query).await
    }
}

type _EnsureObjectSafeInit = Box<dyn Init>;
type _EnsureObjectSafeAdapter = Box<dyn Adapter>;

// #[test]
// fn dyn_compat() {
//     let _: Box<dyn AdapterInit> = Box::new();
// }
