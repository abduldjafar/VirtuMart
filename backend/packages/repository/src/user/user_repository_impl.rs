use async_trait::async_trait;
use serde_json::Value;
use tracing;

use super::user_repository::{UserRepository, UserRepositoryTrait};
use database::interface::DBInterface as _;
use errors::{
    Error::{DataDuplicationError, DataNotAvailable},
    Result,
};
use model::{domain::user::User, surreal_db::user::User as UserSurreal};

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    #[tracing::instrument(err, skip_all)]
    async fn insert_data(&self, data: User) -> Result<String> {
        let result: Option<UserSurreal> = self.db.insert_record("user", data).await?;

        result
            .and_then(|user| user.id.map(|id| id.id.to_string()))
            .ok_or_else(|| DataNotAvailable("id".to_string()))
            .map(|id| id.replace("⟨", "").replace("⟩", ""))
    }

    #[tracing::instrument(err, skip_all)]
    async fn update_data(&self, id: &str, data: Value) -> Result<bool> {
        self.db.update_record(id, "user", data).await
    }
}

impl UserRepository {
    async fn is_data_empty(&self, field: &str, value: &str) -> Result<bool> {
        let data: Vec<Value> = self
            .db
            .select_where("user", &format!("{} = '{}'", field, value), field)
            .await?;

        Ok(data.is_empty())
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn is_data_empty_by_username(&self, username: &str) -> Result<bool> {
        self.is_data_empty("username", username).await
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn is_data_empty_by_email(&self, email: &str) -> Result<bool> {
        self.is_data_empty("email", email).await
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn is_data_empty_by_id(&self, id: &str) -> Result<bool> {
        self.is_data_empty("id", id).await
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn is_verified(&self, id: &str) -> Result<bool> {
        if self.is_data_empty_by_id(id).await? {
            return Err(DataNotAvailable(format!("id:{}", id)));
        }

        let data: Vec<User> = self
            .db
            .select_where("user", &format!("id = {}", id), "*")
            .await?;

        data.first()
            .cloned()
            .map(|user| user.verified)
            .ok_or_else(|| DataNotAvailable(format!("user with id {} not exists", id)))
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn get_data_by_email(&self, email: &str) -> Result<User> {
        let data: Vec<User> = self
            .db
            .select_where("user", &format!("email = '{}'", email), "*")
            .await?;

        match data.len() {
            0 => Err(DataNotAvailable(format!(
                "user with email {} not exists",
                email
            ))),
            1 => Ok(data.into_iter().next().unwrap()),
            _ => Err(DataDuplicationError("found more than one".to_string())),
        }
    }
}
