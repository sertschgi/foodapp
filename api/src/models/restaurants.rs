use super::deps::*;

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = restaurants)]
pub struct Restaurant {
    pub id: Uuid,
    pub name: String,
    pub location: Point,
    pub picture: Vec<u8>,
}

#[derive(Insertable)]
#[diesel(table_name = restaurants)]
pub struct NewRestaurant {
    pub name: String,
    pub location: Point,
    pub picture: Vec<u8>,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(belongs_to(Restaurant))]
#[diesel(table_name = ratings)]
pub struct Rating {
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub stars: i16,
    pub price: i16,
    pub rating: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = ratings)]
pub struct NewRating {
    pub restaurant_id: Uuid,
    pub stars: i16,
    pub price: i16,
    pub rating: Option<String>,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = favourites)]
pub struct Favourite {
    pub user_id: Uuid,
    pub restaurant_id: Uuid,
}
