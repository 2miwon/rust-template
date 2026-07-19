use crate::core::domain::Item;
use crate::error::Result;

/// Port: implemented by an outer layer (see `utils`) so that use cases stay
/// independent of any concrete storage or framework (Dependency Inversion).
pub trait ItemRepository {
    /// Looks up an [`Item`] by id, or `None` if it doesn't exist.
    fn find(&self, id: u64) -> Result<Option<Item>>;
}

/// Use case: fetch a single [`Item`] through an [`ItemRepository`] port.
pub struct GetItem<'a, R: ItemRepository> {
    repo: &'a R,
}

impl<'a, R: ItemRepository> GetItem<'a, R> {
    /// Builds the use case against a given repository implementation.
    pub fn new(repo: &'a R) -> Self {
        Self { repo }
    }

    /// Runs the use case for the given id.
    pub fn execute(&self, id: u64) -> Result<Option<Item>> {
        self.repo.find(id)
    }
}
