//! Frameworks & drivers: concrete implementations of the ports defined in
//! `core` (e.g. a real database adapter would live here), plus shared
//! cross-cutting helpers like logging setup.

use std::collections::HashMap;

use tracing_subscriber::EnvFilter;

use crate::core::domain::Item;
use crate::core::usecase::ItemRepository;
use crate::error::Result;

/// Initializes the global `tracing` subscriber. `RUST_LOG` overrides
/// `default_level` when set.
pub fn init_tracing(default_level: &str) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level));

    tracing_subscriber::fmt().with_env_filter(filter).init();
}

/// In-memory `ItemRepository` for the template/demo. Swap for a real
/// database adapter without touching `core` or `api`.
pub struct InMemoryItemRepository {
    items: HashMap<u64, Item>,
}

impl InMemoryItemRepository {
    /// Builds a repository pre-populated with one sample item (id `1`).
    pub fn seeded() -> Self {
        let mut items = HashMap::new();
        items.insert(1, Item::new(1, "sample-item"));
        Self { items }
    }
}

impl ItemRepository for InMemoryItemRepository {
    fn find(&self, id: u64) -> Result<Option<Item>> {
        Ok(self.items.get(&id).cloned())
    }
}
