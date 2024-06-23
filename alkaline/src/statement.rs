use self::query::Query;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum Statement {
    Create(Create),
    Query(Query),
}
impl From<Query> for Statement {
    fn from(q: Query) -> Self {
        Self::Query(q)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum Create {
    Database { name: String },
}

pub mod query {
    use crate::value::Value;
    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};

    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone)]
    pub struct Query {
        from: QueryFrom,
        selected_cols: Vec<String>,
    }
    impl Query {
        /// Construct a Query `from <name> [args]`.
        pub fn new(name: impl Into<String>, args: impl Into<Value>) -> Self {
            let source = QueryFrom {
                name: name.into(),
                args: args.into(),
            };
            Self {
                from: source,
                selected_cols: Default::default(),
            }
        }
        pub fn select<S: Into<String>>(mut self, cols: impl IntoIterator<Item = S>) -> Self {
            self.selected_cols = cols.into_iter().map(|s| s.into()).collect();
            self
        }
    }
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone)]
    struct QueryFrom {
        name: String,
        args: Value,
    }
}

#[cfg(test)]
#[allow(unused)]
#[tokio::test]
async fn api_design() {
    use crate::value::Value;

    let mut alk = crate::client::Alkaline::test();
    // let _ = alk.statement(Query::new("fake", Value::None)).await;
}
