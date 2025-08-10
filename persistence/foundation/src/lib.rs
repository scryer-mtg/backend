#![forbid(unsafe_code)]

pub mod dao;
pub mod entities;
mod number_duration_ext;
pub mod sqlx_helpers;

use crate::dao::{ColorDao, DeckDao};
use anyhow::Result;
use async_trait::async_trait;
pub use number_duration_ext::*;

#[async_trait]
pub trait Storage {
    async fn connect() -> Result<impl Storage>;
    async fn disconnect(&self) -> Result<()>;

    fn color_dao(&self) -> impl ColorDao;
    fn deck_dao(&self) -> impl DeckDao;
}
