use async_trait::async_trait;

use super::user_repository::{UserRepository, UserRepositoryTrait};
use database::interface::DBInterface as _;

use errors::{
    Error::{self, DataNotAvaliable},
    Result,
};

use model::{domain::user::User, surreal_db::user::User as UserSurreal};

use serde_json::Value;
use tracing;

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    #[tracing::instrument(err, skip_all)]
    async fn insert_data(&self, data: User) -> Result<String> {
        let result: Option<UserSurreal> = self.db.insert_record("user", data).await?;
        let id = result
            .ok_or(DataNotAvaliable("id".to_string()))?
            .id
            .ok_or(DataNotAvaliable("id".to_string()))?
            .id
            .to_string()
            .replace("⟨", "")
            .replace("⟩", "");
        Ok(id)
    }

    #[tracing::instrument(err, skip_all)]
    async fn update_data(&self, id: &str, data: Value) -> Result<bool> {
        let result: bool = self.db.update_record(id, "user", data).await?;
        Ok(result)
    }
}

impl UserRepository {
    #[tracing::instrument(err, skip_all)]
    pub async fn is_data_empty_by_username(&self, username: &str) -> Result<bool> {
        let db = &self.db;

        let data: Vec<Value> = db
            .select_where("user", &format!("username = '{}'", username), "username")
            .await?;

        Ok(data.is_empty())
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn is_data_empty_by_email(&self, email: &str) -> Result<bool> {
        let db = &self.db;

        let data: Vec<Value> = db
            .select_where("user", &format!("email = '{}'", email), "email")
            .await?;

        Ok(data.is_empty())
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn is_data_empty_by_id(&self, id: &str) -> Result<bool> {
        let db = &self.db;

        let data: Vec<Value> = db
            .select_where("user", &format!("id = {} ", id), "email")
            .await?;

        Ok(data.is_empty())
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn is_verified(&self, id: &str) -> Result<bool> {
        let db = &self.db;
        let is_id_empty = self.is_data_empty_by_id(id).await?;

        if is_id_empty {
            return Err(DataNotAvaliable(format!("id:{}", id)));
        }

        let data: Vec<User> = db
            .select_where("user", &format!("id = {} ", id), "*")
            .await?;

        let user = data
            .first()
            .cloned()
            .ok_or_else(|| DataNotAvaliable(format!("user with id {} not exists", id)))?;

        if user.verified {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn get_data_by_email(&self, email: &str) -> Result<User> {
        let db = &self.db;

        let data: Vec<User> = db
            .select_where("user", &format!("email = '{}'", email), "*")
            .await?;

        if data.len() > 1 {
            return Err(Error::DataDuplicationError(
                "found more than one".to_string(),
            ));
        }

        let user = data.first().cloned().ok_or_else(|| {
            let error_message = format!("user with email {} not exists", email);
            DataNotAvaliable(error_message)
        })?;

        Ok(user)
    }
}
