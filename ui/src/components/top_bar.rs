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
