use crate::*;

const NOT_FOUND_CSS: Asset = asset!("/assets/styling/pages/not_found.css");

#[component]
pub fn NotFound(route: Vec<String>, #[props(into)] home: NavigationTarget) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NOT_FOUND_CSS }

        div {
            id: "NotFound",
            h1 { "404" }
            p { "Route: {route:?}" }
            p { "Sieht so aus als hättest du dich verirrt..." }
            Link {
                to: home,
                svg {
                    "viewBox": "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "currentColor",
                    path { d: "M7.82843 10.9999H20V12.9999H7.82843L13.1924 18.3638L11.7782 19.778L4 11.9999L11.7782 4.22168L13.1924 5.63589L7.82843 10.9999Z" }
                }
                span { "Zur Startseite" }
            }
        }
    }
}
