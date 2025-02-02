

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Utc;
    use database::database::{DatabaseClient, SurrealDb};
    use environment::Environment;
    use repository::user::user_repository::{UserRepository, UserRepositoryTrait};
    use tokio::test;
    use errors::Result;
    use surrealdb::engine::remote::ws::{Client, Ws};
    use surrealdb::opt::auth::Root;
    use surrealdb::Surreal;
    use surrealdb::sql::Thing;

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

        client
            .use_ns(env.db_namespace)
            .use_db(env.db_name)
            .await?;

        Ok(client)
    }

    async fn setup_user_repo() -> Result<UserRepository> {
        // Setup your UserRepository instance here
        // For example, you might want to connect to a test database
        let user_repo = UserRepository{
            db: Arc::new(DatabaseClient::Surreal(
                SurrealDb{
                    client: Some(setup_direct_db().await?)
                }
            ))
        };
        Ok(user_repo)
    }

    #[test]
    async fn test_insert_data() -> Result<()>{
        let user_repo = setup_user_repo().await.unwrap();
        let user = model::domain::user::User{
            id: 1,
            username: "test".to_string(),
            password: "test".to_string(),
            role:"buyer".to_string(),
            email: "test@email.test".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = user_repo.insert_data(user).await.unwrap();
        assert_eq!(result, "1".to_string());

        setup_direct_db().await.unwrap().query("delete from  user where id = user:1").await?;
        Ok(())
    }

}