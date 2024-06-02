use async_trait::async_trait;

pub mod memory {
    use super::Storage;

    #[derive(Default)]
    pub struct Memory;
    impl Storage for Memory {}
}

#[async_trait]
pub trait Storage {}
type _EnsureObjectSafeStorage = Box<dyn Storage>;
