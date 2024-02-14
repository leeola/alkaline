use crate::adapter::{Adapter, Init};
use std::collections::HashMap;

pub struct Database {
    adapters: HashMap<String, Box<dyn Adapter>>,
}
impl Database {
    pub fn builder() -> DatabaseBuilder {
        DatabaseBuilder::default()
    }
}

#[derive(Default)]
pub struct DatabaseBuilder {
    inits: HashMap<&'static str, Box<dyn Init>>,
    adapters: HashMap<String, Box<dyn Adapter>>,
}
impl DatabaseBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push_adapter_init(&mut self, init: Box<dyn Init>) {
        todo!()
    }
    // pub fn push_adapter()
}
