use crate::storage::Storage;

pub struct Alkaline {
    #[allow(unused)]
    storage: Box<dyn Storage>,
}
impl Alkaline {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self { storage }
    }
}
