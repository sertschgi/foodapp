use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const PLATFORM_CSS: Asset = asset!("/assets/platform.css");

use crate::route::*;

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: PLATFORM_CSS }

        Router::<Route> {}
    }
}
