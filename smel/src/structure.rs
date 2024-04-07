use crate::pattern::{Bind, Pattern, PatternBody};
use compact_str::CompactString;
use comrak::nodes::NodeValue;
use std::{cell::RefCell, fmt};

pub type Field = CompactString;
pub type Value = CompactString;
pub type ComrakNode<'a> = &'a comrak::arena_tree::Node<'a, RefCell<comrak::nodes::Ast>>;
type StructureResult<'a> = (Option<Structure<'a>>, ComrakNode<'a>);

/// The output of a [`Pattern`](crate::pattern::Pattern) onto a
/// [`Document`](crate::document::Document).
pub struct Structure<'a> {
    // TODO: Think i need to add an identical enum as `Pattern` but with the associated nodes.
    // Otherwise this can't directly create the markdown ast nodes.
    //
    // Alternatively, the combination of the pattern + struct could handle creation. Bit awkward
    // tho.
    // _fields: HashMap<Field, Value>,
    node: Node<'a>,
}
impl<'a> Structure<'a> {
    pub fn new(pattern: &Pattern, md_node: ComrakNode<'a>) -> StructureResult<'a> {
        dbg!(pattern);
        match pattern {
            Pattern::Start(body) => Self::start(body, md_node),
            Pattern::Body(body) => Self::body(body, md_node),
        }
    }
    fn start(body: &PatternBody, cnode: ComrakNode<'a>) -> StructureResult<'a> {
        if let NodeValue::Document = cnode.data.borrow().value {
            // NIT: Assuming the rest of the nodes consume or always make progress, this func may
            // need to make progress.
            return (None, cnode);
        }
        Self::body(body, cnode)
    }
    fn body(body: &PatternBody, cnode: ComrakNode) -> StructureResult<'a> {
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
