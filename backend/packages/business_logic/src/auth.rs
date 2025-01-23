use chrono::Utc;
use database::{
    db::DatabaseClient,
    interface::DBInterface,
    model::{PayloadUserResponse, User},
};

use errors::Result;

use repository::user::UserRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthServices {
    pub repo: Arc<DatabaseClient>,
    pub user_repository: UserRepository,
}

impl AuthServices {
    #[tracing::instrument(err, skip_all)]
    pub async fn user_verification(&self, token: String) -> Result<bool> {
        let repo = &self.repo;
        let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime::from(Utc::now());

        let vect_data: Vec<PayloadUserResponse> = repo
            .select_where(
                "user".to_owned(),
                format!("verified_token = '{}'", token),
                "*".to_string(),
            )
            .await?;

        if vect_data.is_empty() {
            return Err(errors::Error::UserUnauthorized(String::from(
                "Invalid verification token",
            )));
        }

        let mut user = vect_data.first().unwrap().clone();
        let cloned_user = user.clone();

        if user.verified_token.unwrap() != token {
            return Err(errors::Error::UserUnauthorized(String::from(
                "Invalid verification token",
            )));
        }

        user.verified = true;

        let updated_data = User {
            id: Some(cloned_user.id),
            username: cloned_user.username,
            user_type: cloned_user.user_type,
            email: cloned_user.email,
            password: cloned_user.password,
            verified: user.verified,
            verified_token: Some(cloned_user.verified_token.unwrap()),
            created_at: cloned_user.created_at,
            updated_at: Some(time_now),
        };

        let updated_user = self
            .repo
            .update_record(user.id.to_string(), "user".to_string(), updated_data)
            .await?;

        Ok(updated_user)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn login(&self, email: String) -> Result<PayloadUserResponse> {
        let repo = &self.repo;

        let vect_data: Vec<PayloadUserResponse> = repo
            .select_where(
                "user".to_owned(),
                format!("email = '{}'", email),
                "*".to_string(),
            )
            .await?;

        if vect_data.is_empty() {
            return Err(errors::Error::DataNotAvaliable(String::from(
                "User not found",
            )));
        }

        let data = vect_data.first().unwrap().clone();

        if !data.verified {
            return Err(errors::Error::UserNotVerified(String::from(
                "User not Verified Yet",
            )));
        }

        Ok(data)
    }
}
