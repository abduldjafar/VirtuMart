use super::user_service::{UserService, UserServiceTrait};
use async_trait::async_trait;
use errors::Result;
use model::domain::user::User;
use repository::user::user_repository::UserRepositoryTrait as _;
use serde_json::Value;

#[async_trait]
impl UserServiceTrait for UserService {
    async fn register_profile(&self, data: User) -> Result<String> {
        let is_empty_by_username = self
            .user_repo
            .is_data_empty_by_username(&data.username)
            .await?;
        let is_empty_by_email = self.user_repo.is_data_empty_by_email(&data.email).await?;

        if !is_empty_by_username {
            return Err(errors::Error::DataExist(format!(
                "username:{}",
                data.username
            )));
        }

        if !is_empty_by_email {
            return Err(errors::Error::DataExist(format!("email:{}", data.email)));
        }

        self.user_repo.insert_data(data).await
    }

    async fn update_profile(&self, id: &str, data: Value) -> Result<bool> {
        self.user_repo.update_data(id, data).await
    }
}
