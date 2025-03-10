/* Define the SurrealDb struct */
use async_trait::async_trait;

use environment::Environment;
use errors::Result;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

#[derive(Clone, Debug)]
pub struct SurrealDb {
    pub client: Option<Surreal<Client>>,
}

pub enum DatabaseType {
    SurrealDB,
}

#[derive(Clone, Debug)]
pub enum DatabaseClient {
    Surreal(SurrealDb),
    // Add other database clients here, e.g., Postgres(PostgresDb)
}

/* Define the DatabaseSource struct */
pub struct DatabaseSource {
    pub db_type: DatabaseType,
}

/* Trait for initializing a database connection */
#[async_trait]
pub trait Initializable {
    async fn init(&self) -> Result<DatabaseClient>;
}

/* Trait for generic database connection operations */
#[async_trait]
pub trait Connection {
    fn ping(&self) -> String;
}

/* Trait for sources to connect to a database */
#[async_trait]
pub trait Sources {
    async fn connect(&mut self) -> Result<DatabaseClient>;
}

#[async_trait]
impl Initializable for SurrealDb {
    async fn init(&self) -> Result<DatabaseClient> {
        let env = Environment::new();
        let hostname = format!("{}:{}", env.db_host, env.db_port);
        let temp_client = Surreal::new::<Ws>(hostname).await?;

        temp_client
            .signin(Root {
                username: &env.db_user,
                password: &env.db_pass,
            })
            .await?;

        temp_client
            .use_ns(env.db_namespace)
            .use_db(env.db_name)
            .await?;

        let client = Some(temp_client);
        Ok(DatabaseClient::Surreal(SurrealDb { client }))
    }
}

impl Connection for SurrealDb {
    fn ping(&self) -> String {
        if let Some(_client) = &self.client {
            String::from("Pong!")
        } else {
            String::from("Connection Failed")
        }
    }
}

/* Implementation of Connection for DatabaseClient */
impl Connection for DatabaseClient {
    fn ping(&self) -> String {
        match self {
            DatabaseClient::Surreal(surrealdb) => surrealdb.ping(),
            // Add other database client pings here
        }
    }
}

/* Implementation of Sources for DatabaseSource */
#[async_trait]
impl Sources for DatabaseSource {
    async fn connect(&mut self) -> Result<DatabaseClient> {
        match &self.db_type {
            DatabaseType::SurrealDB => {
                let surrealdb = SurrealDb { client: None };
                surrealdb.init().await
            } // Add other database types here
        }
    }
}
