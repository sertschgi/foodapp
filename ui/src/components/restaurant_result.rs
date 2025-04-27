use crate::prelude::{components::prelude::*, *};

const RESTAURANT_RESULT_CSS: Asset = asset!("/assets/styling/components/restaurant_result.css");

#[component]
pub fn RestaurantResult(
    #[props(into)] name: String,
    #[props(into)] stars: f32,
    #[props(into)] price: f32,
) -> Element {
    debug!("stars: {stars}");
    debug!("price: {price}");

    let full_stars = stars as i16;
    let half_star = stars.fract() > 0.5;
    let empty_stars = 5 - full_stars - half_star as i16;
    let full_price = price as i16;
    let empty_price = 3 - full_price;

    rsx! {
        document::Link { rel: "stylesheet", href: RESTAURANT_RESULT_CSS }
        div {
            class: "RestaurantResult",
            button {
                class: "Favourite",
                AddFavouriteIcon {}
            }
            img {

            }
            h2 {
                class: "RestaurantName",
                {name}
            }
            div {
                class: "Ratings Bar",
                for _ in 0..full_stars {
                    StarFullIcon {}
                }
                if half_star {
                    StarHalfIcon {}
                }
                for _ in 0..empty_stars {
                    StarEmptyIcon {}
                }
            }
            div {
                class: "Price Bar",
                for _ in 0..full_price {
                    PriceFullIcon {}
                }
                for _ in 0..empty_price {
                    PriceEmptyIcon {}
                }
            }
        }
    }
}
