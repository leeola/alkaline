use crate::pattern::Bind;

#[derive(Debug, Default)]
pub enum PatternMatch {
    /// The start of the document.
    #[default]
    Start,
    Node(PatternMatchBody),
}
#[derive(Debug)]
pub enum PatternMatchBody {
    Header {
        level: Option<u8>,
        bind: Bind,
        inner: Option<Box<PatternMatchBody>>,
    },
    /// A glob, matching any markdown nodes until the next structure.
    Glob { next: Box<PatternMatchBody> },
    /// The end of the document.
    End,
}
