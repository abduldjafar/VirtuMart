use super::user_service::{UserService, UserServiceTrait};
use argon2::password_hash::rand_core::OsRng;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use async_trait::async_trait;
use chrono::Utc;
use errors::Error::{DataExist, DataNotAvaliable};
use errors::Result;
use model::domain::user::User as UserData;
use model::web::user_request::User;
use model::web::user_response::User as UserResponse;
use repository::user::user_repository::UserRepositoryTrait as _;
use serde_json::Value;

impl UserService {
    fn password_hasher(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(hashed_password)
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
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

        let db_data = UserData {
            id: data.id,
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

    async fn update_profile(&self, id: &str, data: Value) -> Result<bool> {
        let is_empty_by_user_id = self.user_repo.is_data_empty_by_username(id).await?;

        if is_empty_by_user_id {
            return Err(DataNotAvaliable(format!("id:{}", id)));
        }

        self.user_repo.update_data(id, data).await
    }
}
