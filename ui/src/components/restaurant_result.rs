use crate::prelude::{components::prelude::*, *};

const RESTAURANT_RESULT_CSS: Asset = asset!("/assets/styling/components/restaurant_result.css");

#[component]
pub fn RestaurantResult() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: RESTAURANT_RESULT_CSS }
        div {
            class: "RestaurantResult",
            img {

            }
            h2 {
                class: "RestaurantName",
                "Sample Restaurant"
            }
            div {
                class: "Ratings Bar",
                StarFullIcon {}
                StarFullIcon {}
                StarFullIcon {}
                StarFullIcon {}
                StarFullIcon {}
            }
            div {
                class: "Price Bar",
                PriceFullIcon {}
                PriceFullIcon {}
                PriceFullIcon {}
            }
        }
    }
}
