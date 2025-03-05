use async_trait::async_trait;
use database::interface::DBInterface;
use model::domain::store::Store;

use super::store_repository::{StoreRepository, StoreRepositoryTrait};
use errors::{Error::DataDuplicationError, Result};

use model::surreal_db::store::Store as SurrealStore;
use serde_json::Value;

#[async_trait]
impl StoreRepositoryTrait for StoreRepository {
    async fn insert_data(&self, data: Store) -> Result<bool> {
        let result: Option<SurrealStore> = self.db.insert_record("store", data).await?;

        if result.is_some() {
            Ok(true)
        } else {
            Err(DataDuplicationError("id".to_string()))
        }
    }
    async fn get_by_user_id(&self, user_id: &str) -> Result<Vec<Store>> {
        let filter = format!("user_id={}", user_id);

        let stores: Vec<Store> = self.db.select_where("store", &filter, "*").await?;
        Ok(stores)
    }
    async fn get_by_id(&self, id: &str) -> Result<Option<Store>> {
        let filter = format!("id={}", id);

        let stores: Option<Store> = self
            .db
            .select_where("store", &filter, "*")
            .await?
            .first()
            .cloned();
        Ok(stores)
    }
    async fn delete_data(&self, id: &str) -> Result<bool> {
        self.db.delete(id).await
    }
    async fn update_data(&self, id: &str, data: Value) -> Result<bool> {
        self.db.update_record(id, "store", data).await
    }
}
