use super::user_repository::{UserRepository, UserRepositoryTrait};
use async_trait::async_trait;
use database::interface::DBInterface as _;
use errors::Error::{self, DataNotAvaliable};
use errors::Result;
use model::domain::user::User;
use model::surreal_db::user::User as UserSurreal;
use serde_json::Value;

#[async_trait]
impl UserRepositoryTrait for UserRepository {
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

    async fn update_data(&self, id: &str, data: Value) -> Result<bool> {
        let result: bool = self.db.update_record(id, "user", data).await?;
        Ok(result)
    }
}

impl UserRepository {
    pub async fn is_data_empty_by_username(&self, username: &str) -> Result<bool> {
        let db = &self.db;

        let data: Vec<Value> = db
            .select_where("user", &format!("username = '{}'", username), "username")
            .await?;

        Ok(data.is_empty())
    }

    pub async fn is_data_empty_by_email(&self, email: &str) -> Result<bool> {
        let db = &self.db;

        let data: Vec<Value> = db
            .select_where("user", &format!("email = '{}'", email), "email")
            .await?;

        Ok(data.is_empty())
    }

    pub async fn is_data_empty_by_id(&self, id: &str) -> Result<bool> {
        let db = &self.db;

        let data: Vec<Value> = db
            .select_where("user", &format!("id = {} ", id), "email")
            .await?;

        Ok(data.is_empty())
    }

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
            errors::Error::DataNotAvaliable(format!("user with email {} not exists", email))
        })?;

        Ok(user)
    }
}
