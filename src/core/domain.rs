/// A domain entity. Pure data + invariants — no I/O, no framework types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub id: u64,
    pub name: String,
}

impl Item {
    /// Creates a new [`Item`].
    pub fn new(id: u64, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}
