use super::user_repository::{UserRepository, UserRepositoryTrait};
use async_trait::async_trait;
use database::interface::DBInterface as _;
use errors::Result;
use model::domain::user::User;
use model::surreal_db::user::User as UserSurreal;
use serde_json::Value;

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn insert_data(&self, data: User) -> Result<String> {
        let result: Option<UserSurreal> = self.db.insert_record("user", data).await?;
        Ok(result.unwrap().id.unwrap().id.to_string())
    }

    async fn update_data(&self, id: &str, data: Value) -> Result<bool> {
        let result: bool = self.db.update_record(id, "user", data).await?;
        Ok(result)
    }
}
