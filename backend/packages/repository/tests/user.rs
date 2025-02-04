#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Utc;
    use database::database::{DatabaseClient, SurrealDb};
    use environment::Environment;
    use errors::Result;
    use repository::user::user_repository::{UserRepository, UserRepositoryTrait};
    use serde::{Deserialize, Serialize};
    use surrealdb::engine::remote::ws::{Client, Ws};
    use surrealdb::opt::auth::Root;
    use surrealdb::Surreal;
    use tokio::test;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct UserUpdatedUserName {
        pub username: String,
        pub password: String,
        pub role: String,
        pub email: String,
    }

    async fn setup_direct_db() -> Result<Surreal<Client>> {
        let env = Environment::new();
        let hostname = format!("{}:{}", env.db_host, env.db_port);
        let client = Surreal::new::<Ws>(hostname).await?;

        client
            .signin(Root {
                username: &env.db_user,
                password: &env.db_pass,
            })
            .await?;

        client.use_ns(env.db_namespace).use_db(env.db_name).await?;

        Ok(client)
    }

    async fn setup_user_repo() -> Result<UserRepository> {
        let user_repo = UserRepository {
            db: Arc::new(DatabaseClient::Surreal(SurrealDb {
                client: Some(setup_direct_db().await?),
            })),
        };
        Ok(user_repo)
    }

    #[test]
    async fn test_insert_data() -> Result<()> {
        let user_repo = setup_user_repo().await.unwrap();
        let user = model::domain::user::User {
            id: "1".to_string(),
            username: "test".to_string(),
            password: "test".to_string(),
            role: "buyer".to_string(),
            email: "test@email.test".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = user_repo.insert_data(user).await.unwrap();
        assert_eq!(result, "1".to_string());

        setup_direct_db()
            .await
            .unwrap()
            .query("delete from  user where id = user:1")
            .await?;
        Ok(())
    }
    #[test]
    async fn test_is_data_empty_by_username() -> Result<()> {
        let user_repo = setup_user_repo().await.unwrap();
        setup_direct_db()
            .await
            .unwrap()
            .query(
                r#"
            -- Create a new record with a numeric id
            CREATE user:user_12347 CONTENT {
                username: 'Tobies7',
                password: 'password',
                role: 'buyer',
                email: '',
                created_at: time::now(),
                updated_at: time::now()
            };
        "#,
            )
            .await?;

        let is_empty = user_repo
            .is_data_empty_by_username("Tobies7")
            .await
            .unwrap();
        assert_eq!(is_empty, false);
        Ok(())
    }
    #[test]
    async fn test_update_data() -> Result<()> {
        let user_repo = setup_user_repo().await.unwrap();
        setup_direct_db()
            .await
            .unwrap()
            .query(
                r#"
            -- Create a new record with a numeric id
            CREATE user:user_12345 CONTENT {
                username: 'Tobie',
                password: 'password',
                role: 'buyer',
                email: 'asoi@gmail.co'
            };
        "#,
            )
            .await?;

        let updated_user = serde_json::json!({
            "username": "John Doe",
            "password": "password123"
        });

        let data_updated = user_repo
            .update_data("user:user_12345", updated_user)
            .await?;
        assert_eq!(data_updated, true);

        let result: Option<UserUpdatedUserName> = setup_direct_db()
            .await
            .unwrap()
            .query("select * from user where id = user:user_12345")
            .await?
            .take(0)?;

        assert_eq!(result.clone().unwrap().username, "John Doe");
        assert_eq!(result.clone().unwrap().password, "password123");
        assert_eq!(result.clone().unwrap().role, "buyer");
        assert_eq!(result.clone().unwrap().email, "asoi@gmail.co");

        setup_direct_db()
            .await
            .unwrap()
            .query("delete from  user where id = user:user_12345")
            .await?;
        Ok(())
    }
}
