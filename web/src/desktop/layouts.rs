pub mod go_back;
pub mod mobile_navbar;
pub mod mobile_top_bar;
pub mod page;
pub mod view;

pub mod prelude {
    pub use super::go_back::GoBack;
    pub use super::mobile_navbar::MobileNavbar;
    pub use super::mobile_top_bar::MobileTopBar;
    pub use super::page::Page;
    pub use super::view::View;
}
