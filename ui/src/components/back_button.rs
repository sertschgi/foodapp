use crate::prelude::{components::prelude::*, *};

const BACK_BUTTON_CSS: Asset = asset!("/assets/styling/components/back_button.css");

#[component]
pub fn BackButton() -> Element {
    let click = |e: MouseEvent| {
        navigator().go_back();
    };
    rsx! {
        document::Link { rel: "stylesheet", href: BACK_BUTTON_CSS }

        button {
            class: "BackButton",
            onclick: click,
            BackIcon {}
        }
    }
}
