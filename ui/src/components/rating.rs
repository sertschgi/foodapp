use crate::prelude::{components::prelude::*, *};
use api::prelude::*;
use dioxus_storage::{use_synced_storage, LocalStorage};

const RATING_CSS: Asset = asset!("/assets/styling/components/restaurant_result.css");

#[component]
pub fn Rating(rating: api::models::restaurants::Rating) -> Element {
    let stars = rating.stars as f32;
    let price = rating.price as f32;
    rsx! {
        document::Link { rel: "stylesheet", href: RATING_CSS }
        div {
            class: "Rating",
            RawRatingsBar {
                stars,
                price,
            }
            p {
                {rating.rating}
            }
            p {
                class: "author",
                {rating.author}
            }
        }
    }
}
