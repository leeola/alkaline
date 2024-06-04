use async_trait::async_trait;

pub mod memory {
    use super::{DatabaseStorage, Persist};
    use async_trait::async_trait;

    #[derive(Default)]
    pub struct MemoryPersist {
        active_database: Option<String>,
    }
    #[async_trait]
    impl Persist for MemoryPersist {
        async fn active_database(&self) -> Option<String> {
            self.active_database.clone()
        }
    }

    #[derive(Default)]
    pub struct MemoryDb;
    #[async_trait]
    impl DatabaseStorage for MemoryDb {}
}

#[async_trait]
pub trait Persist {
    async fn active_database(&self) -> Option<String>;
}

#[async_trait]
pub trait DatabaseStorage {}
type _EnsureObjectSafeStorage = Box<dyn DatabaseStorage>;
