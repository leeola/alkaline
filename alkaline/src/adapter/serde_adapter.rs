use super::Adapter;
use crate::{
    error::Result,
    query::Query,
    value::{de::from_value, Map, Value},
};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug)]
pub struct SerdeAdapter<T, R> {
    adapter: T,
    read_buf: Vec<R>,
}
// TODO: Rename Ext, not sure what i wanna call this..
#[async_trait]
pub trait SerdeAdapterExt: Sized {
    type Config: Serialize + DeserializeOwned + Send + Sync;
    type Read: Serialize + DeserializeOwned + Send + Sync;
    async fn init(config: &Self::Config) -> Result<Self>;
    async fn read<'a>(&self, rows_buf: &mut Vec<Self::Read>, query: Query<'a>) -> Result<()>;
    // async fn update<'a>(&self, rows_buf: Vec<Self::Row>) -> Result<()>;
}
#[async_trait]
impl<T, R> Adapter for SerdeAdapter<T, R>
where
    T: SerdeAdapterExt<Read = R> + Sync,
    R: Sync,
{
    async fn init(_config: &Map) -> Result<Self> {
        // NIT: Pretend the 5 value is a config.. lol, until Map gets added to Value.
        // let config = from_value::<T::Config>(config.clone().into());
        let config = from_value::<T::Config>(5.into()).unwrap();
        let adapter = T::init(&config).await?;
        Ok(Self {
            adapter,
            read_buf: Default::default(),
        })
    }
    async fn read<'a>(&self, rows_buf: &mut Vec<Value>, query: Query<'a>) -> Result<()> {
        todo!()
    }
}
