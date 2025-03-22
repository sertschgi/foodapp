use dioxus::prelude::*;
use ui::pages::prelude::*;

use crate::layouts::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(View)]
    #[layout(WebNavbar)]
    #[layout(Page)]
    #[route("/startseite")]
    Home {},    
    #[route("/favouriten")]
    Favourite {},
    #[route("/suche")]
    Search {},
    #[end_layout]
    #[end_layout]
    #[route("/account")]
    Account {},
    #[route("/:..route")]
    Default { route: Vec<String> },
}

#[component]
pub fn Default(route: Vec<String>) -> Element {
    rsx! {
        NotFound {
            route,
            home: Route::Home {}
        }
    }
}
