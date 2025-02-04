use async_trait::async_trait;
use errors::Result;
use model::domain::user::User;
use repository::user::user_repository::UserRepository;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct UserService {
    pub user_repo: UserRepository,
}

#[async_trait]
pub trait UserServiceTrait {
    async fn register_profile(&self, data: User) -> Result<String>;
    async fn update_profile(&self, id: &str, data: Value) -> Result<bool>;
}
