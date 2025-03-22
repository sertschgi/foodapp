use dioxus::prelude::*;

const PAGE_CSS: Asset = asset!("/assets/styling/components/page.css");

#[component]
pub fn Page(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: PAGE_CSS }

        div {
            id: "Page",
            {children}
        }
    }
}
