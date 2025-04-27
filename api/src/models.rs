pub mod auth;
pub mod restaurants;

pub(crate) mod deps {
    pub(crate) use crate::schema::*;
    pub(crate) use chrono::prelude::*;
    pub(crate) use derive_builder::Builder;
    pub(crate) use diesel::prelude::*;
    pub(crate) use diesel::serialize::*;
    pub(crate) use ipnet::IpNet;
    pub(crate) use postgis_diesel::types::*;
    pub(crate) use serde::{Deserialize, Serialize};
    pub(crate) use uuid::Uuid;
}

pub mod prelude {
    pub use super::auth::*;
    pub use super::restaurants::*;
}
