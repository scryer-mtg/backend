use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Deck {
    pub id: uuid::Uuid,
    pub name: String,
    pub active: bool,
}
