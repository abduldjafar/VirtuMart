#[cfg(test)]
mod tests {
    use chrono::Utc;
    use common::setup_direct_db;
    use database::database::{DatabaseClient, SurrealDb};
    use environment::Environment;
    use errors::{Error::DataNotAvailable, Result};
    use repository::store::store_repository::{StoreRepository, StoreRepositoryTrait};
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;
    use surrealdb::{
        engine::remote::ws::{Client, Ws},
        opt::auth::Root,
        Surreal,
    };
    use tokio::test;

    use crate::setup_repo_with_surreal;
    mod common;


    setup_repo_with_surreal!(setup_store_repo, StoreRepository, db);

    #[test]
    async fn test_insert_data() -> Result<()> {
        let store_repo = setup_store_repo().await?;
        let store = model::domain::store::Store {
            id: "store_12341".to_string(),
            user_id: "user_12341".to_string(),
            name: "Test Store".to_string(),
            description: "Test Description".to_string(),
            address: "Test Address".to_string(),
            phone_number: "1234567890".to_string(),
            latitude: Some(1.0),
            longitude: Some(1.0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let is_stored = store_repo.insert_data(store).await?;
        assert_eq!(is_stored, true);

        Ok(())
    }


}