use super::deps::*;

#[derive(Insertable, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    #[diesel(skip_insertion)]
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub salt: String,
    #[diesel(skip_insertion)]
    pub created_at: DateTime<Utc>,
    #[diesel(skip_insertion)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_sessions)]
pub struct UserSession {
    #[diesel(skip_insertion)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_token: String,
    pub refresh_token: String,
    pub device_info: String,
    pub user_agent: String,
    pub is_active: bool,
    #[diesel(skip_insertion)]
    pub created_at: DateTime<Utc>,
    #[diesel(skip_insertion)]
    pub expires_at: DateTime<Utc>,
    #[diesel(skip_insertion)]
    pub last_accessed_at: DateTime<Utc>,
}
