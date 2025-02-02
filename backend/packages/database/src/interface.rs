use async_trait::async_trait;
use errors::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::database::DatabaseClient;

/* Trait for database interface operations */
#[async_trait]
pub trait DBInterface {
    /* Method to insert a record into the database */
    async fn insert_record<
        T: Serialize + Sync + Send + 'static,
        U: DeserializeOwned + Sync + Clone + 'static,
    >(
        &self,
        tb_name: &str,
        data: T,
    ) -> Result<Option<U>>;

    /* Method to select records from the database */
    async fn select<T: DeserializeOwned + Sync>(&self, tb_name: &str) -> Result<Vec<T>>;

    /* Method to delete a record from the database */
    async fn delete(&self, id: &str) -> Result<bool>;

    /* Method to update a record into the database */
    async fn update_record<T: Serialize + for<'de> Deserialize<'de> + Sync + Send + 'static>(
        &self,
        id: &str,
        tb_name: &str,
        data: T,
    ) -> Result<bool>;

    /* Method to select records from the database */
    async fn select_where<T: DeserializeOwned + Sync>(
        &self,
        tb_name: &str,
        filter: &str,
        columns: &str, // separate columns by ',' in string format
    ) -> Result<Vec<T>>;
}

/* Implementation of the DBInterface trait for DatabaseClient */
#[async_trait]
impl DBInterface for DatabaseClient {
    /* Method to insert a record into the database */
    async fn insert_record<
        T: Serialize + Sync + Send + 'static,
        U: DeserializeOwned + Sync + Clone + 'static,
    >(
        &self,
        tb_name: &str,
        data: T,
    ) -> Result<Option<U>> {
        match self {
            DatabaseClient::Surreal(surrealdb) => surrealdb.insert_record(tb_name, data).await,
            // Add other database client implementations here
        }
    }

    /* Method to select records from the database */
    async fn select<T: DeserializeOwned + Sync>(&self, tb_name: &str) -> Result<Vec<T>> {
        match self {
            DatabaseClient::Surreal(surrealdb) => surrealdb.select(tb_name).await,
            // Add other database client implementations here
        }
    }

    /* Method to delete a record from the database */
    async fn delete(&self, id: &str) -> Result<bool> {
        match self {
            DatabaseClient::Surreal(surrealdb) => surrealdb.delete(id).await,
            // Add other database client implementations here
        }
    }

    async fn update_record<T: Serialize + for<'de> Deserialize<'de> + Sync + Send + 'static>(
        &self,
        id: &str,
        tb_name: &str,
        data: T,
    ) -> Result<bool> {
        match self {
            DatabaseClient::Surreal(surrealdb) => surrealdb.update_record(id, tb_name, data).await,
        }
    }

    async fn select_where<T: DeserializeOwned + Sync>(
        &self,
        tb_name: &str,
        filter: &str,
        columns: &str, // separate columns by ',' in string format
    ) -> Result<Vec<T>> {
        match self {
            DatabaseClient::Surreal(surrealdb) => {
                surrealdb.select_where(tb_name, filter, columns).await
            }
        }
    }
}
