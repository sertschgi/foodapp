use crate::prelude::{components::prelude::*, *};
use api::prelude::*;
use dioxus_storage::{use_synced_storage, LocalStorage};

#[component]
pub fn SelectableIcon(
    empty: Element,
    full: Element,
    selected: Signal<bool>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let mut pressed = use_signal(|| 0);
    use_effect(move || selected.set(!pressed() % 2 == 0));
    rsx! {
        div {
            class: "SelectableIcon",
            onclick: move |e| {
                 pressed += 1;
                onclick.unwrap_or_default()(e)
            },
            if selected() {
                {full}
            } else {
                {empty}
            }
        }
    }
}

#[component]
fn Star(onclick: Option<EventHandler<MouseEvent>>, selected: Signal<bool>) -> Element {
    rsx! {
        SelectableIcon {
            empty: StarEmptyIcon(),
            full: StarFullIcon(),
            onclick,
            selected
        }
    }
}

#[component]
fn Price(onclick: Option<EventHandler<MouseEvent>>, selected: Signal<bool>) -> Element {
    rsx! {
        SelectableIcon {
            empty: PriceEmptyIcon(),
            full: PriceFullIcon(),
            onclick,
            selected
        }
    }
}

#[component]
pub fn ActiveRatingsBar(stars: Signal<i16>, price: Signal<i16>) -> Element {
    let mut star5 = use_signal(|| false);
    let mut star4 = use_signal(|| false);
    let mut star3 = use_signal(|| false);
    let mut star2 = use_signal(|| false);
    let mut star1 = use_signal(|| false);
    use_effect(move || {
        if star5() {
            star4.set(true);
        }
    });
    use_effect(move || {
        if star4() {
            star3.set(true);
        }
    });
    use_effect(move || {
        if star3() {
            star2.set(true);
        }
    });
    use_effect(move || {
        if star2() {
            star1.set(true);
        }
    });
    use_effect(move || {
        if !star4() {
            star5.set(false);
        }
    });
    use_effect(move || {
        if !star3() {
            star4.set(false);
        }
    });
    use_effect(move || {
        if !star2() {
            star3.set(false);
        }
    });
    use_effect(move || {
        if !star1() {
            star2.set(false);
        }
    });

    use_effect(move || {
        let r = {
            if star5() {
                5
            } else if star4() {
                4
            } else if star3() {
                3
            } else if star2() {
                2
            } else if star1() {
                1
            } else {
                0
            }
        };
        stars.set(r);
    });

    let mut price3 = use_signal(|| false);
    let mut price2 = use_signal(|| false);
    let mut price1 = use_signal(|| false);
    use_effect(move || {
        if price3() {
            price2.set(true);
        }
    });
    use_effect(move || {
        if price2() {
            price1.set(true);
        }
    });
    use_effect(move || {
        if !price2() {
            price3.set(false);
        }
    });
    use_effect(move || {
        if !price1() {
            price2.set(false);
        }
    });

    use_effect(move || {
        let r = {
            if price3() {
                3
            } else if price2() {
                2
            } else if price1() {
                1
            } else {
                0
            }
        };
        price.set(r)
    });

    rsx! {
        div {
            class: "ActiveRatingsBar",
            div {
                class: "RatingsBar",
                display: "flex",
                Star {
                    selected: star1
                }
                Star {

                    selected: star2
                }
                Star {

                    selected: star3
                }
                Star {

                    selected: star4
                }
                Star {

                    selected: star5
                }
            }
            div {
                class: "PriceBar",
                display: "flex",
                Price {
                    selected: price1
                }
                Price {
                    selected: price2
                }
                Price {
                    selected: price3
                }
            }
        }
    }
}

#[component]
pub fn RawRatingsBar(stars: f32, price: f32) -> Element {
    let full_stars = stars as i16;
    let half_star = stars.fract() > 0.5;
    let empty_stars = 5 - full_stars - half_star as i16;
    let full_price = price as i16;
    let empty_price = 3 - full_price;

    rsx! {
        div {
            class: "RatingsBar",
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

#[component]
pub fn RatingsBar(restaurant_id: Uuid, ratings: Vec<Rating>) -> Element {
    let stars = ratings.iter().fold(0, |acc, rating| acc + rating.stars) as f32
        / ratings.iter().len() as f32;

    let price = ratings.iter().fold(0, |acc, rating| acc + rating.price) as f32
        / ratings.iter().len() as f32;

    rsx! {
        RawRatingsBar {stars, price}
    }
}
