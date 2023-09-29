use std::collections::HashMap;

use async_trait::async_trait;
use dataloader::{cached::Loader, BatchFn};

use crate::{models::sport::Sport, DATABASE};
pub struct SportBatcher;

#[async_trait]
impl BatchFn<i32, Sport> for SportBatcher {
    // A hashmap is used, as we need to return an array which maps each original key to a Sport.
    async fn load(&mut self, keys: &[i32]) -> HashMap<i32, Sport> {
        let mut sport_hashmap = HashMap::new();
        get_sport_by_ids(&mut sport_hashmap, keys.to_vec()).await;
        sport_hashmap
    }
}

pub type SportLoader = Loader<i32, Sport, SportBatcher>;

// To create a new loader
pub fn get_loader() -> SportLoader {
    Loader::new(SportBatcher).with_yield_count(100)
}

pub async fn get_sport_by_ids(hashmap: &mut HashMap<i32, Sport>, ids: Vec<i32>) {
    let db = &DATABASE.get().await.conn_ref;
    let ids: String = ids
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    sqlx::query!(r#"SELECT id, name FROM sports WHERE id IN ($1)"#, ids)
        .fetch_all(db)
        .await
        .unwrap()
        .into_iter()
        .for_each(|row| {
            let sport = Sport {
                id: row.id as i32,
                name: row.name,
            };
            hashmap.insert(sport.id, sport);
        });
}
