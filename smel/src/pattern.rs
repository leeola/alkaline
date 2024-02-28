/// An optional bind param for fields of a [`Structure`].
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bind {
    #[default]
    None,
    Name(String),
    OptionalName(String),
}

#[derive(Debug, Default)]
pub enum Pattern {
    /// The start of the document.
    #[default]
    Start,
    Node(PatternNode),
}
#[derive(Debug)]
pub enum PatternNode {
    Header {
        level: Option<u8>,
        bind: Bind,
        inner: Option<Box<PatternNode>>,
    },
    /// A glob, matching any markdown nodes until the next structure.
    Glob { next: Box<PatternNode> },
    /// The end of the document.
    End,
}
