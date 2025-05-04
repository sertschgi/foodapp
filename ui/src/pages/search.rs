use crate::components::prelude::*;
use crate::*;
use api::prelude::*;
use dioxus_storage::{use_synced_storage, LocalStorage};
use serde_json::from_str as serialize;

const SEARCH_CSS: Asset = asset!("/assets/styling/pages/search.css");

#[component]
pub fn Search() -> Element {
    let mut restaurants = use_signal(Vec::<RestaurantItem>::new);
    let session =
        use_synced_storage::<LocalStorage, Option<LoginSession>>("session".into(), || {
            Option::<LoginSession>::None
        });
    let change = move |e: FormEvent| async move {
        restaurants.set(
            get_restaurants(format!("{}", e.value()), session())
                .await
                .unwrap_or_default(),
        );
    };
    rsx! {
        document::Link { rel: "stylesheet", href: SEARCH_CSS }

        div {
            id: "Search",
             PageHeader {
                ColorMode {  }
             }
            form {
                onsubmit: |e| { e.prevent_default() },
                label {
                    svg {
                        class: "icon",
                        fill: "currentColor",
                        "viewBox": "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M18.031 16.6168L22.3137 20.8995L20.8995 22.3137L16.6168 18.031C15.0769 19.263 13.124 20 11 20C6.032 20 2 15.968 2 11C2 6.032 6.032 2 11 2C15.968 2 20 6.032 20 11C20 13.124 19.263 15.0769 18.031 16.6168ZM16.0247 15.8748C17.2475 14.6146 18 12.8956 18 11C18 7.1325 14.8675 4 11 4C7.1325 4 4 7.1325 4 11C4 14.8675 7.1325 18 11 18C12.8956 18 14.6146 17.2475 15.8748 16.0247L16.0247 15.8748Z" }
                    }
                    input {
                        type: "search",
                        oninput: change
                    }
                }
            }
            RestaurantScroll {
                restaurants
            }
        }
    }
}
