use juniper::graphql_object;

use crate::{
    db::Database,
    models::{color::Color, sport::Sport, user::User},
};

pub struct Query;
#[graphql_object(context = Database)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }

    async fn user(
        context: &Database,
        #[graphql(description = "id of the user")] id: i32,
    ) -> Option<User> {
        context.get_user(&id).await
    }
    async fn get_user_count(context: &Database) -> i32 {
        context.get_user_count().await
    }
    async fn users(context: &Database) -> Vec<User> {
        context.get_users().await
    }
    async fn colors(context: &Database) -> Vec<Color> {
        context.get_colors().await
    }
    async fn sport(
        context: &Database,
        #[graphql(description = "id of the user")] id: i32,
    ) -> Option<Sport> {
        match context.sport_loader.try_load(id).await {
            Ok(sport) => Some(sport),
            Err(_) => None,
        }
    }
    async fn sports(context: &Database) -> Vec<Sport> {
        context.get_sports().await
    }
}
