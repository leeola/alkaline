use alkaline::{adapter::Adapter, error::Result, query::Query, value::Value};
use async_trait::async_trait;

pub mod io;

pub struct MarkdownAdapter;
#[async_trait]
impl Adapter for MarkdownAdapter {
    async fn read<'a>(&self, _rows_buf: &mut Vec<Value>, _query: Query<'a>) -> Result<()> {
        todo!()
    }
}
