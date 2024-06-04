use super::{Adapter, Init};
use crate::{
    error::{AdapterReadError, Result},
    query::Query,
    value::Value,
};
use async_trait::async_trait;
use tokio_stream::Stream;

pub struct OnceValueInit;
#[async_trait]
impl Init for OnceValueInit {
    async fn init_adapter(&self, config: Value) -> Result<Box<dyn Adapter>> {
        Ok(Box::new(OnceValue(config)))
    }
}

pub struct OnceValue(pub Value);
#[async_trait]
impl Adapter for OnceValue {
    fn read(&self, _: Query) -> Box<dyn Stream<Item = Result<Value, AdapterReadError>> + '_> {
        Box::new(tokio_stream::once(Ok(self.0.clone())))
    }
}
