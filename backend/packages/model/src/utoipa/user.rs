use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug,ToSchema)]
pub struct User {
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
    #[serde(deserialize_with = "datetime_to_string")]
    pub created_at: String,
    #[serde(deserialize_with = "datetime_to_string")]
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponseUsername {
    username: String,
}

fn thing_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let t = Thing::deserialize(deserializer)?;
    Ok(t.to_raw())
}


fn datetime_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let t = DateTime::<Utc>::deserialize(deserializer)?;
    Ok(t.to_string())
}