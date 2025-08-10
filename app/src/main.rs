#![forbid(unsafe_code)]

mod telemetry;
pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

use crate::telemetry::init_telemetry;
use scryer_persistence::Storage;
use scryer_persistence::backends::scryer_persistence_backend_sqlite::SqliteStorage;
use scryer_persistence::dao::{ColorDao, DeckDao};
use tracing::info;
use uuid::uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _telemetry_guard = init_telemetry();

    let storage = SqliteStorage::connect().await?;
    do_work(&storage).await.expect("This should work");
    storage.disconnect().await?;
    Ok(())
}

async fn do_work(storage: &impl Storage) -> anyhow::Result<()> {

    let colors = storage.color_dao().get_colors().await?;
    info!("{:#?}", colors);

    let ids = vec![
        uuid!("019899fb-b71a-730f-846f-478e315ebd34"),
        uuid!("019899fb-b71a-730f-846f-000000000000"),
    ];
    let decks = storage.deck_dao().get_decks_by_id(&ids).await?;
    info!("{:#?}", decks);

    Ok(())
}
