use std::sync::Arc;

use super::midleware::jwt_auth::JWTAuthMiddleware;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use database::model::{
    PayloadGymProfileResponses, PayloadGymRequest, PayloadIdResponses, PayloadUser, User,
};
use errors::Result;
use rand_core::OsRng;
use serde_json::json;
use services::email::EmailServices;
use state::axum_state::AppState;
use uuid::Uuid;

pub async fn register(
    State(app_state): State<Arc<AppState>>,
    payload: Json<PayloadUser>,
) -> Result<impl IntoResponse> {
    // Generate salt and hash the password
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)?
        .to_string();

    let verify_token = Uuid::new_v4().to_string();

    // Create user with hashed password
    let user = User {
        id: None,
        username: payload.username.clone(),
        user_type: String::from("gym"),
        email: payload.email.clone(),
        created_at: None,
        updated_at: None,
        password: hashed_password,
        verified: false,
        verified_token: Some(verify_token.clone()),
    };

    // Register user profile
    let gym_svc = &app_state.gym_services;
    let user_id = gym_svc.register_profile(user.clone()).await?.unwrap();

    // Create response payload
    let payload_id_responses = PayloadIdResponses {
        id: format!("{}:{}", user_id.id.tb, user_id.id.id),
    };

    EmailServices::send_verification_email(&user.username, &user.email, &verify_token).await?;

    Ok(Json(json!({
        "status": "success",
        "data": payload_id_responses
    })))
}

pub async fn get_profile(
    State(app_state): State<Arc<AppState>>, // Extract application state
    Extension(jwt): Extension<JWTAuthMiddleware>, // Extract JWT authentication details
) -> Result<impl IntoResponse> {
    // Retrieve the gym service from the application state
    let svc = &app_state.gym_services;

    let (is_empty, _) = svc.is_gym_data_empty_by_id(&jwt.entity_id).await?;

    if is_empty {
        return Err(errors::Error::DataNotAvaliable(format!(
            "user {} not available",
            &jwt.entity_id
        )));
    }

    // Fetch profile details using the entity ID from the JWT
    let data = svc.profile_details(jwt.entity_id).await?;

    // Construct the response payload with profile details
    let profile = PayloadGymProfileResponses {
        name: data.name,
        description: data.description,
        profile_picture: data.profile_picture,
        created_at: data.created_at,
        updated_at: data.updated_at,
    };

    // Return a JSON response with the profile data
    Ok(Json(json!({
        "status": "success",
        "data": profile
    })))
}

pub async fn update_profile(
    State(app_state): State<Arc<AppState>>,
    Extension(jwt): Extension<JWTAuthMiddleware>,
    payload: Json<PayloadGymRequest>,
) -> Result<impl IntoResponse> {
    let svc = &app_state.gym_services;

    let (is_empty, _) = svc.is_gym_data_empty_by_id(&jwt.entity_id).await?;

    if is_empty {
        return Err(errors::Error::DataNotAvaliable(format!(
            "user {} not available",
            &jwt.entity_id
        )));
    }

    svc.update_profile(&payload, &jwt.entity_id).await?;

    Ok(Json(json!({
        "status": "success",
        "data":{}
    })))
}
