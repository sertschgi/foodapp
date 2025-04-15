#[cfg(feature = "desktop")]
pub mod desktop;
#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "desktop")]
pub use desktop::*;
#[cfg(feature = "server")]
pub use server::*;

fn main() {
    launch()
}
