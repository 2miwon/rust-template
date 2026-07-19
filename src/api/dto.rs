use crate::core::domain::Item;

/// Outward-facing shape of an [`Item`], decoupled from the domain type so
/// the two can evolve independently.
#[derive(Debug, Clone)]
pub struct ItemResponse {
    pub id: u64,
    pub name: String,
}

impl From<Item> for ItemResponse {
    fn from(item: Item) -> Self {
        Self {
            id: item.id,
            name: item.name,
        }
    }
}
