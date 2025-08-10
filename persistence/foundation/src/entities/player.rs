use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Player {
    pub id: uuid::Uuid,
    pub name: String,
}
