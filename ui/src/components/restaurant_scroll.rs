use crate::prelude::{components::prelude::*, *};
use api::prelude::*;
use dioxus_storage::{use_synced_storage, LocalStorage};
const RESTAURANT_SCROLL_CSS: Asset = asset!("/assets/styling/components/restaurant_scroll.css");

#[component]
pub fn RestaurantScroll(restaurants: Signal<Vec<RestaurantItem>>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: RESTAURANT_SCROLL_CSS }
        section {
            id: "RestaurantScroll",
            for item in restaurants.iter() {
                RestaurantResult {
                    restaurant_id: item.restaurant.id.clone(),
                    name: item.restaurant.name.clone(),
                    ratings: item.ratings.clone(),
                    favourite: Signal::new(item.favourite)
                }
            }
        }
    }
}
