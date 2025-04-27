pub(crate) mod deps {
    pub(crate) use derive_builder::Builder;
    pub(crate) use serde::{Deserialize, Serialize};
}
use deps::*;

#[derive(Serialize, Deserialize, Builder)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Builder)]
pub struct LoginRequest {
    pub ident: String,
    pub password: String,
}
