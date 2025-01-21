use chrono::Utc;

use database::model::{GymSeeker, Id, PayloadGymSeekerRequest, PayloadGymSeekerResponse, User};
use errors::Result;
use repository::{gymseeker::GymSeekerRepository, user::UserRepository};

#[derive(Clone, Debug)]
pub struct GymSeekerServices {
    pub repository: GymSeekerRepository,
    pub user_repository: UserRepository,
}

impl GymSeekerServices {
    #[tracing::instrument(err, skip_all)]
    pub async fn is_gym_seeker_user_empty(&self, user_id: &str) -> Result<(bool, Vec<GymSeeker>)> {
        let data_exists = self
            .repository
            .is_gym_seeker_data_empty_by_user_id(user_id)
            .await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn is_gym_seeker_data_empty_by_id(&self, id: &str) -> Result<(bool, Vec<GymSeeker>)> {
        let data_exists = self.repository.is_gym_seeker_data_empty_by_id(id).await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    async fn is_user_empty(&self, data: &User) -> Result<(bool, Vec<User>)> {
        let data_exists = self
            .repository
            .is_gym_seeker_user_empty_by_email(&data.email)
            .await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    async fn is_username_empty(&self, data: &User) -> Result<(bool, Vec<User>)> {
        let data_exists = self.user_repository.is_data_empty_by_username(data).await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn register_profile(&self, data: &User) -> Result<Option<Id>> {
        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime::from(Utc::now());

        let (is_user_empty, _) = self.is_user_empty(data).await?;
        if !is_user_empty {
            return Err(errors::Error::DataExist(format!("email:{}", data.email)));
        }

        let (is_username_empty, _) = self.is_username_empty(data).await?;
        if !is_username_empty {
            return Err(errors::Error::DataExist(format!(
                "username:{}",
                data.username
            )));
        }

        let insert_into_user_tb: Option<Id> =
            self.user_repository.insert_data(data.clone()).await?;

        let user_id = insert_into_user_tb.unwrap();

        let (not_exists, _) = self
            .is_gym_seeker_user_empty(user_id.id.to_string().as_str())
            .await?;

        if !not_exists {
            return Err(errors::Error::DataExist(format!("email:{}", data.email)));
        }

        let gym_seeker_data = GymSeeker {
            id: None,
            name: "".to_string(),
            created_at: Some(time_now.clone()),
            updated_at: Some(time_now.clone()),
            birth_date: Some(time_now.clone()),
            sex: "".to_string(),
            user_id: Some(user_id.id),
            profile_picture: "".to_string(),
            fitness_goals: "".to_string(),
            preferred_workout_time: "".to_string(),
            gym_preferences: "".to_string(),
            membership_status: "".to_string(),
            bio: "".to_string(),
        };

        let insert_into_gym_tb: Option<Id> = self.repository.insert_data(gym_seeker_data).await?;

        Ok(insert_into_gym_tb)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn profile_details(&self, id: String) -> Result<PayloadGymSeekerResponse> {
        let (is_empty, temp_gym_seeker_user) = match id.split(':').next() {
            Some("gym_seeker") => self.is_gym_seeker_data_empty_by_id(&id).await?,
            _ => self.is_gym_seeker_user_empty(&id).await?,
        };

        if is_empty {
            return Err(errors::Error::DataNotAvaliable(id.to_string()));
        }

        let data_array = temp_gym_seeker_user
            .first()
            .map(|gym_seeker| PayloadGymSeekerResponse {
                id: gym_seeker.clone().id.unwrap().to_string(),
                name: gym_seeker.clone().name,
                user_id: gym_seeker.clone().user_id,
                birth_date: gym_seeker.clone().birth_date,
                sex: gym_seeker.clone().sex,
                profile_picture: gym_seeker.clone().profile_picture,
                fitness_goals: gym_seeker.clone().fitness_goals,
                preferred_workout_time: gym_seeker.clone().preferred_workout_time,
                gym_preferences: gym_seeker.clone().gym_preferences,
                membership_status: gym_seeker.clone().membership_status,
                bio: gym_seeker.clone().bio,
                created_at: gym_seeker.clone().created_at,
                updated_at: gym_seeker.clone().updated_at,
            });

        let data = match data_array {
            Some(data) => data.to_owned(),
            None => {
                return Err(errors::Error::DataNotAvaliable(id.to_string()));
            }
        };

        Ok(data)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn update_profile(&self, payload: &PayloadGymSeekerRequest, id: &str) -> Result<()> {
        let (not_exists, existing_data) = self.is_gym_seeker_data_empty_by_id(id).await?;

        if not_exists {
            return Err(errors::Error::DataNotAvaliable(id.to_string()));
        }

        let existing_record = existing_data
            .first()
            .ok_or_else(|| errors::Error::DataNotAvaliable(id.to_string()))?
            .clone();
        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime::from(Utc::now());

        let data = PayloadGymSeekerRequest {
            created_at: existing_record.created_at.clone(),
            updated_at: Some(time_now),
            birth_date: payload
                .birth_date
                .clone()
                .or_else(|| existing_record.birth_date.clone()),
            sex: payload
                .sex
                .clone()
                .or_else(|| Some(existing_record.sex.clone())),
            user_id: existing_record.user_id.clone(),
            profile_picture: payload
                .profile_picture
                .clone()
                .or_else(|| Some(existing_record.profile_picture.clone())),
            fitness_goals: payload
                .fitness_goals
                .clone()
                .or_else(|| Some(existing_record.fitness_goals.clone())),
            preferred_workout_time: payload
                .preferred_workout_time
                .clone()
                .or_else(|| Some(existing_record.preferred_workout_time.clone())),
            gym_preferences: payload
                .gym_preferences
                .clone()
                .or_else(|| Some(existing_record.gym_preferences.clone())),
            membership_status: payload
                .membership_status
                .clone()
                .or_else(|| Some(existing_record.membership_status.clone())),
            bio: payload
                .bio
                .clone()
                .or_else(|| Some(existing_record.sex.clone())),
            name: payload
                .name
                .clone()
                .or_else(|| Some(existing_record.name.clone())),
        };

        let gym_seeker_id = existing_record.clone().id.unwrap().to_string();

        let update_data = self.repository.update_data(gym_seeker_id, data).await?;

        if !update_data {
            return Err(errors::Error::DatabaseError(id.to_string()));
        }

        Ok(())
    }
}
