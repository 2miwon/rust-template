use rust_template::{api, utils};

#[test]
fn get_item_returns_seeded_item() {
    let repo = utils::InMemoryItemRepository::seeded();

    let result = api::get_item(&repo, 1).unwrap();

    assert_eq!(result.unwrap().name, "sample-item");
}

#[test]
fn get_item_returns_none_for_missing_id() {
    let repo = utils::InMemoryItemRepository::seeded();

    let result = api::get_item(&repo, 999).unwrap();

    assert!(result.is_none());
}
