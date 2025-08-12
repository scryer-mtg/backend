#![forbid(unsafe_code)]

mod telemetry;
pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

use crate::telemetry::init_telemetry;
use scryer_persistence::Storage;
use scryer_persistence::backends::scryer_persistence_backend_sqlite::SqliteStorage;

#[tokio::main]
async fn main() {
    let _telemetry_guard = init_telemetry();

    let storage: Box<dyn Storage> = Box::<SqliteStorage>::default();
    storage.say_hello();
}
