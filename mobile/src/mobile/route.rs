use crate::prelude::{pages::prelude as pages, *};

pub fn Home() -> Element {
    rsx! {
        pages::Home {
            account_route: Route::Account {},
            settings_route: Route::Settings{},
        }
    }
}

use pages::Restaurant;

pub fn Favourite() -> Element {
    rsx! {
        pages::Favourite {}
    }
}

pub fn Search() -> Element {
    rsx! {
        pages::Search {}
    }
}

pub fn Account() -> Element {
    rsx! {
        pages::Account {}
    }
}

pub fn Settings() -> Element {
    rsx! {}
}

#[component]
pub fn HandleNotFound(route: Vec<String>) -> Element {
    let nav = navigator();
    debug!("route: {:?}", route);
    if route.is_empty() {
        nav.push(Route::Home {});
    }
    rsx! {
        pages::NotFound {
            route,
            home: Route::Home {}
        }
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(View)]
    #[layout(MobileNavbar)]
    #[layout(Page)]
    #[route("/startseite")]
    Home {},    
    #[route("/favoriten")]
    Favourite {},
    #[route("/suche")]
    Search {},
    #[end_layout]
    #[end_layout]
    #[layout(MobileTopBar)]
    #[layout(Page)]
    #[route("/account")]
    Account {},
    #[route("/settings")]
    Settings {},
    #[end_layout]
    #[end_layout]
    #[route("/restaurant/:name")]
    Restaurant { name: String },   
    #[route("/:..route")]
    HandleNotFound { route: Vec<String> },
}
