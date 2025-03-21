use dioxus::prelude::*;

const HERO_CSS: Asset = asset!("/assets/styling/components/page.css");

#[component]
pub fn Page(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }

        div {
            id: "Page",
            {children}
        }
    }
}
