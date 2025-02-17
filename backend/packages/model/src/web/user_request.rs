use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
pub struct User {
    #[validate(length(min = 5))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[validate(custom(function = "validate_role"))]
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
pub struct UserLogin {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

impl From<Json<User>> for User {
    fn from(payload: Json<User>) -> Self {
        Self {
            username: payload.username.clone(),
            password: payload.password.clone(),
            role: payload.role.clone(),
            email: payload.email.clone(),
        }
    }
}

fn validate_role(role: &str) -> Result<(), validator::ValidationError> {
    let valid_roles = ["customer", "seller", "admin"];
    if !valid_roles.contains(&role) {
        return Err(validator::ValidationError::new("invalid_role"));
    }
    Ok(())
}
