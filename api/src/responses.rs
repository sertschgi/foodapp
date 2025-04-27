pub(crate) mod deps {
    pub(crate) use derive_builder::Builder;
    pub(crate) use serde::{Deserialize, Serialize};
}
use deps::*;

#[derive(Serialize, Deserialize, Builder)]
pub struct AuthResponse {
    pub ident: String,
    pub password: String,
}
