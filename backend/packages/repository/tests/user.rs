#[cfg(test)]
mod tests {
    use chrono::Utc;
    use common::{cleanup_user, execute_sql, setup_direct_db};
    use database::database::{DatabaseClient, SurrealDb};
    use errors::{Error::DataNotAvailable, Result};
    use repository::user::user_repository::{UserRepository, UserRepositoryTrait};
    use serde::{Deserialize, Serialize};

    use tokio::test;

    use crate::setup_repo_with_surreal;

    mod common;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct UserUpdatedUserName {
        pub username: String,
        pub password: String,
        pub role: String,
        pub email: String,
    }

    setup_repo_with_surreal!(setup_user_repo, UserRepository, db);

    #[test]
    async fn test_insert_data() -> Result<()> {
        let user_repo = setup_user_repo().await?;
        let user = model::domain::user::User {
            id: "user_12341".to_string(),
            username: "test".to_string(),
            password: "test".to_string(),
            role: "buyer".to_string(),
            email: "test@email.test".to_string(),
            verified: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = user_repo.insert_data(user).await?;
        assert_eq!(result, "user_12341");
        cleanup_user("user:user_12341").await?;
        Ok(())
    }

    #[test]
    async fn test_is_data_empty_by_username() -> Result<()> {
        let user_repo = setup_user_repo().await?;
        execute_sql(
            r#"CREATE user:user_12347 CONTENT {
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
        assert!(!user_repo.is_data_empty_by_username("Tobies7").await?);
        cleanup_user("user:user_12347").await?;
        Ok(())
    }

    #[test]
    async fn test_is_data_empty_by_id() -> Result<()> {
        let user_repo = setup_user_repo().await?;
        execute_sql(
            r#"CREATE user:user_123478 CONTENT {
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
        assert!(!user_repo.is_data_empty_by_id("user:user_123478").await?);
        cleanup_user("user:user_123478").await?;
        Ok(())
    }

    #[test]
    async fn test_get_data_by_email() -> Result<()> {
        let user_repo = setup_user_repo().await?;
        execute_sql(
            r#"CREATE user:user_asoi CONTENT {
                username: 'Tobies7',
                password: 'password',
                role: 'buyer',
                email: 'kotekaman@gmail.com',
                verified: false,
                created_at: time::now(),
                updated_at: time::now()
            };"#,
        )
        .await?;

        assert_eq!(
            user_repo
                .get_data_by_email("kotekaman@gmail.com")
                .await?
                .email,
            "kotekaman@gmail.com"
        );

        assert!(user_repo
            .get_data_by_email("kotekaman@gmails.com")
            .await
            .is_err());
        cleanup_user("user:user_asoi").await?;
        Ok(())
    }

    #[test]
    async fn test_is_user_verified() -> Result<()> {
        let user_repo = setup_user_repo().await?;
        execute_sql(
            r#"CREATE user:user_asoi1 CONTENT {
                username: 'Tobies7',
                password: 'password',
                role: 'buyer',
                email: 'kotekaman@gmail.coms',
                verified: true,
                created_at: time::now(),
                updated_at: time::now()
            };"#,
        )
        .await?;

        assert!(user_repo.is_verified("user:user_asoi1").await?);
        cleanup_user("user:user_asoi1").await?;
        Ok(())
    }

    #[test]
    async fn test_update_data() -> Result<()> {
        let user_repo = setup_user_repo().await?;
        //let db = setup_direct_db().await?;
        execute_sql(
            r#"CREATE user:user_12345 CONTENT {
                username: 'Tobie',
                password: 'password',
                role: 'buyer',
                email: 'asoi@gmail.co',
                verified: false,
                created_at: time::now(),
                updated_at: time::now()
            };"#,
        )
        .await?;

        let updated_user = serde_json::json!({ "username": "John Doe", "password": "password123" });
        assert!(
            user_repo
                .update_data("user:user_12345", updated_user)
                .await?
        );

        let result: Option<UserUpdatedUserName> =
            execute_sql("SELECT * FROM user WHERE id = user:user_12345")
                .await?
                .take(0)?;

        let result = result.ok_or(DataNotAvailable("Error extracting data".to_string()))?;
        assert_eq!(result.username, "John Doe");
        assert_eq!(result.password, "password123");
        assert_eq!(result.role, "buyer");
        assert_eq!(result.email, "asoi@gmail.co");

        cleanup_user("user:user_12345").await?;
        Ok(())
    }
}
