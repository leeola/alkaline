use crate::{
    error::{AdapterReadError, Result},
    query::Query,
    value::{de::from_value, ser::to_value, Value},
};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use tokio_stream::{Stream, StreamExt};

#[async_trait]
pub trait SerdeInit: Send + Sync {
    type Adapter: SerdeAdapter;
    type Config: Serialize + DeserializeOwned + Send + Sync;
    async fn init_adapter(&self, config: Self::Config) -> Result<Self::Adapter>;
}
#[async_trait]
pub trait SerdeAdapter: Send + Sync + 'static {
    type Read: Serialize + DeserializeOwned + Send + Sync;
    fn read(&self, query: Query) -> impl Stream<Item = Result<Self::Read, AdapterReadError>>;
}

/// A provider of [`Init`](super::Init) for the [`SerdeInit`] trait, where the inner `T`
/// has it's associated row types buffered to bridge between [`Value`] and and Serde
/// (de)Serializable row types.
pub struct Init<I: SerdeInit>(pub I);
#[async_trait]
impl<I, A, R> super::Init for Init<I>
where
    I: SerdeInit<Adapter = A>,
    A: SerdeAdapter<Read = R> + Sync,
    R: Serialize + Send + Sync + 'static,
{
    async fn init_adapter(&self, _config: Value) -> Result<Box<dyn super::Adapter>> {
        // NIT: Pretend the 5 value is a config.. lol, until Map gets added to Value.
        // let config = from_value::<T::Config>(config.clone().into());
        let config = from_value::<I::Config>(5.into()).unwrap();
        let adapter = self.0.init_adapter(config).await?;
        Ok(Box::new(Adapter {
            adapter,
            read_buf: Default::default(),
        }))
    }
}

/// A provider of [`Adapter`](super::Adapter) for the [`SerdeAdapter`] trait, where the inner `T`
/// has it's associated row types buffered to bridge between [`Value`] and and Serde
/// (de)Serializable row types.
#[derive(Debug)]
pub struct Adapter<T, R> {
    #[allow(unused)]
    adapter: T,
    #[allow(unused)]
    read_buf: Vec<R>,
}
#[async_trait]
impl<T, R> super::Adapter for Adapter<T, R>
where
    T: SerdeAdapter<Read = R> + Sync,
    R: Serialize + Send + Sync + 'static,
{
    fn read(&self, query: Query) -> Box<dyn Stream<Item = Result<Value, AdapterReadError>> + '_> {
        Box::new(
            self.adapter
                .read(query)
                .map(|res| res.map(|r| to_value(r).unwrap())),
        )
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use serde::{Deserialize, Serialize};

    pub struct TestInit;
    #[async_trait]
    impl SerdeInit for TestInit {
        type Adapter = TestAdapter;
        type Config = ();
        async fn init_adapter(&self, _: Self::Config) -> Result<Self::Adapter> {
            Ok(TestAdapter)
        }
    }
    pub struct TestAdapter;
    #[async_trait]
    impl SerdeAdapter for TestAdapter {
        type Read = SomeRow;
        fn read(&self, _query: Query) -> impl Stream<Item = Result<Self::Read, AdapterReadError>> {
            tokio_stream::iter(vec![
                SomeRow {
                    name: "foo".into(),
                    age: 200,
                },
                SomeRow {
                    name: "bar".into(),
                    age: 400,
                },
            ])
            .map(Ok)
        }
    }
    #[derive(Serialize, Deserialize)]
    pub struct SomeRow {
        pub name: String,
        pub age: i64,
    }

    async fn _binding_sanity_check() {
        let init: Box<dyn crate::adapter::Init> = Box::new(Init(TestInit));
        let _: Result<Box<dyn crate::adapter::Adapter>, _> =
            init.init_adapter(Default::default()).await;
    }
}
