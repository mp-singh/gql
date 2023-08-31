use juniper::GraphQLInputObject;

use super::{color_input::ColorInput, phone_input::PhoneInput};

#[derive(Clone, GraphQLInputObject)]
///a user
pub struct UserInput {
    ///the name
    pub name: String,
    ///the color
    pub color: ColorInput,
    ///the phone number
    pub phone: PhoneInput,
}
