use crate::{
    pattern::Pattern,
    structure::{ComrakNode, Structure},
};
use comrak::ComrakOptions;
use std::iter;

/// A memory Arena for the internal markdown parser, [`comrak`].
///
/// Construct with `Arena::new()`.
pub type Arena<'a> = comrak::Arena<ComrakNode<'a>>;

#[allow(unused)]
pub struct Document<'a> {
    structures: Vec<Structure<'a>>,
}
impl<'a> Document<'a> {
    pub fn new(arena: &'a Arena<'a>, pattern: Pattern, md: &str) -> Self {
        let mut node = comrak::parse_document(arena, md, &ComrakOptions::default());
        // NIT: Not sure how the AST parse will handle being left in a deep node.
        let structures = iter::from_fn(move || {
            let (structure, remaining) = Structure::new(&pattern, node);
            node = remaining;
            structure
        })
        .collect::<Vec<_>>();
        Self { structures }
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
        let _doc = Document::new(
            &arena,
            Pattern::default(),
            "\
                # foo\n\
                bar baz\n\
                ## Bang\n\
            ",
        );
    }
}
