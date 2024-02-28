//! A Structured Markdown Editing interface and (eventually) Language.

pub mod document;
pub mod pattern;
pub mod structure;
pub mod value;

pub type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[derive(Clone)]
pub struct Smel {}
impl Smel {
    pub fn new<S: Into<String>>(_s: S) -> Result<Self> {
        todo!()
    }
}
