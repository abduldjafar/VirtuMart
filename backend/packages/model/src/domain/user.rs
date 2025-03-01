use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

static RE_USER_PREFIX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^user_[a-z]{2}$").unwrap());

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct User {
    #[validate(regex(path = *RE_USER_PREFIX))]
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    pub password: String,
    pub verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn thing_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let t = Thing::deserialize(deserializer)?;
    Ok(t.to_raw())
}
