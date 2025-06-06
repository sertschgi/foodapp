use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/components/navbar.css");

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "Navbar",
            {children}
        }
    }
}
