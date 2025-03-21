use dioxus::prelude::*;

const PRICING_CSS: Asset = asset!("/assets/styling/pages/pricing.css");

#[component]
pub fn Pricing() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: PRICING_CSS }

        div {
            id: "Favourite",
        }
    }
}
