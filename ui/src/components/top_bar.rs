use crate::prelude::{components::prelude::*, *};
const TOP_BAR_CSS: Asset = asset!("/assets/styling/components/top_bar.css");

#[component]
pub fn TopBar(title: String, children: Element) -> Element {
    let click = |e: MouseEvent| {
        navigator().go_back();
    };
    rsx! {
        document::Link { rel: "stylesheet", href: TOP_BAR_CSS }

        div {
            id: "TopBar",
            section {
                class: "wrapper main",
                button {
                    class: "BackButton",
                    onclick: click,
                    BackIcon {}
                }
                div {
                    class: "wrapper title",
                    h3 {
                        id: "title",
                        {title}
                    }
                }
                div {
                    class: "wrapper children",
                    {children}
                }
            }
            section {
                id: "TopBarPlaceholder"
            }
        }

    }
}
