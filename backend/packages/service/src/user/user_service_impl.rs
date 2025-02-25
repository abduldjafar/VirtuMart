use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};

use errors::{
    Error::{DataExist, DataNotAvaliable, UserNotVerified},
    Result,
};

use model::{
    domain::user::User as UserData, web::user_request::User,
    web::user_response::User as UserResponse,
};

use repository::user::user_repository::UserRepositoryTrait as _;

use serde_json::Value;

use super::user_service::{UserService, UserServiceTrait};

impl UserService {
    #[tracing::instrument(err, skip_all)]
    fn password_hasher(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(hashed_password)
    }

    #[tracing::instrument(err, skip_all)]
    pub async fn login(&self, email: String) -> Result<UserData> {
        let repo = &self.user_repo;

        repo.is_data_empty_by_email(&email).await?;

        let user_response = repo.get_data_by_email(&email).await?;

        if !user_response.verified {
            return Err(UserNotVerified(format!(
                "user with emai {} not verified",
                email
            )));
        }

        Ok(user_response)
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    #[tracing::instrument(err, skip_all)]
    async fn register_profile(&self, data: User) -> Result<UserResponse> {
        let is_empty_by_username = self
            .user_repo
            .is_data_empty_by_username(&data.username)
            .await?;
        let is_empty_by_email = self.user_repo.is_data_empty_by_email(&data.email).await?;

        if !is_empty_by_username {
            return Err(DataExist(format!("username:{}", data.username)));
        }

        if !is_empty_by_email {
            return Err(DataExist(format!("email:{}", data.email)));
        }

        let hashed_password = Self::password_hasher(&data.password)?;
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let user_id = format!("user_{}", Uuid::new_v4().to_string().replace("-", "_"));

        let db_data = UserData {
            id: user_id,
            username: data.username,
            email: data.email,
            role: data.role,
            password: hashed_password,
            verified: false,
            created_at,
            updated_at,
        };

        self.user_repo.insert_data(db_data.clone()).await?;

        Ok(UserResponse {
            id: db_data.id,
            username: db_data.username,
            email: db_data.email,
            role: db_data.role,
            created_at,
            updated_at,
        })
    }

    #[tracing::instrument(err, skip_all)]
    async fn update_profile(&self, id: &str, data: Value) -> Result<bool> {
        let is_empty_by_user_id = self.user_repo.is_data_empty_by_id(id).await?;

        if is_empty_by_user_id {
            return Err(DataNotAvaliable(format!("id:{}", id)));
        }

        let is_verified = self.user_repo.is_verified(id).await?;

        if !is_verified {
            return Err(UserNotVerified("User not verified".to_string()));
        }

        self.user_repo.update_data(id, data).await
    }
}
