use dioxus::prelude::*;
use dioxus_storage::{use_synced_storage, LocalStorage};

const VIEW_CSS: Asset = asset!("/assets/styling/components/view.css");

#[component]
pub fn View(children: Element) -> Element {
    let darkmode = use_synced_storage::<LocalStorage, bool>("darkmode".into(), || false);
    use_effect(move || {
        if darkmode() {
            dioxus::logger::tracing::info!("Darkmode now active!");
        } else {
            dioxus::logger::tracing::info!("Lightmode now active!");
        }
    });
    rsx! {
        document::Link { rel: "stylesheet", href: VIEW_CSS }

        div {
            id: "View",
            "darkmode": darkmode(),
            {children}
        }
    }
}
