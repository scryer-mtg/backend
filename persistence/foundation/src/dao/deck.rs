use crate::entities::Deck;
use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;

#[async_trait]
pub trait DeckDao {
    async fn get_decks(&self) -> anyhow::Result<Vec<Deck>>;
    async fn get_deck_by_id(&self, id: &Uuid) -> anyhow::Result<Deck>;
    async fn get_decks_by_id(&self, id: &Vec<Uuid>) -> anyhow::Result<HashMap<Uuid, Deck>>;
}
