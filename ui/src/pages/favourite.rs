use crate::prelude::{components::prelude::*, *};
use api::prelude::*;
use dioxus_storage::{use_synced_storage, LocalStorage};

const FAVOURITE_CSS: Asset = asset!("/assets/styling/pages/favourite.css");

#[component]
pub fn Favourite() -> Element {
    let mut restaurants = use_signal(Vec::<RestaurantItem>::new);
    let session =
        use_synced_storage::<LocalStorage, Option<LoginSession>>("session".into(), || {
            Option::<LoginSession>::None
        });
    let change = move |_| async move {
        debug!("out");
        if let Some(s) = session() {
            debug!("into");
            restaurants.set(get_favourites(s).await.unwrap_or_default());
        }
    };
    rsx! {
        document::Link { rel: "stylesheet", href: FAVOURITE_CSS }

        div {
            id: "Favourite",
            onvisible: change,
            PageHeader {
                ColorMode {  }
            }
            RestaurantScroll {
                restaurants
            }
        }
    }
}
