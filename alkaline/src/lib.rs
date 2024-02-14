pub mod error;
pub mod value;
pub mod schema {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Schema {}
}
pub mod adapter;
pub mod database;
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
