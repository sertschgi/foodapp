use crate::prelude::*;

const PLATFORM_CSS: Asset = asset!("/assets/platform.css");

#[component]
pub fn App() -> Element {
    // let result = std::panic::catch_unwind(|| {
    //     dioxus::fullstack::prelude::server_fn::client::set_server_url("http://172.20.0.3:80");
    // });
    rsx! {
        document::Link { rel: "stylesheet", href: PLATFORM_CSS }
        Router::<Route> { config: || {
            RouterConfig::default()
        }}
        // if result.is_err() {
        //     "Bitte schalte dein Internet ein, um die App nutzen zu können :)"
        // }
    }
}
