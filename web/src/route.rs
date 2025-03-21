use dioxus::prelude::*;
use ui::pages::prelude::*;

use crate::layouts::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/favourite")]
    Favourite {},
    #[route("/pricing")]
    Pricing {},
    #[route("/distance")]
    Distance {}
}
