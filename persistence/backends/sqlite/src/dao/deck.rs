use async_trait::async_trait;
use itertools::Itertools;
use scryer_persistence_foundation::dao::DeckDao;
use scryer_persistence_foundation::entities::Deck;
use scryer_persistence_foundation::sqlx_helpers::QueryAsExt;
use sqlx::types::Uuid;
use sqlx::{SqlitePool, query_as};
use std::collections::HashMap;
use std::iter;
use tracing::instrument;

pub struct SqliteDeckDao<'a>(pub(crate) &'a SqlitePool);

#[async_trait]
impl<'a> DeckDao for SqliteDeckDao<'a> {
    #[instrument(skip(self))]
    async fn get_decks(&self) -> anyhow::Result<Vec<Deck>> {
        query_as::<_, Deck>(r#"SELECT id, name, active FROM decks"#)
            .fetch_all(self.0)
            .await
            .map_err(anyhow::Error::from)
    }

    #[instrument(skip(self))]
    async fn get_deck_by_id(&self, id: &Uuid) -> anyhow::Result<Deck> {
        query_as::<_, Deck>(r#"SELECT id, name, active FROM decks WHERE id = ?"#)
            .bind(id)
            .fetch_one(self.0)
            .await
            .map_err(anyhow::Error::from)
    }

    #[instrument(skip(self))]
    async fn get_decks_by_id(&self, ids: &Vec<Uuid>) -> anyhow::Result<HashMap<Uuid, Deck>> {
        if ids.len() == 0 {
            return Ok(HashMap::new());
        }

        query_as::<_, Deck>(
            #[allow(unstable_name_collisions)]
            format!(
                // language=sqlite
                r#"SELECT id, name, active FROM decks WHERE id IN ({})"#,
                iter::repeat_n('?', ids.len())
                    .intersperse(',')
                    .collect::<String>()
                    .as_str()
            )
            .as_str(),
        )
        .bind_all(ids)
        .fetch_all(self.0)
        .await
        .map(move |decks| {
            let mut map = HashMap::<Uuid, Deck>::new();
            for deck in decks {
                map.insert(deck.id, deck);
            }
            map
        })
        .map_err(anyhow::Error::from)
    }
}
