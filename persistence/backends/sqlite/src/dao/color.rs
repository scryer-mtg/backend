use async_trait::async_trait;
use scryer_persistence_foundation::dao::ColorDao;
use scryer_persistence_foundation::entities::Color;
use sqlx::{SqlitePool, query_as};
use tracing::instrument;

pub struct SqliteColorDao<'a>(pub(crate) &'a SqlitePool);

#[async_trait]
impl<'a> ColorDao for SqliteColorDao<'a> {
    #[instrument(skip(self))]
    async fn get_colors(&self) -> anyhow::Result<Vec<Color>> {
        query_as::<_, Color>(r#"SELECT id, name FROM colors"#)
            .fetch_all(self.0)
            .await
            .map_err(anyhow::Error::from)
    }
}
