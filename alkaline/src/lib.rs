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

    #[cfg(feature = "serde")]
    pub mod serde_adapter;

    #[async_trait]
    pub trait Adapter: Sized {
        async fn init(config: &Map) -> Self;

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
}
pub mod query {
    use crate::{filter::Filter, select::Select};

    #[derive(Debug)]
    pub struct Query<'a> {
        pub select: &'a Select,
        pub filter: &'a Filter,
    }
}
pub mod select {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Select {
        /// Select the whole value.
        Value,
        // /// Select
        // Indexes(Vec<(usize, Select)>),
        /// Select values corresponding to the given keys.
        Keys(Vec<(String, Select)>),
    }
}
pub mod filter {
    use crate::value::Value;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Filter {
        Op { op: Op, value: Value },
        Fields(Vec<(Value, Filter)>),
    }
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Op {
        Eq,
    }
}
