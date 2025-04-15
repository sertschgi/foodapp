use super::deps::*;

#[derive(Insertable, Selectable, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = restaurants)]
pub struct Restaurant {
    #[diesel(skip_insertion)]
    pub id: Uuid,
    pub name: String,
    pub location: Point,
}

#[derive(Selectable, Queryable, Identifiable, Serialize, Deserialize)]
// #[diesel(belongs_to(Restaurant))]
// #[diesel(table_name = ratings)]
pub struct Rating {
    #[diesel(skip_insertion)]
    pub id: Uuid,
    pub restaurant_id: Uuid,
    pub stars: i16,
    pub price: i16,
    pub rating: Option<String>,
    // #[diesel(skip_insertion)]
    // pub created_at: DateTime<Utc>,
}
