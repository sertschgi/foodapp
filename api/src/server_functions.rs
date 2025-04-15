use crate::models::prelude::*;
use diesel::prelude::*;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::to_string as serialize;
use uuid::Uuid;

#[server(GetServerPing)]
pub async fn get_server_ping() -> Result<String, ServerFnError> {
    Ok("Ping from Server".to_string())
}

#[derive(Serialize, Deserialize)]
pub struct RestaurantItem {
    pub restaurant: Restaurant,
    pub ratings: Vec<Rating>,
}

#[cfg(feature = "server")]
async fn get_ratings_of_restaurant(
    restaurant_uuid: Uuid,
    manager: &deadpool_diesel::postgres::Object,
) -> Result<Vec<Rating>, ServerFnError> {
    use crate::schema::ratings::dsl::*;
    let result = manager
        .interact(move |c| {
            ratings
                .select(Rating::as_select())
                .filter(restaurant_id.eq(restaurant_uuid))
                .load::<Rating>(c)
                .expect("Could not query ratings!")
        })
        .await?;
    Ok(result)
}

#[server(GetRestaurants)]
pub async fn get_restaurants(query: String) -> Result<String, ServerFnError> {
    use crate::schema::restaurants::dsl::*;
    pub use axum::extract::*;
    pub use deadpool_diesel::postgres::Pool;
    dioxus::logger::tracing::debug!("Extension");
    let Extension(pool) = extract::<Extension<Pool>, _>().await?;
    let manager = pool.get().await?;
    let result = manager
        .interact(|c| {
            restaurants
                .filter(name.ilike(query))
                .load::<Restaurant>(c)
                .expect("Could not query restaurants!")
        })
        .await?;
    let mut items = Vec::<RestaurantItem>::new();
    for restaurant in result {
        let ratings = get_ratings_of_restaurant(restaurant.id, &manager).await?;
        items.push(RestaurantItem {
            restaurant,
            ratings,
        })
    }
    Ok(serialize(&items)?)
}
