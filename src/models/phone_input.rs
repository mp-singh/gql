use std::fmt::Display;

use juniper::{GraphQLEnum, GraphQLInputObject};

#[derive(Clone, GraphQLInputObject)]
pub struct PhoneInput {
    pub number: String,
    pub phone_type: PhoneTypeInput,
}

#[derive(Clone, GraphQLEnum)]
pub enum PhoneTypeInput {
    Home,
    Work,
    Mobile,
}

impl From<String> for PhoneTypeInput {
    fn from(s: String) -> Self {
        match s.as_str() {
            "HOME" => PhoneTypeInput::Home,
            "WORK" => PhoneTypeInput::Work,
            "MOBILE" => PhoneTypeInput::Mobile,
            _ => PhoneTypeInput::Mobile,
        }
    }
}

impl Display for PhoneTypeInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhoneTypeInput::Home => write!(f, "HOME"),
            PhoneTypeInput::Work => write!(f, "WORK"),
            PhoneTypeInput::Mobile => write!(f, "MOBILE"),
        }
    }
}
