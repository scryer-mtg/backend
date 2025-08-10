#![forbid(unsafe_code)]

use scryer_persistence_foundation::Storage;

#[derive(Debug, Copy, Clone, Default)]
pub struct SqliteStorage;

impl Storage for SqliteStorage {
    fn say_hello(&self) {
        println!("Hello from sqlite");
    }
}
