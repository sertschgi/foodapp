pub mod app;
pub mod layouts;
pub mod route;

pub(crate) mod deps {
    pub(crate) use super::layouts::prelude::*;
    pub(crate) use super::route::*;
    pub(crate) use dioxus::logger::tracing::*;
    pub(crate) use dioxus::prelude::*;
    pub(crate) use ui::prelude::*;
}

pub fn launch() {
    dioxus::logger::initialize_default();
    dioxus_storage::set_dir!();
    dioxus::fullstack::prelude::server_fn::client::set_server_url("http://172.20.0.3:80");
    dioxus::launch(app::App)
}
