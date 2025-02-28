use async_trait::async_trait;
use errors::Result;

use model::web::user::{user_request::User, user_response::User as UserResponse};

use repository::user::user_repository::UserRepository;

use serde_json::Value;

#[derive(Clone, Debug)]
pub struct UserService {
    pub user_repo: UserRepository,
}

#[async_trait]
pub trait UserServiceTrait {
    async fn register_profile(&self, data: User) -> Result<UserResponse>;
    async fn update_profile(&self, id: &str, data: Value) -> Result<bool>;
}
