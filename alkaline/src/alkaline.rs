use crate::storage::Storage;

pub struct Alkaline {
    #[allow(unused)]
    storage: Box<dyn Storage>,
}
impl Alkaline {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self { storage }
    }
    #[cfg(test)]
    pub fn test() -> Self {
        Self::new(Box::<crate::storage::memory::Memory>::default())
    }
}
