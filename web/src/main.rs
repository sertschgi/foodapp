pub(crate) use dioxus::prelude::*;
pub(crate) use route::*;

mod app;
mod layouts;
mod route;

fn main() {
    dioxus::logger::initialize_default();
    dioxus::launch(app::App);
}
