/// An optional bind param for fields of a [`Structure`].
#[derive(Default)]
pub enum Bind {
    #[default]
    None,
    Name(String),
}
/// A markdown "structure" in Smel matches some markdown pattern, with context to update or insert
/// instances of that pattern.
pub enum Structure {
    /// The start of the document.
    Start,
    Header {
        level: u8,
        bind: Bind,
        next: Option<Box<Structure>>,
    },
    /// A glob, matching any markdown nodes until the next structure.
    //
    // TODO: I think there's various types of lookaheads
    Glob { next: Box<Structure> },
    /// The end of the document.
    End,
}
impl Structure {}

pub struct Value;
