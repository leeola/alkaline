use compact_str::CompactString;

/// An optional bind param for fields of a [`Structure`].
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bind {
    #[default]
    None,
    Name(CompactString),
    OptionalName(CompactString),
}
impl From<&str> for Bind {
    fn from(name: &str) -> Self {
        Self::Name(name.into())
    }
}

#[derive(Debug)]
pub enum Pattern {
    /// The start of the document.
    Start(PatternBody),
    Body(PatternBody),
}
#[derive(Debug)]
pub enum PatternBody {
    Header {
        level: Level,
        bind: Bind,
        inner: Option<Box<PatternBody>>,
    },
    /// A glob, matching any markdown nodes until the next structure.
    Glob { next: Box<PatternBody> },
    /// The end of the document.
    End,
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    #[default]
    None,
    Absolute(u8),
    Relative(i8),
}
#[cfg(test)]
pub mod test {
    use comrak::{
        nodes::{AstNode, NodeValue},
        Arena, ComrakOptions,
    };
    #[test]
    fn exploring_comrak() {
        let input = "\
            # foo\n\
            bar baz\n\
            ## Bang\n\
        ";
        let arena = Arena::new();
        let root = comrak::parse_document(&arena, input, &ComrakOptions::default());
        modify_headers_in_ast(root);
        let mut md = Vec::<u8>::new();
        comrak::format_commonmark(root, &ComrakOptions::default(), &mut md).unwrap();
        let md = String::from_utf8(md).unwrap();
        dbg!(input, md);
    }
    fn modify_headers_in_ast<'a>(node: &'a AstNode<'a>) {
        for child in node.children() {
            dbg!(&child);
            #[allow(clippy::single_match)]
            match &mut child.data.borrow_mut().value {
                NodeValue::Heading(heading) => {
                    dbg!("got a heading");
                    // Prepend and append quotes to the heading's content
                    heading.level = 4; // Example modification, adjust as needed
                },
                _ => {
                    dbg!("not a heading?");
                },
            }
            modify_headers_in_ast(child);
        }
    }
}
