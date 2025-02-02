use async_trait::async_trait;
use database::database::DatabaseClient;
use errors::Result;
use model::domain::user::User;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct UserRepository {
    pub db: Arc<DatabaseClient>,
}

#[async_trait]
pub trait UserRepositoryTrait {
    async fn insert_data(&self, data: User) -> Result<String>;
    async fn update_data(&self, id: &str, data: Value) -> Result<bool>;
}
