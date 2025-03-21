use dioxus::prelude::*;

const FAVOURITE_CSS: Asset = asset!("/assets/styling/pages/favourite.css");

#[component]
pub fn Favourite() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: FAVOURITE_CSS }

        div {
            id: "Favourite",
        }
    }
}
