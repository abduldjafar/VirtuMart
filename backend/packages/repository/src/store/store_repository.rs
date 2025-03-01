use std::sync::Arc;

use async_trait::async_trait;

use database::database::DatabaseClient;

use errors::Result;

use model::domain::store::Store;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct StoreRepository {
    pub db: Arc<DatabaseClient>,
}

#[async_trait]
pub trait StoreRepositoryTrait {
    async fn insert_data(&self, data: Store) -> Result<bool>;
    async fn get_by_user_id(&self, user_id: &str) -> Result<Vec<Store>>;
    async fn get_by_id(&self, id: &str) -> Result<Option<Store>>;
    async fn delete_data(&self, id: &str) -> Result<bool>;
    async fn update_data(&self, id: &str, data: Value) -> Result<bool>;
}
