use crate::prelude::{components::prelude::*, *};
use api::prelude::*;
use base64::prelude::*;
use dioxus_storage::{use_synced_storage, LocalStorage};

const RESTAURANT_RESULT_CSS: Asset = asset!("/assets/styling/components/restaurant_result.css");

#[component]
pub fn RestaurantResult(
    restaurant_id: Uuid,
    #[props(into)] name: String,
    ratings: Vec<Rating>,
    favourite: Signal<bool>,
) -> Element {
    let n1 = name.clone();
    let click = move |_| {
        navigator().push(format!("/restaurant/{}", n1));
    };
    rsx! {
        document::Link { rel: "stylesheet", href: RESTAURANT_RESULT_CSS }
        div {
            class: "RestaurantResult",
            FavouriteButton {
                restaurant_id,
                favourite,
            }
            div {
                class: "wrapper",
                onclick: click.clone(),
                img {
                    class: "image",
                    src: "https://dynamic-media-cdn.tripadvisor.com/media/photo-o/0f/b4/d6/77/die-gemutliche-alte-gaststube.jpg",
                }
            }
            div {
                class: "info",
                h2 {
                    class: "name",
                    onclick: click,
                    {name}
                }
                RatingsBar {
                    restaurant_id,
                    ratings,
                }
            }
        }
    }
}
