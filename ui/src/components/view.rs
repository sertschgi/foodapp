use dioxus::prelude::*;
use dioxus_storage::{use_synced_storage, LocalStorage};

const VIEW_CSS: Asset = asset!("/assets/styling/components/view.css");

#[component]
pub fn View(children: Element) -> Element {
    let darkmode = use_synced_storage::<LocalStorage, bool>("darkmode".into(), || false);
    rsx! {
        document::Link { rel: "stylesheet", href: VIEW_CSS }

        div {
            id: "View",
            "darkmode": darkmode(),
            {children}
        }
    }
}
