use chrono::Utc;
use futures::{stream, StreamExt};

use repository::{
    gym::GymRepository, gymseeker::GymSeekerRepository, location::LocationRepository,
    post::PostRepository, trainer::TrainerRepository, user::UserRepository,
};

use database::model::{
    Gym, GymSeeker, Id, PayloadPostResponseDeserilize, Post, PostGallery,
    PostGalleryForFeedDeserialize, Trainer,
};
use errors::Result;

#[derive(Clone)]
pub struct PostServices {
    pub gym_repository: GymRepository,
    pub user_repository: UserRepository,
    pub gym_seeker_repository: GymSeekerRepository,
    pub trainer_repository: TrainerRepository,
    pub post_repository: PostRepository,
    pub location_repository: LocationRepository,
}

impl PostServices {
    #[tracing::instrument(err, skip_all)]
    pub async fn is_post_data_empty_by_id(&self, id: &str) -> Result<(bool, Vec<Post>)> {
        let data_exists = self.post_repository.is_post_empty_by_id(id).await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn is_post_data_empty_by_user_id(&self, user_id: &str) -> Result<(bool, Vec<Post>)> {
        let data_exists = self
            .post_repository
            .is_post_empty_by_user_id(user_id)
            .await?;
        Ok(data_exists)
    }

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
    pub async fn is_gym_seeker_user_empty(&self, user_id: &str) -> Result<(bool, Vec<GymSeeker>)> {
        let data_exists = self
            .gym_seeker_repository
            .is_gym_seeker_data_empty_by_user_id(user_id)
            .await?;
        Ok(data_exists)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn is_gym_seeker_data_empty_by_id(&self, id: &str) -> Result<(bool, Vec<GymSeeker>)> {
        let data_exists = self
            .gym_seeker_repository
            .is_gym_seeker_data_empty_by_id(id)
            .await?;
        Ok(data_exists)
    }

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
    pub async fn insert_post_gallery(&self, payload: PostGallery) -> Result<Option<Id>> {
        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime::from(Utc::now());

        let mut data = payload;
        data.created_at = Some(time_now.clone());

        let result = self.post_repository.insert_post_gallery(data).await?;
        Ok(result)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn create(&self, id: String, user_type: &str, content: String) -> Result<Option<Id>> {
        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime::from(Utc::now());

        let (_, gym_temp_data) = match id.split(':').next() {
            Some("gym") => self.is_gym_data_empty_by_id(&id).await?,
            _ => self.is_gym_user_empty(&id).await?,
        };

        let (_, gym_seeker_temp_data) = match id.split(':').next() {
            Some("gym_seeker") => self.is_gym_seeker_data_empty_by_id(&id).await?,
            _ => self.is_gym_seeker_user_empty(&id).await?,
        };

        let (_, trainer_temp_data) = match id.split(':').next() {
            Some("trainer") => self.is_trainer_data_empty_by_id(&id).await?,
            _ => self.is_trainer_user_empty(&id).await?,
        };

        let data = match user_type {
            "gym" => {
                let gym_data = gym_temp_data
                    .first()
                    .ok_or_else(|| errors::Error::DataNotAvaliable(id.to_string()))?
                    .clone();
                Post {
                    id: None,
                    content: Some(content),
                    created_at: Some(time_now.clone()),
                    user_id: Some(gym_data.clone().owner_id),
                    gym_id: gym_data.clone().id,
                    gym_seeker_id: None,
                    trainer_id: None,
                    is_gym: Some(true),
                    is_gym_seeker: Some(false),
                    is_trainer: Some(false),
                    updated_at: Some(time_now.clone()),
                }
            }
            "gym_seeker" => {
                let gym_seeker_data = gym_seeker_temp_data
                    .first()
                    .ok_or_else(|| errors::Error::DataNotAvaliable(id.to_string()))?
                    .clone();
                Post {
                    id: None,
                    content: Some(content),
                    created_at: Some(time_now.clone()),
                    user_id: gym_seeker_data.clone().user_id,
                    gym_id: None,
                    gym_seeker_id: gym_seeker_data.clone().id,
                    trainer_id: None,
                    is_gym: Some(false),
                    is_gym_seeker: Some(true),
                    is_trainer: Some(false),
                    updated_at: Some(time_now),
                }
            }
            "trainer" => {
                let trainer_data = trainer_temp_data
                    .first()
                    .ok_or_else(|| errors::Error::DataNotAvaliable(id.to_string()))?
                    .clone();
                Post {
                    id: None,
                    content: Some(content),
                    created_at: Some(time_now.clone()),
                    user_id: Some(trainer_data.clone().user_id),
                    gym_id: None,
                    gym_seeker_id: None,
                    trainer_id: trainer_data.clone().id,
                    is_gym: Some(false),
                    is_gym_seeker: Some(false),
                    is_trainer: Some(true),
                    updated_at: Some(time_now),
                }
            }
            _ => return Err(errors::Error::InvalidUserType((id).to_string())),
        };

        self.post_repository.insert_data(data).await
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn update(&self, user_id: String, id: String, content: String) -> Result<bool> {
        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime::from(Utc::now());

        // Check if post data is empty first
        let (is_empty, post_vecs) = self.is_post_data_empty_by_id(&id).await?;
        if is_empty {
            return Err(errors::Error::DataNotAvaliable(id.clone()));
        }

        // Safely get the post data, assuming post_vecs is non-empty after the previous check
        let post_data = post_vecs
            .first()
            .ok_or_else(|| errors::Error::DataNotAvaliable(id.clone()))?;

        // Check if the user_id matches
        if post_data
            .user_id
            .as_ref()
            .map_or(false, |uid| uid.to_string() != user_id)
        {
            return Err(errors::Error::UserNotVerified(format!(
                "user {} not authorized to update",
                id.clone()
            )));
        }

        // Update post content and timestamp
        let mut updated_post_data = post_data.clone();
        updated_post_data.content = Some(content);
        updated_post_data.updated_at = Some(time_now);

        // Call the repository's update function
        self.post_repository
            .update_data(id, updated_post_data)
            .await
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn delete(&self, user_id: String, id: String) -> Result<bool> {
        // Check if post data is empty first
        let (is_empty, post_vecs) = self.is_post_data_empty_by_id(&id).await?;
        if is_empty {
            return Err(errors::Error::DataNotAvaliable(id.clone()));
        }

        // Safely get the post data, assuming post_vecs is non-empty after the previous check
        let post_data = post_vecs
            .first()
            .ok_or_else(|| errors::Error::DataNotAvaliable(id.clone()))?;

        // Check if the user_id matches
        if post_data
            .user_id
            .as_ref()
            .map_or(false, |uid| uid.to_string() != user_id)
        {
            return Err(errors::Error::UserNotVerified(format!(
                "user {} not authorized to update",
                id.clone()
            )));
        }

        self.post_repository.delete_data(id).await
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn get_list(
        &self,
        user_id: String,
        page: i64,
    ) -> Result<Vec<PayloadPostResponseDeserilize>> {
        let (is_empty, post_vecs) = self.post_repository.get_list(page, user_id).await?;
        if is_empty {
            return Ok(Vec::new());
        }

        let data = stream::iter(post_vecs)
            .map(|payload| {
                let post_gallery_deserialized: Vec<PostGalleryForFeedDeserialize> = payload
                    .clone()
                    .post_gallery
                    .unwrap()
                    .iter()
                    .map(|gallery| PostGalleryForFeedDeserialize {
                        id: gallery.clone().id.unwrap().to_string(),
                        link: gallery.clone().link,
                    })
                    .collect();

                PayloadPostResponseDeserilize {
                    id: payload.id.as_ref().map(|id| id.to_string()),
                    content: payload.content.clone(),
                    created_at: payload.created_at.clone(),
                    user_id: payload.user_id.as_ref().map(|user_id| user_id.to_string()),
                    gym_id: payload.gym_id.as_ref().map(|gym_id| gym_id.to_string()),
                    gym_seeker_id: payload
                        .gym_seeker_id
                        .as_ref()
                        .map(|gym_seeker_id| gym_seeker_id.to_string()),
                    trainer_id: payload
                        .trainer_id
                        .as_ref()
                        .map(|trainer_id| trainer_id.to_string()),
                    is_gym: payload.is_gym,
                    is_gym_seeker: payload.is_gym_seeker,
                    is_trainer: payload.is_trainer,
                    post_gallery: Some(post_gallery_deserialized),
                    updated_at: payload.updated_at.clone(),
                }
            })
            .collect::<Vec<_>>()
            .await;
        Ok(data)
    }
}
