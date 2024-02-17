//! Stripped down input/output interface and implementations, focused on supporting FileSystem
//! and "odd WASM APIs" like we might see in Obsidian.md. Long term, this API is expected to
//! remain as small as possible and work for all target platforms, even at the cost of
//! clean/simple impls.
use alkaline::error::Result;
use async_trait::async_trait;
use tokio::io::AsyncRead;

pub mod mem {
    use super::MarkdownStore;
    use alkaline::error::Result;
    use async_trait::async_trait;
    use std::collections::BTreeMap;
    use tokio::io::{AsyncRead, AsyncReadExt};

    /// A [`MarkdownStore`] impl which is not persistent, primarily focused on tests.
    #[derive(Debug, Default, Clone)]
    pub struct MemStore(BTreeMap<String, String>);

    #[async_trait]
    impl MarkdownStore for MemStore {
        type GetReader = tokio::io::Empty;
        async fn list(&self, prefix: &str) -> Result<Vec<String>> {
            Ok(self
                .0
                .keys()
                .filter(|k| k.starts_with(prefix))
                .cloned()
                .collect())
        }
        async fn put(
            &self,
            _key: String,
            mut read: impl AsyncRead + Send + Unpin,
        ) -> Result<usize> {
            let mut s_buf = String::new();
            let _foo = read.read_to_string(&mut s_buf).await;
            todo!()
        }
        async fn get(&self, _key: String) -> Result<Self::GetReader> {
            Ok(tokio::io::empty())
        }
    }
}

#[async_trait]
pub trait MarkdownStore {
    type GetReader: AsyncRead + Send + Unpin;
    async fn list(&self, prefix: &str) -> Result<Vec<String>>;
    async fn put(&self, key: String, read: impl AsyncRead + Send + Unpin) -> Result<usize>;
    async fn get(&self, key: String) -> Result<Self::GetReader>;
}
