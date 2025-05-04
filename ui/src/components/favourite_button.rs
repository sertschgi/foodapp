use crate::prelude::{components::prelude::*, *};
use api::prelude::*;
use dioxus_storage::{use_synced_storage, LocalStorage};
const FAVOURITE_BUTTON_CSS: Asset = asset!("/assets/styling/components/favourite_button.css");

#[component]
pub fn FavouriteButton(restaurant_id: Uuid, favourite: Signal<bool>) -> Element {
    let session =
        use_synced_storage::<LocalStorage, Option<LoginSession>>("session".into(), || {
            Option::<LoginSession>::None
        });

    let favourite_icon = use_memo(move || {
        if favourite() {
            rsx! { FavouriteFullIcon {} }
        } else {
            rsx! { FavouriteEmptyIcon {} }
        }
    });

    let favourite_onclick = move |e: MouseEvent| async move {
        e.stop_propagation();
        if let Some(s) = session() {
            if favourite() {
                if remove_favourite(restaurant_id, s).await.is_ok() {
                    favourite.set(false);
                }
            } else {
                if add_favourite(restaurant_id, s).await.is_ok() {
                    favourite.set(true);
                };
            }
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: FAVOURITE_BUTTON_CSS }
        button {
            class: "Favourite",
            onclick: favourite_onclick,
            {favourite_icon}
        }
    }
}
