use crate::{
    pattern::Pattern,
    structure::{ComrakNode, Structure},
};

/// A memory Arena for the internal markdown parser, [`comrak`].
///
/// Construct with `Arena::new()`.
pub type Arena<'a> = comrak::Arena<ComrakNode<'a>>;

#[allow(unused)]
pub struct Document<'a> {
    structures: Vec<Structure<'a>>,
}
impl<'a> Document<'a> {
    pub fn new(_arena: &Arena<'a>, _pattern: Pattern, _md: &str) -> Self {
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
    // use super::*;
    #[test]
    fn poc() {
        // let arena = Arena::new();
        // let _doc = Document::new(&arena, Pattern::default(), "");
    }
}
