use model::domain::store::Store;
use async_trait::async_trait;

use super::store_repository::{StoreRepository, StoreRepositoryTrait};
use errors::{
    Error::{DataDuplicationError, DataNotAvailable},
    Result,
};

use serde_json::Value;


#[async_trait]
impl StoreRepositoryTrait for StoreRepository {
    async fn insert_data(&self, data: Store) -> Result<bool> {
        let _ = data;
        // Implement the logic to insert data into the store table
       unimplemented!()
    }
    async fn get_by_user_id(&self, user_id: &str) -> Result<Vec<Store>>{
        // Implement the logic to retrieve stores by user_id
       unimplemented!()
    }
    async fn get_by_id(&self, id: &str) -> Result<Option<Store>>{
        // Implement the logic to retrieve store by id
       unimplemented!()
    }
    async fn delete_data(&self, id: &str) -> Result<bool>{
        // Implement the logic to delete store by id
       unimplemented!()
    }
    async fn update_data(&self, id: &str, data: Value) -> Result<bool>{
        unimplemented!()
    }
} 