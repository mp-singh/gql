use juniper::GraphQLObject;

use super::{color::Color, phone::Phone};

#[derive(Clone, GraphQLObject)]
///a user
pub struct User {
    ///the id
    pub id: i32,
    ///the name
    pub name: String,
    ///the color
    pub color: Color,
    ///the phone number
    pub phone: Phone,
}
