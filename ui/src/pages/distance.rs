use dioxus::prelude::*;

const DISTANCE_CSS: Asset = asset!("/assets/styling/pages/distance.css");

#[component]
pub fn Distance() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: DISTANCE_CSS }

        div {
            id: "Favourite",
        }
    }
}
