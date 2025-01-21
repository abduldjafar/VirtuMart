use std::sync::Arc;

use super::{
    DBClient, GymSeekerId, GymSeekerModel, RepositoryGymSeekerRequest, RepositoryResult, UserModel,
};
use database::interface::DBInterface as _;

#[derive(Clone, Debug)]
pub struct GymSeekerRepository {
    pub repo: Arc<DBClient>,
}

impl GymSeekerRepository {
    pub async fn is_gym_seeker_data_empty_by_user_id(
        &self,
        user_id: &str,
    ) -> RepositoryResult<(bool, Vec<GymSeekerModel>)> {
        let repo = &self.repo;

        let data_exists = {
            let data: Vec<GymSeekerModel> = repo
                .select_where(
                    "gym_seeker".to_owned(),
                    format!("user_id = {}", user_id),
                    "*".to_string(),
                )
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }

    pub async fn is_gym_seeker_data_empty_by_id(
        &self,
        id: &str,
    ) -> RepositoryResult<(bool, Vec<GymSeekerModel>)> {
        let repo = &self.repo;

        let data_exists = {
            let data: Vec<GymSeekerModel> = repo
                .select_where(
                    "gym_seeker".to_owned(),
                    format!("id = {}", id),
                    "*".to_string(),
                )
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }

    pub async fn is_gym_seeker_user_empty_by_email(
        &self,
        email: &str,
    ) -> RepositoryResult<(bool, Vec<UserModel>)> {
        let repo = self.repo.clone();

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

    pub async fn insert_data(&self, data: GymSeekerModel) -> RepositoryResult<Option<GymSeekerId>> {
        let repo = &self.repo;
        let insert_into_gym_tb: Option<GymSeekerId> =
            repo.insert_record(String::from("gym_seeker"), data).await?;
        Ok(insert_into_gym_tb)
    }

    pub async fn update_data(
        &self,
        gymseeker_id: String,
        data: RepositoryGymSeekerRequest,
    ) -> RepositoryResult<bool> {
        let repo = &self.repo;
        let update_data = repo
            .update_record(gymseeker_id, "gym_seeker".to_string(), data)
            .await?;

        Ok(update_data)
    }
}
