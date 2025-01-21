use std::sync::Arc;

use super::{
    DBClient, LocationId, LocationModel, RepositoryLocationRequest, RepositoryResult, UserModel,
};
use database::interface::DBInterface as _;

#[derive(Clone, Debug)]
pub struct LocationRepository {
    pub repo: Arc<DBClient>,
}

impl LocationRepository {
    pub async fn insert_data(
        &self,
        data: LocationModel,
        user_id: &str,
    ) -> RepositoryResult<Option<LocationId>> {
        if self.is_user_a_gym(user_id).await?.0 {
            let location_id: Option<LocationId> = self
                .repo
                .insert_record("location".to_string(), data)
                .await?;

            return Ok(location_id);
        }
        Ok(None)
    }

    pub async fn update_data(
        &self,
        location_id: String,
        data: RepositoryLocationRequest,
    ) -> RepositoryResult<bool> {
        let updated = self
            .repo
            .update_record(location_id, "location".to_string(), data)
            .await?;

        Ok(updated)
    }

    pub async fn is_user_a_gym(&self, user_id: &str) -> RepositoryResult<(bool, Vec<UserModel>)> {
        let users: Vec<UserModel> = self
            .repo
            .select_where(
                "user".to_string(),
                format!("id = {} and user_type = 'gym'", user_id),
                "*".to_string(),
            )
            .await?;

        let is_gym = !users.is_empty(); // Check if the result set is not empty
        Ok((is_gym, users))
    }

    pub async fn get_location(
        &self,
        location_id: &str,
    ) -> RepositoryResult<(bool, Vec<LocationModel>)> {
        let location: Vec<LocationModel> = self
            .repo
            .select_where(
                "location".to_string(),
                format!("id = {}", location_id),
                "*".to_string(),
            )
            .await?;

        let is_location = !location.is_empty(); // Check if the result set is not empty
        Ok((is_location, location))
    }
}
