// use std::collections::HashMap;

// use async_trait::async_trait;
// use dataloader::{cached::Loader, BatchFn};

// use crate::models::sport::Sport;

// pub struct SportBatcher;

// #[async_trait]
// impl BatchFn<i32, Sport> for SportBatcher {
//     // A hashmap is used, as we need to return an array which maps each original key to a Sport.
//     async fn load(&self, keys: &[i32]) -> HashMap<i32, Sport> {
//         println!("load sport batch {keys:?}");
//         let mut sport_hashmap = HashMap::new();
//         get_sport_by_ids(sef&mut sport_hashmap, keys.to_vec());
//         sport_hashmap
//     }
// }

// pub type SportLoader = Loader<i32, Sport, SportBatcher>;

// // To create a new loader
// pub fn get_loader() -> SportLoader {
//     Loader::new(SportBatcher).with_yield_count(100)
// }
