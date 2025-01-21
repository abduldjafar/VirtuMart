use chrono::prelude::*;
use database::model::{Gym, Id, Location, PayloadGymRequest, PayloadGymResponses, User};
use errors::Result;
use repository::{gym::GymRepository, location::LocationRepository, user::UserRepository};

#[derive(Clone)]
pub struct GymServices {
    pub gym_repository: GymRepository,
    pub user_repository: UserRepository,
    pub location_repository: LocationRepository,
}

impl GymServices {
    #[tracing::instrument(err, skip_all)]
    pub async fn is_gym_data_empty_by_id(&self, id: &str) -> Result<(bool, Vec<Gym>)> {
        let data_exists = self.gym_repository.is_gym_data_empty_by_id(id).await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn is_gym_user_empty(&self, user_id: &str) -> Result<(bool, Vec<Gym>)> {
        let data_exists = self.gym_repository.is_gym_data_empty(user_id).await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    async fn is_user_empty(&self, data: &User) -> Result<(bool, Vec<User>)> {
        let data_exists = self
            .gym_repository
            .is_gym_data_empty_by_email(&data.email)
            .await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    async fn is_username_empty(&self, data: &User) -> Result<(bool, Vec<User>)> {
        let data_exists = self.user_repository.is_data_empty_by_username(data).await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn register_profile(&self, mut data: User) -> Result<Option<Id>> {
        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime::from(Utc::now());

        let (is_user_empty, _) = self.is_user_empty(&data).await?;
        if !is_user_empty {
            return Err(errors::Error::DataExist(format!("email:{}", data.email)));
        }

        let (is_username_empty, _) = self.is_username_empty(&data).await?;
        if !is_username_empty {
            return Err(errors::Error::DataExist(format!(
                "username:{}",
                data.username
            )));
        }

        data.created_at = Some(time_now.clone());
        data.updated_at = Some(time_now);

        let insert_into_user_tb: Option<Id> =
            self.user_repository.insert_data(data.clone()).await?;
        let user_id = insert_into_user_tb.unwrap();

        let (not_exists, _) = self
            .is_gym_user_empty(user_id.id.to_string().as_ref())
            .await?;
        if !not_exists {
            return Err(errors::Error::DataExist(format!("email:{}", data.email)));
        }

        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime::from(Utc::now());

        let location = Location {
            id: None,
            address: "".to_string(),
            created_at: None,
            updated_at: None,
            latitude: 0.0,
            longitude: 0.0,
        };

        let insert_into_location_tb: Option<Id> = self
            .location_repository
            .insert_data(location, user_id.id.to_string().as_str())
            .await?;

        let gym_data = Gym {
            id: None,
            name: String::from(""),
            description: String::from(""),
            profile_picture: String::from(""),
            location_id: Some(insert_into_location_tb.unwrap().id),
            owner_id: user_id.id,
            created_at: Some(time_now.clone()),
            updated_at: Some(time_now.clone()),
        };

        let insert_into_gym_tb: Option<Id> = self.gym_repository.insert_data(gym_data).await?;
        Ok(insert_into_gym_tb)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn profile_details(&self, id: String) -> Result<PayloadGymResponses> {
        let (is_empty, temp_data) = match id.split(':').next() {
            Some("gym") => self.is_gym_data_empty_by_id(&id).await?,
            _ => self.is_gym_user_empty(&id).await?,
        };

        if is_empty {
            return Err(errors::Error::DataNotAvaliable((id).to_string()));
        }

        let data_array: Vec<PayloadGymResponses> = temp_data
            .into_iter()
            .map(|gym| PayloadGymResponses {
                id: gym.id.unwrap().to_string(),
                name: gym.name,
                description: gym.description,
                profile_picture: gym.profile_picture,
                location_id: gym.location_id,
                created_at: gym.created_at,
                updated_at: gym.updated_at,
                owner_id: Some(gym.owner_id),
            })
            .collect();

        let data = data_array
            .first()
            .ok_or_else(|| errors::Error::DataNotAvaliable(id.to_string()))?
            .clone();

        Ok(data)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn update_profile(&self, payload: &PayloadGymRequest, id: &str) -> Result<()> {
        let (not_exists, existing_data) = self.gym_repository.is_gym_data_empty_by_id(id).await?;
        if not_exists {
            return Err(errors::Error::DataNotAvaliable(id.to_string()));
        }

        let existing_record = existing_data
            .first()
            .ok_or_else(|| errors::Error::DataNotAvaliable(id.to_string()))?;
        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime::from(Utc::now());

        let data = PayloadGymRequest {
            name: payload
                .name
                .clone()
                .or_else(|| Some(existing_record.name.clone())),
            description: payload
                .description
                .clone()
                .or_else(|| Some(existing_record.description.clone())),
            profile_picture: payload
                .profile_picture
                .clone()
                .or_else(|| Some(existing_record.profile_picture.clone())),
            location_id: existing_record.location_id.clone(),
            created_at: existing_record.created_at.clone(),
            owner_id: Some(existing_record.owner_id.clone()),
            updated_at: Some(time_now),
        };

        let gym_id = existing_record.clone().id.unwrap().to_string();
        let update_data = self.gym_repository.update_data(gym_id, data).await?;
        if !update_data {
            return Err(errors::Error::DatabaseError(id.to_string()));
        }

        Ok(())
    }
}
