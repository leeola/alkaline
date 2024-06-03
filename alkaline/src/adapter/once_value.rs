use super::{Adapter, Init};
use crate::{
    error::{AdapterReadError, Result},
    query::Query,
    value::{Map, Value},
};
use async_trait::async_trait;
use tokio_stream::Stream;

pub struct OnceValueInit;
#[async_trait]
impl Init for OnceValueInit {
    async fn init_adapter(&self, _config: &Map) -> Result<Box<dyn Adapter>> {
        // TODO: convert the map to a general value i suspect? Requiring a map seems restrictive
        // for a flexible adapter init.

        // faking a value until the config is converted.
        Ok(Box::new(OnceValue(0xBEEF.into())))
    }
}

pub struct OnceValue(pub Value);
#[async_trait]
impl Adapter for OnceValue {
    fn read(&self, _: Query) -> Box<dyn Stream<Item = Result<Value, AdapterReadError>> + '_> {
        Box::new(tokio_stream::once(Ok(self.0.clone())))
    }
}
