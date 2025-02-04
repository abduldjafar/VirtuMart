use super::interface;
use crate::database::SurrealDb;
use async_trait::async_trait;
use errors::Result;
use interface::DBInterface;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/* Implementation of DBInterface for SurrealDb */
#[async_trait]
impl DBInterface for SurrealDb {
    
    /// Method to insert a record into the database
    ///
    /// Using `ok_or_else` instead of `ok_or` is generally preferred for better performance and idiomatic Rust code.
    /// The `ok_or_else` method allows you to lazily evaluate the error value only if the `Option` is `None`,
    /// whereas `ok_or` always evaluates the error value, even if it is not needed.
    
    async fn insert_record<T, U>(&self, tb_name: &str, data: T) -> Result<Option<U>>
    where
        T: Serialize + Sync + Send + 'static,
        U: DeserializeOwned + Sync + Clone + 'static,
    {
        let client = self.client.clone().ok_or_else(|| {
            errors::Error::DatabaseErrorExecution("surrealdb: Client connection error".to_string())
        })?;
        let created: Vec<U> = client.insert(tb_name).content(data).await?;
        let record = created.first().cloned();
        Ok(record)
    }

    /* Method to select records from the database */
    async fn select<T: DeserializeOwned + Sync>(&self, tb_name: &str) -> Result<Vec<T>> {
        let client = self.client.clone().ok_or_else(|| {
            errors::Error::DatabaseErrorExecution("surrealdb: Client connection error".to_string())
        })?;
        let data: Vec<T> = client.select(tb_name).await?;
        Ok(data)
    }

    /* Method to delete a record from the database */
    async fn delete(&self, id: &str) -> Result<bool> {
        let client = self.client.clone().ok_or_else(|| {
            errors::Error::DatabaseErrorExecution("surrealdb: Client connection error".to_string())
        })?;
        let result = client.query(format!("DELETE {}", id)).await?.check();
        Ok(result.is_ok())
    }

    /* Method to update a record in the database */
    async fn update_record<T>(&self, id: &str, tb_name: &str, data: T) -> Result<bool>
    where
        T: Serialize + for<'de> Deserialize<'de> + Sync + Send + 'static,
    {
        let data_id: Vec<&str> = id.split(':').collect();
        let client = self.client.clone().ok_or_else(|| {
            errors::Error::DatabaseErrorExecution("surrealdb: Client connection error".to_string())
        })?;
        let updated_result: Option<model::surreal_db::user::ReturnedUser> =
            client.update((tb_name, data_id[1])).merge(data).await?;
        Ok(updated_result.is_some())
    }

    /* Method to select records with parameters from the database */
    async fn select_where<T: DeserializeOwned + Sync>(
        &self,
        tb_name: &str,
        filter: &str,
        columns: &str,
    ) -> Result<Vec<T>> {
        let client = self
            .client
            .clone()
            .ok_or(errors::Error::DatabaseErrorExecution(
                "surrealdb: Client connection error".to_string(),
            ))?;

        let filtered_query = if filter.is_empty() {
            String::new()
        } else {
            format!("where {}", filter)
        };

        let tb_columns = if columns.is_empty() {
            String::from(" * ")
        } else {
            format!(" {} ", columns)
        };

        let sql = format!("SELECT {} FROM {} {}", tb_columns, tb_name, filtered_query);
        println!("SQL: {}", sql);

        let mut results = client.query(&sql).await?;
        let data: Vec<T> = results.take(0)?;
        Ok(data)
    }
}
