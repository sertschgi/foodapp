pub mod components;
pub mod icons;
pub mod pages;

pub(crate) use dioxus::logger::tracing::*;
pub(crate) use dioxus::prelude::*;

pub mod prelude {
    pub use super::components;
    pub use super::icons;
    pub use super::pages;
    pub(crate) use dioxus::logger::tracing::*;
    pub(crate) use dioxus::prelude::*;
    pub(crate) use icons::*;
    pub(crate) use uuid::Uuid;
}
