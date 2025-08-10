use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Color {
    pub id: uuid::Uuid,
    pub name: String,
}
