use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("/assets/styling/pages/home.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }

        div {
            id: "Favourite",
        }
    }
}
