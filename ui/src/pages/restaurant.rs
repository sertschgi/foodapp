use crate::icons::*;
use crate::prelude::{components::prelude::*, *};
use api::prelude::*;
use dioxus_storage::{use_synced_storage, LocalStorage};

const RESTAURANT_CSS: Asset = asset!("/assets/styling/pages/restaurant.css");

#[component]
pub fn Restaurant(#[props(into)] name: String) -> Element {
    let mut restaurant_id = use_signal(Uuid::default);
    let mut ratings = use_signal(Vec::<Rating>::new);
    let mut favourite = use_signal(|| false);
    let mut found = use_signal(|| false);
    let mut alternatives = use_signal(Vec::<String>::new);
    let mut stars = use_signal(|| 0);
    let mut price = use_signal(|| 0);

    let session =
        use_synced_storage::<LocalStorage, Option<LoginSession>>("session".into(), || {
            Option::<LoginSession>::None
        });

    let n1 = name.clone();
    let get_restaurant = use_signal(|| {
        move || {
            let n2 = n1.clone();
            async move {
                let result: Result<Vec<RestaurantItem>, _> = get_restaurants(n2, session()).await;
                if let Ok(items) = result {
                    if items.len() > 1 || items.is_empty() {
                        alternatives.set(items.iter().map(|i| i.restaurant.name.clone()).collect());
                    } else {
                        let item = items.first().unwrap();
                        restaurant_id.set(item.restaurant.id);
                        ratings.set(item.ratings.clone());
                        favourite.set(item.favourite);
                        found.set(true);
                    }
                }
            }
        }
    });

    let change = move |_| async move {
        get_restaurant()().await;
    };

    let new_rating = move |e: FormEvent| async move {
        let rating_value = e.values().get("rating").unwrap().as_value();
        let mut rating = None;
        if !rating_value.is_empty() {
            rating = Some(rating_value);
        }
        let request = RatingRequest {
            restaurant_id: restaurant_id(),
            rating,
            stars: stars(),
            price: price(),
        };
        if add_rating(request, session().unwrap()).await.is_ok() {
            get_restaurant()().await;
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: RESTAURANT_CSS }

        div {
            id: "Restaurant",
            onvisible: change,
            TopBar {
                title: &name,
                FavouriteButton {
                    restaurant_id: restaurant_id(),
                    favourite,
                }
            }
            if found() {
                div {
                    class: "wrapper",
                    img {
                        class: "image",
                        src: "https://dynamic-media-cdn.tripadvisor.com/media/photo-o/0f/b4/d6/77/die-gemutliche-alte-gaststube.jpg",
                    }
                }
                div {
                    class: "wrapper view",
                    div {
                        class: "wrapper info",

                        h1 {
                            id: "name",
                            {name}
                        }
                        RatingsBar {
                            restaurant_id: restaurant_id(),
                            ratings: ratings(),
                        }
                    }
                }
                section {
                    class: "wrapper",
                    h2 {
                        id: "ratings",
                        "Rezensionen"
                    }
                    if session().is_some() {
                        h3 {
                            "Eigene Rezension"
                        }
                        form {
                            id: "new_rating",
                            onsubmit: new_rating,
                            ActiveRatingsBar {
                                stars,
                                price,
                            }
                            textarea {
                                spellcheck: true,
                                autocomplete: true,
                                name: "rating"
                            }
                            button {
                                r#type: "submit",
                                cursor: "pointer",
                                class: "send",
                                SendIcon {}
                                "Abschicken"
                            }
                        }
                        h3 {
                            "Andere Rezensionen"
                        }
                }
                div {
                    id: "ratings_scroll",
                    for rating in ratings.iter() {
                        components::rating::Rating {
                            rating: rating.clone()
                        }
                    }
                }
                }
            }
            else {
                h1 {
                    id: "not_found_heading",
                    "Kein Restaurant unter diesem Namen"
                }
                if !alternatives().is_empty() {
                    p {
                        "Vielleicht meintest du:"
                        for alt in alternatives.iter() {
                            "{alt}"
                        }
                    }
                }
            }
        }
    }
}
