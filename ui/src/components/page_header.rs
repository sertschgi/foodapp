use change_case::pascal_case;
use dioxus::prelude::*;

const PAGE_HEADER_CSS: Asset = asset!("/assets/styling/components/page_header.css");

#[component]
pub fn PageHeader(children: Element) -> Element {
    let route_str = router().full_route_string();
    let unfmt_name = route_str.split("/").last().unwrap_or_default();
    let name = pascal_case(unfmt_name);
    rsx! {
        document::Link { rel: "stylesheet", href: PAGE_HEADER_CSS }

        div {
            id: "PageHeader",
            h1 {
                "{name}"
            }
            aside {
                {children}
            }
        }
    }
}
