use std::sync::Arc;

use super::{DBClient, GymId, GymModel, RepositoryGymRequest, RepositoryResult, UserModel};
use database::interface::DBInterface as _;

#[derive(Clone)]
pub struct GymRepository {
    pub repo: Arc<DBClient>,
}

impl GymRepository {
    pub async fn is_gym_data_empty_by_id(
        &self,
        id: &str,
    ) -> RepositoryResult<(bool, Vec<GymModel>)> {
        let repo = &self.repo;

        let data_exists = {
            let data: Vec<GymModel> = repo
                .select_where("gym".to_owned(), format!("id = {}", id), "*".to_string())
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }

    pub async fn is_gym_data_empty(
        &self,
        user_id: &str,
    ) -> RepositoryResult<(bool, Vec<GymModel>)> {
        let repo = &self.repo;

        let data_exists = {
            let data: Vec<GymModel> = repo
                .select_where(
                    "gym".to_owned(),
                    format!("owner_id =  {}", user_id),
                    "*".to_string(),
                )
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }

    pub async fn is_gym_data_empty_by_email(
        &self,
        email: &str,
    ) -> RepositoryResult<(bool, Vec<UserModel>)> {
        let repo = &self.repo;

        let data_exists = {
            let data: Vec<UserModel> = repo
                .select_where(
                    "user".to_owned(),
                    format!("email = '{}'", email),
                    "*".to_string(),
                )
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }

    pub async fn insert_data(&self, data: GymModel) -> RepositoryResult<Option<GymId>> {
        let repo = &self.repo;
        let insert_into_gym_tb: Option<GymId> =
            repo.insert_record(String::from("gym"), data).await?;
        Ok(insert_into_gym_tb)
    }

    pub async fn get_details(&self, user_id: &str) -> RepositoryResult<Vec<GymModel>> {
        let repo = &self.repo;

        let data: Vec<GymModel> = repo
            .select_where(
                "gym".to_owned(),
                format!("owner_id =  '{}'", user_id),
                "*".to_string(),
            )
            .await?;

        Ok(data)
    }

    pub async fn update_data(
        &self,
        gym_id: String,
        data: RepositoryGymRequest,
    ) -> RepositoryResult<bool> {
        let repo = &self.repo;

        let update_data = repo.update_record(gym_id, "gym".to_string(), data).await?;

        Ok(update_data)
    }
}
