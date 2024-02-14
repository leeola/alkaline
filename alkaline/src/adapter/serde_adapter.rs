use crate::{error::Result, query::Query};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug)]
pub struct SerdeAdapter {}
#[async_trait]
pub trait SerdeAdapterExt: Sized {
    type Row: Serialize + DeserializeOwned;
    async fn read<'a>(&self, rows_buf: &mut Vec<Self::Row>, query: Query<'a>) -> Result<()>;
    // async fn update<'a>(&self, rows_buf: Vec<Self::Row>) -> Result<()>;
}
