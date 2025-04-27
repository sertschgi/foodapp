use super::deps::*;

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub(crate) struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub salt: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub(crate) struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub salt: String,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_sessions)]
pub(crate) struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_info: String,
    pub ip_addr: IpNet,
    pub user_agent: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = user_sessions)]
pub(crate) struct NewUserSession {
    pub user_id: Uuid,
    pub ip_address: IpNet,
    pub device_info: String,
    pub user_agent: String,
    pub expires_at: DateTime<Utc>,
}
