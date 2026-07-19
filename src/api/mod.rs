//! Interface adapters: translate between the outside world (CLI, HTTP, ...)
//! and the application's use cases. Add handlers/controllers here.

pub mod dto;

use crate::core::usecase::{GetItem, ItemRepository};
use crate::error::Result;
use dto::ItemResponse;

/// Fetches an item and adapts it to an [`ItemResponse`] for callers outside
/// `core` (e.g. an HTTP handler or CLI command would call this).
pub fn get_item<R: ItemRepository>(repo: &R, id: u64) -> Result<Option<ItemResponse>> {
    let item = GetItem::new(repo).execute(id)?;
    Ok(item.map(ItemResponse::from))
}
