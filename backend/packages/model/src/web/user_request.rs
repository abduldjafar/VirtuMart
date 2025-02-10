use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

impl From<Json<User>> for User {
    fn from(payload: Json<User>) -> Self {
        Self {
            id: payload.id.clone(),
            username: payload.username.clone(),
            password: payload.password.clone(),
            role: payload.role.clone(),
            email: payload.email.clone(),
        }
    }
}
