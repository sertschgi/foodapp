use crate::prelude::NewRating;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Builder, Debug, Clone)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Builder, Debug, Clone)]
pub struct LoginRequest {
    pub ident: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Builder, Debug, Clone)]
pub struct RatingRequest {
    pub restaurant_id: Uuid,
    pub stars: i16,
    pub price: i16,
    pub rating: Option<String>,
}
