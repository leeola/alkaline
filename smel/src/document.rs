use crate::{pattern::Pattern, value::Value};
use comrak::{arena_tree::Node, nodes::Ast};
use std::cell::RefCell;

/// A memory Arena for the internal markdown parser, [`comrak`].
///
/// Construct with `Arena::new()`.
pub type Arena<'a> = comrak::Arena<Node<'a, RefCell<Ast>>>;

pub struct Document {}
impl Document {
    pub fn new(arena: &Arena, pattern: Pattern, md: &str) -> Self {
        todo!()
    }
    // pub fn iter() -> impl Iterator<Item = &Value> {
    //     todo!()
    // }
    // pub fn iter_mut() -> impl Iterator<Item = &mut Value> {
    //     todo!()
    // }
    // pub fn insert(value: Value) {}
}
#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn poc() {
        let arena = Arena::new();
        let doc = Document::new(&arena, Pattern::default(), "");
    }
}
