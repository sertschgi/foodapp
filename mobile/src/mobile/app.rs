use crate::prelude::*;

const PLATFORM_CSS: Asset = asset!("/assets/platform.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: PLATFORM_CSS }
        Router::<Route> { config: || {
            RouterConfig::default()
        }}
    }
}
