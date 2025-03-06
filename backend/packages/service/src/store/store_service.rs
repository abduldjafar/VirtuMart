use async_trait::async_trait;
use errors::Result;
use model::web::store::{store_request::Store, store_response::Store as StoreResponse};
use repository::store::store_repository::StoreRepository;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct StoreService {
    pub store_repo: StoreRepository,
}

#[async_trait]
pub trait StoreServiceTrait {
    async fn register_store(&self, data: Store) -> Result<StoreResponse>;
    async fn update_store(&self, id: &str, data: Value) -> Result<bool>;
    async fn delete_store(&self, id: &str) -> Result<bool>;
    async fn get_store_by_user_id(&self, user_id: &str) -> Result<Vec<StoreResponse>>;
    async fn get_store_by_id(&self, id: &str) -> Result<Option<StoreResponse>>;
}
