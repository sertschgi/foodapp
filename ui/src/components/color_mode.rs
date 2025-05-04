use crate::prelude::{components::prelude::*, *};
use api::prelude::*;
use dioxus_storage::{use_synced_storage, LocalStorage};

#[component]
pub fn ColorMode() -> Element {
    let mut darkmode = use_synced_storage::<LocalStorage, bool>("darkmode".into(), || false);

    rsx! {
        a {
            id: "ColorMode",
            onclick: move |_| {
                if *darkmode.peek() {
                    darkmode.set(false);
                } else {
                    darkmode.set(true);
                }
            },
            if darkmode() {
                ColorModeIconFull {}
            } else {
                ColorModeIconEmpty {}
            }
        }
    }
}
