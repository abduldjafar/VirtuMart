use std::sync::Arc;

use super::{DBClient, RepositoryResult, UserId, UserModel};
use database::interface::DBInterface as _;

#[derive(Clone, Debug)]
pub struct UserRepository {
    pub repo: Arc<DBClient>,
}

impl UserRepository {
    pub async fn insert_data(&self, data: UserModel) -> RepositoryResult<Option<UserId>> {
        let repo = &self.repo;
        let insert_into_user_tb: Option<UserId> =
            repo.insert_record(String::from("user"), data).await?;
        Ok(insert_into_user_tb)
    }

    pub async fn is_data_empty_by_username(
        &self,
        data: &UserModel,
    ) -> RepositoryResult<(bool, Vec<UserModel>)> {
        let repo = &self.repo;

        let data_exists = {
            let data: Vec<UserModel> = repo
                .select_where(
                    "user".to_owned(),
                    format!("username = '{}'", data.username),
                    "*".to_string(),
                )
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }
}
