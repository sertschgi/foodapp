#[cfg(feature = "server")]
pub mod server;
pub mod web;

fn main() {
    #[cfg(feature = "server")]
    server::launch();
    #[cfg(feature = "web")]
    web::launch()
}
