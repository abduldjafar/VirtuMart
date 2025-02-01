use chrono::{DateTime, Utc};
pub struct  User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}