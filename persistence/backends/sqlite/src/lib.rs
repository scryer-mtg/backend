#![forbid(unsafe_code)]

mod dao;

use crate::dao::{SqliteColorDao, SqliteDeckDao};
use async_trait::async_trait;
use scryer_persistence_foundation::dao::{ColorDao, DeckDao};
use scryer_persistence_foundation::{Storage, from_minutes};
use sqlx::SqlitePool;
use sqlx::migrate::Migrator;
use sqlx::sqlite::SqlitePoolOptions;
use std::thread::available_parallelism;
use std::time::Duration;
use tracing::{Instrument, debug_span, instrument};

const DEFAULT_POOL_SIZE: u32 = 4;
const MAX_POOL_SIZE: u32 = 64;
const MIN_POOL_SIZE: u32 = 2;
const POOL_LIFETIME: Duration = from_minutes(30);

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[derive(Debug, Clone)]
pub struct SqliteStorage(pub(crate) SqlitePool);

#[async_trait]
impl Storage for SqliteStorage {
    #[instrument]
    async fn connect() -> anyhow::Result<Self> {
        let connection_count: u32 = match available_parallelism() {
            Ok(n) => n.get() as u32,
            Err(_) => DEFAULT_POOL_SIZE,
        }
        .min(MAX_POOL_SIZE)
        .max(MIN_POOL_SIZE);

        let pool = SqlitePoolOptions::new()
            .max_connections(connection_count)
            .max_lifetime(POOL_LIFETIME)
            .test_before_acquire(true)
            .connect("sqlite::memory:")
            .instrument(debug_span!(
                "Connecting to SQLite database",
                "sql.pool_size" = connection_count
            ))
            .await?;

        MIGRATOR
            .run(&pool)
            .instrument(debug_span!("Applying migrations"))
            .await?;

        Ok(Self(pool))
    }

    async fn disconnect(&self) -> anyhow::Result<()> {
        self.0.close().await;
        Ok(())
    }

    fn color_dao(&self) -> impl ColorDao {
        SqliteColorDao(&self.0)
    }

    fn deck_dao(&self) -> impl DeckDao {
        SqliteDeckDao(&self.0)
    }
}
