use crate::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const PLATFORM_CSS: Asset = asset!("/assets/platform.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: PLATFORM_CSS }

        Router::<Route> {}
    }
}
