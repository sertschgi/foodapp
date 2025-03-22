pub mod components;
pub mod pages;

pub(crate) use dioxus::logger::tracing::*;
pub(crate) use dioxus::prelude::*;

pub mod prelude {
    pub use super::components::prelude::*;
    pub use super::pages::prelude::*;
}
