use std::sync::Arc;

use super::{
    DBClient, RepositoryResult, RepositoryTrainerRequests, TrainerId, TrainerModel, UserModel,
};
use database::interface::DBInterface as _;

#[derive(Clone)]
pub struct TrainerRepository {
    pub repo: Arc<DBClient>,
}

impl TrainerRepository {
    pub async fn is_trainer_data_empty_by_id(
        &self,
        id: &str,
    ) -> RepositoryResult<(bool, Vec<TrainerModel>)> {
        let repo = &self.repo;

        let data_exists = {
            let data: Vec<TrainerModel> = repo
                .select_where(
                    "trainer".to_owned(),
                    format!("id = {}", id),
                    "*".to_string(),
                )
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }

    pub async fn is_trainer_data_empty(
        &self,
        user_id: &str,
    ) -> RepositoryResult<(bool, Vec<TrainerModel>)> {
        let repo = &self.repo;

        let data_exists = {
            let data: Vec<TrainerModel> = repo
                .select_where(
                    "trainer".to_owned(),
                    format!("user_id =  {}", user_id),
                    "*".to_string(),
                )
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }

    pub async fn is_trainer_data_empty_by_email(
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

    pub async fn insert_data(&self, data: TrainerModel) -> RepositoryResult<Option<TrainerId>> {
        let repo = &self.repo;
        let insert_into_gym_tb: Option<TrainerId> =
            repo.insert_record(String::from("trainer"), data).await?;
        Ok(insert_into_gym_tb)
    }

    pub async fn get_details(&self, id: &str) -> RepositoryResult<Vec<TrainerModel>> {
        let repo = &self.repo;

        let data: Vec<TrainerModel> = repo
            .select_where(
                "trainer".to_owned(),
                format!("id  =  '{}'", id),
                "*".to_string(),
            )
            .await?;

        Ok(data)
    }

    pub async fn update_data(
        &self,
        gym_id: String,
        data: RepositoryTrainerRequests,
    ) -> RepositoryResult<bool> {
        let repo = &self.repo;

        let update_data = repo
            .update_record(gym_id, "trainer".to_string(), data)
            .await?;

        Ok(update_data)
    }
}
