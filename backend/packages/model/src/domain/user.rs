use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use surrealdb::sql::Thing;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
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
