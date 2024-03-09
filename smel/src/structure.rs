use std::collections::HashMap;

pub type Field = String;
pub type Value = String;

/// The output of a [`Pattern`](crate::pattern::Pattern) onto a
/// [`Document`](crate::document::Document).
pub struct Structure {
    // TODO: Think i need to add an identical enum as `Pattern` but with the associated nodes.
    // Otherwise this can't directly create the markdown ast nodes.
    //
    // Alternatively, the combination of the pattern + struct could handle creation. Bit awkward
    // tho.
    fields: HashMap<Field, Value>,
}
