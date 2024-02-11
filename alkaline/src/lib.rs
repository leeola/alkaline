pub mod error {
    pub type Result<T, E = Error> = std::result::Result<T, E>;
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Other(anyhow::Error),
    }
}
pub mod value {
    /// Early limited dynamic value type.
    pub enum Value {
        Number(Number),
    }
    pub enum Number {
        Integer(i64),
        Float(f64),
    }
    pub struct Map;
}
pub mod schema {
    pub enum Schema {}
}
pub mod adapter {
    use crate::{
        error::Result,
        schema::Schema,
        value::{Map, Value},
    };
    use async_trait::async_trait;

    #[async_trait]
    pub trait Adapter: Sized {
        async fn init(config: &Map) -> Self;

        async fn read(&self, rows_buf: &mut Vec<Value>, query: &()) -> Result<()>;

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

    pub struct Query {
        selection: Value,
        filters: (),
    }
}
