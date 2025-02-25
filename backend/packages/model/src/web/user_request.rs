use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

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
            email: payload.email.clone(),
            password: payload.password.clone(),
            role: payload.role.clone(),
        }
    }
}

/// Validates that the given role is one of the predefined valid roles.
fn validate_role(role: &str) -> Result<(), ValidationError> {
    const VALID_ROLES: [&str; 3] = ["customer", "seller", "admin"];

    if !VALID_ROLES.contains(&role) {
        return Err(ValidationError::new("invalid_role"));
    }

    Ok(())
}
