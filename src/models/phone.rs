use std::fmt::Display;

use juniper::{GraphQLEnum, GraphQLObject};

#[derive(Clone, GraphQLObject)]
pub struct Phone {
    pub id: i32,
    pub number: String,
    pub phone_type: PhoneType,
}

#[derive(Clone, GraphQLEnum)]
pub enum PhoneType {
    Home,
    Work,
    Mobile,
}

impl From<String> for PhoneType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "HOME" => PhoneType::Home,
            "WORK" => PhoneType::Work,
            "MOBILE" => PhoneType::Mobile,
            _ => PhoneType::Mobile,
        }
    }
}

impl Display for PhoneType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhoneType::Home => write!(f, "HOME"),
            PhoneType::Work => write!(f, "WORK"),
            PhoneType::Mobile => write!(f, "MOBILE"),
        }
    }
}
