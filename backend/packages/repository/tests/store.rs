#[cfg(test)]
mod tests {
    use chrono::Utc;
    use common::{cleanup_data, execute_sql, setup_direct_db};
    use database::database::{DatabaseClient, SurrealDb};
    use errors::Result;
    use repository::store::store_repository::{StoreRepository, StoreRepositoryTrait};
    use surrealdb::sql::Thing;
    use tokio::test;

    use crate::setup_repo_with_surreal;
    mod common;

    setup_repo_with_surreal!(store_repo, StoreRepository, db);

    #[test]
    async fn test_insert_data() -> Result<()> {
        let user_id = Thing::from(("user", "_12341"));
        let store_repo = store_repo().await?;
        let store = model::domain::store::Store {
            id: "store_12341".to_string(),
            user_id,
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
        cleanup_data("store:store_12341", "store").await?;

        Ok(())
    }

    #[test]
    async fn test_get_by_user_id() -> Result<()> {
        execute_sql(
            r#"CREATE user:user_1_2_3 CONTENT {
                username: 'Tobies7',
                password: 'password',
                role: 'buyer',
                email: '',
                verified: false,
                created_at: time::now(),
                updated_at: time::now()
            };"#,
        )
        .await?;

        execute_sql(
            r#"CREATE store:store_12347 CONTENT {
                user_id: user:user_1_2_3,
                name: 'Test Store',
                description: 'Test Description',
                address: 'Test Address',
                phone_number: '1234567890',
                latitude: 1.0,
                longitude: 1.0,
                created_at: time::now(),
                updated_at: time::now()
            };"#,
        )
        .await?;

        execute_sql(
            r#"CREATE store:store_12348 CONTENT {
                user_id: user:user_1_2_3,
                name: 'Test Store',
                description: 'Test Description',
                address: 'Test Address',
                phone_number: '1234567890',
                latitude: 1.0,
                longitude: 1.0,
                created_at: time::now(),
                updated_at: time::now()
            };"#,
        )
        .await?;

        let store_repo = store_repo().await?;
        let stores = store_repo.get_by_user_id("user:user_1_2_3").await?;
        assert_eq!(stores.len(), 2);
        cleanup_data("store:store_12347", "store").await?;
        cleanup_data("store:store_12348", "store").await?;
        cleanup_data("user:user_1_2_3", "user").await?;
        Ok(())
    }

    #[test]
    async fn test_get_by_id() -> Result<()> {
        execute_sql(
            r#"CREATE user:user_1_2_4 CONTENT {
                username: 'Tobies7',
                password: 'password',
                role: 'buyer',

                email: '',
                verified: false,
                created_at: time::now(),
                updated_at: time::now()
                };"#,
        )
        .await?;
        execute_sql(
            r#"CREATE store:store_12349 CONTENT {
                user_id: user:user_1_2_4,
                name: 'Test Store',
                description: 'Test Description',
                address: 'Test Address',
                phone_number: '1234567890',
                latitude: 1.0,
                longitude: 1.0,
                created_at: time::now(),
                updated_at: time::now()
                };"#,
        )
        .await?;
        let store_repo = store_repo().await?;
        let store = store_repo.get_by_id("store:store_12347").await?;
        assert_eq!(store.unwrap().name, "Test Store");
        cleanup_data("store:store_12347", "store").await?;
        cleanup_data("user:user_1_2_3", "user").await?;
        Ok(())
    }

    #[test]
    async fn test_update_data() -> Result<()> {
        execute_sql(
            r#"CREATE store:store_123491 CONTENT {
                user_id: user:user_1_2_4_5,
                name: 'Test Store',
                description: 'Test Description',
                address: 'Test Address',
                phone_number: '1234567890',
                latitude: 1.0,
                longitude: 1.0,
                created_at: time::now(),
                updated_at: time::now()
                };"#,
        )
        .await?;

        let store_repo = store_repo().await?;
        let updated_store = serde_json::json!({ "name": "John Doe", "address": "password123" });
        assert!(
            store_repo
                .update_data("store:store_123491", updated_store)
                .await?
        );

        let result: Option<model::domain::store::Store> =
            execute_sql("SELECT * FROM store WHERE id = store:store_123491")
                .await?
                .take(0)?;
        assert!(result.is_some(), "Expected result to be Some, but got None");
        let result = result.unwrap();
        assert_eq!(result.name, "John Doe");
        assert_eq!(result.address, "password123");
        cleanup_data("store:store_123491", "store").await?;

        Ok(())
    }

    #[test]
    async fn test_delete_data() -> Result<()> {
        execute_sql(
            r#"CREATE store:store_123492 CONTENT {
                user_id: user:user_1_2_4_6,
                name: 'Test Store',
                description: 'Test Description',
                address: 'Test Address',
                phone_number: '1234567890',
                latitude: 1.0,
                longitude: 1.0,
                created_at: time::now(),
                updated_at: time::now()
                };"#,
        )
        .await?;
        let store_repo = store_repo().await?;
        let is_delete = store_repo.delete_data("store:store_123492").await?;
        assert!(is_delete);

        Ok(())
    }
}
