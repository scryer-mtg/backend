#![forbid(unsafe_code)]

use scryer_persistence::Storage;
use scryer_persistence::backends::scryer_persistence_backend_sqlite::SqliteStorage;

fn main() {
    let storage: Box<dyn Storage> = Box::<SqliteStorage>::default();
    storage.say_hello();
}
