use errors::Result;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*;
    use database::database::{DatabaseClient, Sources};
    use database::interface::DBInterface as _;
    use environment::Environment;
    use surrealdb::engine::remote::ws::{Client, Ws};
    use surrealdb::opt::auth::Root;
    use surrealdb::sql::Thing;
    use surrealdb::Surreal;
    use tokio::test;

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

    #[test]
    async fn test_insert_record() -> Result<()> {
        let db = setup_db().await?;
        let record = TestRecord {
            id: "1".to_string(),
            name: "Test".to_string(),
        };

        let insert_result: Option<ResultTestRecord> =
            db.insert_record("test_insert_table", record).await?;
        assert!(insert_result.is_some());

        setup_direct_db()
            .await?
            .query("DELETE test_insert_table")
            .await?;

        Ok(())
    }

    #[test]
    async fn test_update_record() -> Result<()> {
        let db = setup_db().await?;

        setup_direct_db()
            .await?
            .query("INSERT INTO test_update_table (id, name) VALUES ('2', 'Test')")
            .await?;

        let updated_record = UpdateTestRecord {
            name: "Test2".to_string(),
        };

        let success = db
            .update_record("2", "test_update_table", updated_record)
            .await?;
        assert!(success);

        setup_direct_db()
            .await?
            .query("DELETE test_update_table")
            .await?;

        Ok(())
    }

    #[test]
    async fn test_select_records() -> Result<()> {
        let db = setup_db().await?;

        setup_direct_db()
            .await?
            .query("INSERT INTO test_select_table (id, name) VALUES ('3', 'Test')")
            .await?;

        let records: Vec<ResultTestRecord> = db.select("test_select_table").await?;
        assert_eq!(records.len(), 1);

        setup_direct_db()
            .await?
            .query("DELETE test_select_table")
            .await?;

        Ok(())
    }

    #[test]
    async fn test_delete_record() -> Result<()> {
        let db = setup_db().await?;

        setup_direct_db()
            .await?
            .query("INSERT INTO test_delete_table (id, name) VALUES ('4', 'Test')")
            .await?;

        let success = db.delete("test_delete_table:4").await?;
        assert!(success);

        Ok(())
    }

    #[test]
    async fn test_select_where_records() -> Result<()> {
        let db = setup_db().await?;

        let direct_db = setup_direct_db().await?;
        direct_db
            .query("INSERT INTO test_select_where_table (id, name) VALUES ('5', 'Test')")
            .await?;
        direct_db
            .query("INSERT INTO test_select_where_table (id, name) VALUES ('6', 'Test2')")
            .await?;

        let records: Vec<ResultTestRecord> = db
            .select_where("test_select_where_table", "name='Test'", "name")
            .await?;

        assert_eq!(records.len(), 1);
        setup_direct_db()
            .await?
            .query("DELETE FROM test_select_where_table")
            .await?;

        Ok(())
    }
}
