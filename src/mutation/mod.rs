use juniper::graphql_object;

use crate::{
    db::Database,
    models::{
        color::Color,
        color_input::ColorInput,
        phone::{Phone, PhoneType},
        phone_input::PhoneInput,
        user::User,
    },
};

pub struct Mutation;
#[graphql_object(context = Database)]
impl Mutation {
    async fn api_version() -> &'static str {
        "1.0"
    }

    async fn delete_user(
        context: &Database,
        #[graphql(description = "id of the user")] id: i32,
    ) -> Option<User> {
        match context.delete_user(&id).await {
            Ok(_) => Some(User {
                id,
                name: "deleted".to_string(),
                color: Color {
                    id: 0,
                    name: "deleted".to_string(),
                },
                phone: Phone {
                    id: 0,
                    number: "deleted".to_string(),
                    phone_type: PhoneType::Home,
                },
            }),
            Err(_) => None,
        }
    }

    async fn add_user(
        context: &Database,
        #[graphql(description = "user")] user_input: String,
        #[graphql(description = "phone")] phone_input: PhoneInput,
        #[graphql(description = "color")] color_input: ColorInput,
    ) -> Option<i32> {
        context
            .add_user(&user_input, &phone_input, &color_input)
            .await
    }
}
