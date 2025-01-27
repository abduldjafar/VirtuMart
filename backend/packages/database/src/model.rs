use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/* Struct representing a User in the database */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub id: Option<String>,
    /// Username of the user
    pub username: String,
    /// Type of the user (e.g., admin, regular user)
    pub role: String,
    /// Email of the user
    pub email: String,
    /// Timestamp when the user was created
    pub created_at: Option<DateTime<Utc>>,
    /// Timestamp when the user was last updated
    pub updated_at: Option<DateTime<Utc>>,
    /// Password of the user
    pub password: String,
    /// Whether the user is verified
    pub verified: bool,
    /// Verification token for the user
    pub verified_token: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PayloadUserResponse {
    /// Primary Key of the user
    pub id: String,
    /// Username of the user
    pub username: String,
    /// Type of the user (e.g., admin, regular user)
    pub role: String,
    /// Email of the user
    pub email: String,
    /// Password of the user
    pub password: String,
    /// Whether the user is verified
    pub verified: bool,
    /// Verification token for the user (optional)
    pub verified_token: Option<String>,
    /// Timestamp when the user was created
    pub created_at: Option<DateTime<Utc>>,
    /// Timestamp when the user was last updated
    pub updated_at: Option<DateTime<Utc>>,
}