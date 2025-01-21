use chrono::Utc;

use repository::{gym::GymRepository, trainer::TrainerRepository, user::UserRepository};

use database::model::{Gym, Id, PayloadTrainerRequest, PayloadTrainerResponse, Trainer, User};
use errors::Result;

#[derive(Clone)]
pub struct TrainerServices {
    pub trainer_repository: TrainerRepository,
    pub user_repository: UserRepository,
    pub gym_repository: GymRepository,
}

impl TrainerServices {
    #[tracing::instrument(err, skip_all)]
    pub async fn is_trainer_data_empty_by_id(&self, id: &str) -> Result<(bool, Vec<Trainer>)> {
        let data_exists = self
            .trainer_repository
            .is_trainer_data_empty_by_id(id)
            .await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn is_trainer_user_empty(&self, user_id: &str) -> Result<(bool, Vec<Trainer>)> {
        let data_exists = self
            .trainer_repository
            .is_trainer_data_empty(user_id)
            .await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    async fn is_user_empty(&self, data: &User) -> Result<(bool, Vec<User>)> {
        let data_exists = self
            .trainer_repository
            .is_trainer_data_empty_by_email(&data.email)
            .await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    async fn is_username_empty(&self, data: &User) -> Result<(bool, Vec<User>)> {
        let data_exists = self.user_repository.is_data_empty_by_username(data).await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    async fn is_gym_id_empty(&self, gym_id: &str) -> Result<(bool, Vec<Gym>)> {
        let data_exists = self.gym_repository.is_gym_data_empty_by_id(gym_id).await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn register_profile(&self, mut data: User, gym_id: String) -> Result<Option<Id>> {
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

        let is_gym_empty = self.is_gym_id_empty(&gym_id).await?;
        if is_gym_empty.0 {
            return Err(errors::Error::DataNotAvaliable(gym_id));
        }

        let gym_id_in_db = is_gym_empty.1.first().unwrap();

        data.created_at = Some(time_now.clone());
        data.updated_at = Some(time_now);

        let insert_into_user_tb: Option<Id> =
            self.user_repository.insert_data(data.clone()).await?;
        let user_id = insert_into_user_tb.unwrap();

        let (not_exists, _) = self
            .is_trainer_user_empty(user_id.id.to_string().as_ref())
            .await?;
        if !not_exists {
            return Err(errors::Error::DataExist(format!("email:{}", data.email)));
        }

        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime::from(Utc::now());

        let gym_data = Trainer {
            id: None,
            created_at: Some(time_now.clone()),
            updated_at: Some(time_now.clone()),
            name: Some(String::from("")),
            sex: Some(String::from("")),
            experience: Some(0),
            expertise: Some(String::from("")),
            gym_id: gym_id_in_db.clone().id.unwrap(),
            user_id: user_id.id,
        };

        let insert_into_gym_tb: Option<Id> = self.trainer_repository.insert_data(gym_data).await?;
        Ok(insert_into_gym_tb)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn profile_details(&self, id: String) -> Result<PayloadTrainerResponse> {
        let (is_empty, temp_data) = match id.split(':').next() {
            Some("trainer") => self.is_trainer_data_empty_by_id(&id).await?,
            _ => self.is_trainer_user_empty(&id).await?,
        };

        if is_empty {
            return Err(errors::Error::DataNotAvaliable((id).to_string()));
        }

        let data_array: Vec<PayloadTrainerResponse> = temp_data
            .into_iter()
            .map(|trainer| PayloadTrainerResponse {
                id: trainer.id.clone().unwrap().to_string(),
                name: trainer.name,
                sex: trainer.sex,
                experience: trainer.experience,
                expertise: trainer.expertise,
                created_at: trainer.created_at,
                updated_at: trainer.updated_at,
            })
            .collect();

        let data = data_array
            .first()
            .ok_or_else(|| errors::Error::DataNotAvaliable(id.clone()))?
            .clone();
        Ok(data)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn update_profile(&self, payload: &PayloadTrainerRequest, id: &str) -> Result<()> {
        let (not_exists, existing_data) = self
            .trainer_repository
            .is_trainer_data_empty_by_id(id)
            .await?;
        if not_exists {
            return Err(errors::Error::DataNotAvaliable(id.to_string()));
        }

        let existing_record = existing_data
            .first()
            .ok_or_else(|| errors::Error::DataNotAvaliable(id.to_string()))?
            .clone();
        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime::from(Utc::now());

        let data = PayloadTrainerRequest {
            name: payload
                .name
                .clone()
                .or_else(|| existing_record.name.clone()),
            sex: payload.sex.clone().or_else(|| existing_record.sex.clone()),
            experience: payload.experience.or(existing_record.experience),
            expertise: payload
                .expertise
                .clone()
                .or_else(|| existing_record.expertise.clone()),
            gym_id: Some(existing_record.gym_id.clone()),
            user_id: Some(existing_record.user_id.clone()),
            created_at: existing_record.created_at.clone(),
            updated_at: Some(time_now),
        };

        let trainer_id = existing_record.clone().id.unwrap().to_string();
        let update_data = self
            .trainer_repository
            .update_data(trainer_id, data)
            .await?;
        if !update_data {
            return Err(errors::Error::DatabaseError(id.to_string()));
        }

        Ok(())
    }
}
