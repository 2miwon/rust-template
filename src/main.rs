use rust_template::{api, config::Config, utils};

fn main() -> rust_template::Result<()> {
    let cfg = Config::from_env()?;
    utils::init_tracing(&cfg.log_level);

    tracing::info!(app_name = %cfg.app_name, "starting up");

    let repo = utils::InMemoryItemRepository::seeded();
    match api::get_item(&repo, 1)? {
        Some(item) => tracing::info!(?item, "found item"),
        None => tracing::warn!("item not found"),
    }

    Ok(())
}
