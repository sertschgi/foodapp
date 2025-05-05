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

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[diesel(table_name = users)]
pub struct UserInfo {
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_sessions)]
pub(crate) struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_info: String,
    pub ip_address: String,
    pub user_agent: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = user_sessions)]
pub(crate) struct NewUserSession {
    pub user_id: Uuid,
    pub ip_address: String,
    pub device_info: String,
    pub user_agent: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[diesel(table_name = user_sessions)]
pub struct LoginSession {
    pub id: Uuid,
    pub user_id: Uuid,
}

impl From<UserSession> for LoginSession {
    fn from(s: UserSession) -> Self {
        LoginSession {
            id: s.id,
            user_id: s.user_id,
        }
    }
}
