pub mod error;
pub mod value;
pub mod schema {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Schema {}
}
pub mod adapter {
    use crate::{
        error::Result,
        query::Query,
        schema::Schema,
        value::{Map, Value},
    };
    use async_trait::async_trait;

    #[async_trait]
    pub trait Adapter: Sized {
        async fn init(config: &Map) -> Self;

        async fn read(&self, rows_buf: &mut Vec<Value>, query: &Query) -> Result<()>;

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
}
pub mod query {
    use crate::value::Value;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Query {
        Op { op: Op, value: Value },
    }
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Op {
        Eq,
    }
}
