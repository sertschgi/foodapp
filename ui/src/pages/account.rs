use dioxus::prelude::*;

const ACCOUNT_CSS: Asset = asset!("/assets/styling/pages/account.css");

#[component]
pub fn Account() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: ACCOUNT_CSS }

        div {
            id: "Favourite",
        }
    }
}
