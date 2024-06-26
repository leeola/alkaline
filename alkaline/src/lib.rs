pub mod error;
pub mod value;
pub mod schema {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Schema {}
}
pub mod backend;
pub use client::Alkaline;
pub mod adapter;
pub mod client;
pub mod database;
pub mod statement;
pub mod storage;
pub mod query {
    use crate::{filter::Filter, select::Select};

    #[derive(Debug, Clone)]
    pub struct Query {
        pub select: Select,
        pub filter: Filter,
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
