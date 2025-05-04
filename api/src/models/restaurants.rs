use super::deps::*;

#[derive(Queryable, Identifiable, Selectable, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[diesel(table_name = restaurants)]
pub struct Restaurant {
    pub id: Uuid,
    pub name: String,
    pub location: Point,
    pub picture: Option<Vec<u8>>,
}

#[derive(Insertable)]
#[diesel(table_name = restaurants)]
pub struct NewRestaurant {
    pub name: String,
    pub location: Point,
    pub picture: Vec<u8>,
}

#[derive(Queryable, Identifiable, Selectable, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[diesel(belongs_to(Restaurant))]
#[diesel(table_name = ratings)]
pub struct Rating {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub stars: i16,
    pub price: i16,
    pub rating: Option<String>,
    pub author: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = ratings)]
pub struct NewRating {
    pub restaurant_id: Uuid,
    pub stars: i16,
    pub price: i16,
    pub author: String,
    pub rating: Option<String>,
}

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = favourites)]
pub struct Favourite {
    pub id: Uuid,
    pub user_id: Uuid,
    pub restaurant_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = favourites)]
pub struct NewFavourite {
    pub user_id: Uuid,
    pub restaurant_id: Uuid,
}
