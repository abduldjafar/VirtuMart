use super::user_service::{UserService, UserServiceTrait};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use async_trait::async_trait;
use chrono::Utc;
use errors::{
    Error::{DataExist, DataNotAvailable, UserNotVerified},
    Result,
};
use model::{
    domain::user::User as UserData, web::user_request::User,
    web::user_response::User as UserResponse,
};
use repository::user::user_repository::UserRepositoryTrait as _;
use serde_json::Value;
use uuid::Uuid;

impl UserService {
    /// Hashes a password using Argon2.
    #[tracing::instrument(err, skip_all)]
    fn password_hasher(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(hashed_password)
    }

    /// Authenticates a user by email.
    #[tracing::instrument(err, skip_all)]
    pub async fn login(&self, email: String) -> Result<UserData> {
        let repo = &self.user_repo;

        repo.is_data_empty_by_email(&email).await?;
        let user_response = repo.get_data_by_email(&email).await?;

        if !user_response.verified {
            return Err(UserNotVerified(format!(
                "User with email '{}' is not verified",
                email
            )));
        }

        Ok(user_response)
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    /// Registers a new user profile.
    #[tracing::instrument(err, skip_all)]
    async fn register_profile(&self, data: User) -> Result<UserResponse> {
        let is_username_taken = !self
            .user_repo
            .is_data_empty_by_username(&data.username)
            .await?;
        let is_email_taken = !self.user_repo.is_data_empty_by_email(&data.email).await?;

        if is_username_taken {
            return Err(DataExist(format!(
                "Username '{}' already exists",
                data.username
            )));
        }
        if is_email_taken {
            return Err(DataExist(format!("Email '{}' already exists", data.email)));
        }

        let hashed_password = Self::password_hasher(&data.password)?;
        let now = Utc::now();
        let user_id = format!("user_{}", Uuid::new_v4().to_string().replace("-", "_"));

        let db_data = UserData {
            id: user_id.clone(),
            username: data.username,
            email: data.email,
            role: data.role,
            password: hashed_password,
            verified: false,
            created_at: now,
            updated_at: now,
        };

        self.user_repo.insert_data(db_data.clone()).await?;

        Ok(UserResponse {
            id: db_data.id,
            username: db_data.username,
            email: db_data.email,
            role: db_data.role,
            created_at: now,
            updated_at: now,
        })
    }

    /// Updates user profile if the user is verified.
    #[tracing::instrument(err, skip_all)]
    async fn update_profile(&self, id: &str, data: Value) -> Result<bool> {
        if self.user_repo.is_data_empty_by_id(id).await? {
            return Err(DataNotAvailable(format!("User ID '{}' not found", id)));
        }
        if !self.user_repo.is_verified(id).await? {
            return Err(UserNotVerified("User is not verified".to_string()));
        }

        self.user_repo.update_data(id, data).await
    }
}
