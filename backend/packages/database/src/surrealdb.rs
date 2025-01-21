use super::interface;
use crate::db::SurrealDb;
use axum::async_trait;
use errors::Result;
use interface::DBInterface;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/* Implementation of DBInterface for SurrealDb */
#[async_trait]
impl DBInterface for SurrealDb {
    /* Method to insert a record into the database */
    #[tracing::instrument(err, skip_all)]
    async fn insert_record<T, U>(&self, tb_name: String, data: T) -> Result<Option<U>>
    where
        T: Serialize + Sync + Send + 'static,
        U: DeserializeOwned + Sync + Clone + 'static,
    {
        let client = self.client.clone().unwrap();
        let created: Vec<U> = client.insert(tb_name).content(data).await?;
        let record = created.first().cloned();
        Ok(record)
    }

    /* Method to select records from the database */

    #[tracing::instrument(err, skip_all)]
    async fn select<T: DeserializeOwned + Sync>(&self, tb_name: String) -> Result<Vec<T>> {
        let client = self.client.clone().unwrap();
        let data: Vec<T> = client.select(tb_name).await?;
        Ok(data)
    }

    /* Method to delete a record from the database */

    #[tracing::instrument(err, skip_all)]
    async fn delete(&self, id: String) -> Result<bool> {
        let client = self.client.clone().unwrap();
        let result = client.query(format!("DELETE {}", id)).await?.check();

        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /* Method to update a record in the database */
    #[tracing::instrument(err, skip_all)]
    async fn update_record<T>(&self, id: String, tb_name: String, data: T) -> Result<bool>
    where
        T: Serialize + for<'de> Deserialize<'de> + Sync + Send + 'static,
    {
        let data_id: Vec<&str> = id.split(':').collect();
        let client = self.client.clone().unwrap();
        let updated_result: Option<T> = client.update((tb_name, data_id[1])).content(data).await?;
        Ok(updated_result.is_some())
    }

    /* Method to select records with parameters from the database */
    #[tracing::instrument(err, skip_all)]
    async fn select_where<T: DeserializeOwned + Sync>(
        &self,
        tb_name: String,
        filter: String,
        columns: String,
    ) -> Result<Vec<T>> {
        let client = self.client.clone().unwrap();

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

        let mut results = client.query(&sql).await?;
        let data: Vec<T> = results.take(0)?;
        Ok(data)
    }
}
