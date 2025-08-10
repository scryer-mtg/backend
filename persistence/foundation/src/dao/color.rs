use crate::entities::Color;
use async_trait::async_trait;

#[async_trait]
pub trait ColorDao {
    async fn get_colors(&self) -> anyhow::Result<Vec<Color>>;
}
