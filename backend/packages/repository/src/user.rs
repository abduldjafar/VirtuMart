use std::sync::Arc;

use super::{DBClient, RepositoryResult, UserModel};
use database::interface::DBInterface as _;

#[derive(Clone, Debug)]
pub struct UserRepository {
    pub repo: Arc<DBClient>,
}

impl UserRepository {
    pub async fn insert_data(&self, data: UserModel) -> RepositoryResult<Option<String>> {
        let repo = &self.repo;
        let insert_into_user_tb: Option<String> =
            repo.insert_record(String::from("user"), data).await?;
        Ok(insert_into_user_tb)
    }
}