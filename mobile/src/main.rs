#[cfg(feature = "mobile")]
pub mod mobile;
#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "mobile")]
pub use mobile::*;
#[cfg(feature = "server")]
pub use server::*;

fn main() {
    launch()
}
