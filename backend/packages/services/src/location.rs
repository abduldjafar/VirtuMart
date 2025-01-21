use chrono::Utc;

use repository::{gym::GymRepository, location::LocationRepository, user::UserRepository};

use database::model::{Id, Location, PayloadLocationRequest};
use errors::Result;

#[derive(Clone)]
pub struct LocationServices {
    pub location_repository: LocationRepository,
    pub user_repository: UserRepository,
    pub gym_repository: GymRepository,
}

impl LocationServices {
    #[tracing::instrument(err, skip_all)]
    #[tracing::instrument(err, skip_all)]
    pub async fn register_location(&self, data: &Location, user_id: &str) -> Result<Option<Id>> {
        let is_gym = self.location_repository.is_user_a_gym(user_id).await?.0;

        if !is_gym {
            return Err(errors::Error::DataNotAvaliable(format!(
                "User id: {} is not a gym",
                user_id
            )));
        }

        let insert_into_location_tb: Option<Id> = self
            .location_repository
            .insert_data(data.clone(), user_id)
            .await?;
        Ok(insert_into_location_tb)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn update_location(
        &self,
        payload: &PayloadLocationRequest,
        gym_id: &str,
    ) -> Result<bool> {
        let is_gym = self.gym_repository.is_gym_data_empty_by_id(gym_id).await?;

        if is_gym.0 {
            return Err(errors::Error::DataNotAvaliable(format!(
                "User id: {} is not a gym",
                gym_id
            )));
        }

        let location_id = is_gym
            .1
            .first()
            .unwrap()
            .location_id
            .clone()
            .unwrap()
            .to_string();

        let (_, existing_data) = self.location_repository.get_location(&location_id).await?;

        let existing_record = existing_data
            .first()
            .ok_or_else(|| errors::Error::DataNotAvaliable(location_id.clone()))?;
        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime::from(Utc::now());

        let data = PayloadLocationRequest {
            address: payload.address.clone(),
            latitude: payload.latitude,
            longitude: payload.longitude,
            created_at: existing_record.created_at.clone(),
            updated_at: Some(time_now),
        };

        let updated_data = self
            .location_repository
            .update_data(location_id, data)
            .await?;
        Ok(updated_data)
    }
}
