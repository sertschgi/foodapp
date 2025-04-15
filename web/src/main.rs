pub mod app;
pub mod layouts;
pub mod prelude;
pub mod route;

fn main() {
    dioxus::logger::initialize_default();
    // dioxus::fullstack::prelude::server_fn::client::set_server_url("http://172.21.0.3:8089");
    dioxus::LaunchBuilder::web().launch(app::App);
}
