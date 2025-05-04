use crate::models::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AuthResponse {
    pub ident: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RestaurantItem {
    pub restaurant: Restaurant,
    pub ratings: Vec<Rating>,
    pub favourite: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetCookieResponse {
    pub cookie: String,
}

pub use crate::models::prelude::LoginSession;
pub use crate::models::prelude::UserInfo;
