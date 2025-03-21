pub mod navbar;
pub mod navbar_button;
pub mod page;
pub mod view;

pub mod prelude {
    pub use super::navbar::Navbar;
    pub use super::navbar_button::NavbarButton;
    pub use super::page::Page;
    pub use super::view::View;
}
