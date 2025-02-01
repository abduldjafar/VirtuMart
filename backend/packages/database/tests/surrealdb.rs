use serde::{Deserialize, Serialize};
use errors::Result;

#[cfg(test)]
mod tests {
    use database::database::{Connection, DatabaseClient, Sources};
    use database::interface::DBInterface as _;
    use environment::Environment;
    use surrealdb::engine::remote::ws::{Client, Ws};
    use surrealdb::opt::auth::Root;
    use surrealdb::Surreal;
    use super::*;
    use surrealdb::sql::Thing;

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct TestRecord {
        id: String,
        name: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct UpdateTestRecord {
        name: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct ResultTestRecord {
        id: Option<Thing>,
        name: String,
    }

    async fn setup_db() -> Result<DatabaseClient> {
        let mut surreal_db = database::database::DatabaseSource {
            db_type: database::database::DatabaseType::SurrealDB,
        };
        
        let data = surreal_db.connect().await?;
        Ok(data)
    }

    async fn direct_db() -> Result<Surreal<Client>>{
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

    #[tokio::test]
    async fn test_connection() -> Result<()> {
        let db = setup_db().await?;
        let result = db.ping();

        assert_eq!(result, "Pong!");
        Ok(())
    }

    #[tokio::test]
    async fn test_insert_record() -> Result<()> {
        let db = setup_db().await?;
        let record = TestRecord {
            id: "1".to_string(),
            name: "Test".to_string(),
        };

        let insert_result:Option<ResultTestRecord> = db.insert_record("test_insert_table", record).await?;
        assert!(insert_result.is_some());

        direct_db().await?.query("DELETE test_insert_table").await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_update_record() -> Result<()> {
        let db = setup_db().await?;
        
        direct_db().await?.query("INSERT INTO test_update_table (id, name) VALUES ('2', 'Test')").await?;

        let updated_record = UpdateTestRecord {
            name: "Test2".to_string(),
        };

        let success = db.update_record("2","test_update_table",updated_record).await?;
        
        assert_eq!(success, true);

        direct_db().await?.query("DELETE test_update_table").await?;

        Ok(())
    }
}