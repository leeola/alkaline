use crate::{
    error::Result,
    query::Query,
    schema::Schema,
    value::{Map, Value},
};
use async_trait::async_trait;

#[cfg(feature = "serde")]
pub mod serde_adapter;

#[async_trait]
pub trait Adapter: Sized {
    async fn init(config: &Map) -> Result<Self>;

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
