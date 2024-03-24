use crate::pattern::{Bind, Pattern};
use compact_str::CompactString;
use std::{cell::RefCell, collections::HashMap, fmt};

pub type Field = CompactString;
pub type Value = CompactString;
pub type ComrakNode<'a> = comrak::arena_tree::Node<'a, RefCell<comrak::nodes::Ast>>;

/// The output of a [`Pattern`](crate::pattern::Pattern) onto a
/// [`Document`](crate::document::Document).
pub struct Structure<'a> {
    // TODO: Think i need to add an identical enum as `Pattern` but with the associated nodes.
    // Otherwise this can't directly create the markdown ast nodes.
    //
    // Alternatively, the combination of the pattern + struct could handle creation. Bit awkward
    // tho.
    _fields: HashMap<Field, Value>,
    node: Node<'a>,
}
impl<'a> Structure<'a> {
    pub fn new(pattern: &Pattern, node: &'a ComrakNode<'a>) -> (Option<Self>, &'a ComrakNode<'a>) {
        dbg!(pattern, node);
        todo!()
    }
}
impl fmt::Debug for Structure<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Structure").finish()
    }
}
#[derive(Debug, Default)]
pub enum Node<'a> {
    /// The start of the document.
    #[default]
    Start,
    Node(NodeBody<'a>),
}
#[derive(Debug)]
pub enum NodeBody<'a> {
    Header {
        header: ComrakNode<'a>,
        // NIT: This could probably be an optimized value, if Structure is going to store more
        // efficient refs?
        bind: Bind,
        inner: Option<Box<NodeBody<'a>>>,
    },
    /// A glob, matching any markdown nodes until the next structure.
    Glob { next: Box<NodeBody<'a>> },
    /// The end of the document.
    End,
}
