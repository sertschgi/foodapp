pub mod desktop;
#[cfg(feature = "server")]
pub mod server;

fn main() {
    #[cfg(feature = "server")]
    server::launch();
    #[cfg(feature = "desktop")]
    desktop::launch()
}
