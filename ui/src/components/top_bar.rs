use crate::prelude::{components::prelude::*, *};
use change_case::pascal_case;

const TOP_BAR_CSS: Asset = asset!("/assets/styling/components/top_bar.css");

#[component]
pub fn TopBar(children: Element) -> Element {
    let route_str = router().full_route_string();
    let unfmt_name = route_str.split("/").last().unwrap_or_default();
    let title = pascal_case(unfmt_name);
    let click = |e: MouseEvent| {
        navigator().go_back();
    };
    rsx! {
        document::Link { rel: "stylesheet", href: TOP_BAR_CSS }

        div {
            id: "TopBar",
            div {
                class: "wrapper",
                button {
                    class: "BackButton",
                    onclick: click,
                    BackIcon {}
                }
                h3 {
                    class: "Title",
                    {title}
                }
                {children}
            }
            div {
                id: "TopBarPlaceholder"
            }
        }

    }
}
