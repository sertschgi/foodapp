use dioxus::prelude::*;

const VIEW_CSS: Asset = asset!("/assets/styling/components/view.css");

#[component]
pub fn View(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: VIEW_CSS }

        div {
            id: "View",
            {children}
        }
    }
}
