use std::sync::Arc;

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use chrono::Utc;
use rand_core::OsRng;
use serde_json::json;
use services::email::EmailServices;
use state::axum_state::AppState;
use uuid::Uuid;

use super::midleware::jwt_auth::JWTAuthMiddleware;
use database::model::{
    PayloadGymSeekerProfileResponse, PayloadGymSeekerRequest, PayloadIdResponses, PayloadUser, User,
};
use errors::Result;

pub async fn register(
    State(app_state): State<Arc<AppState>>,
    payload: Json<PayloadUser>,
) -> Result<impl IntoResponse> {
    // Generate salt and hash the password

    let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime::from(Utc::now());

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)?
        .to_string();

    let verify_token = Uuid::new_v4().to_string();

    // Create user with hashed password
    let user = User {
        id: None,
        username: payload.username.clone(),
        user_type: String::from("gym_seeker"),
        email: payload.email.clone(),
        created_at: Some(time_now.clone()),
        updated_at: Some(time_now.clone()),
        password: hashed_password,
        verified: false,
        verified_token: Some(verify_token.clone()),
    };

    // Register user profile
    let svc = &app_state.gymseeker_services;
    let user_id = svc.register_profile(&user).await?.unwrap();

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
    State(app_state): State<Arc<AppState>>,
    Extension(jwt): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse> {
    // Get profile details
    let svc = &app_state.gymseeker_services;

    let (is_empty, _) = svc.is_gym_seeker_data_empty_by_id(&jwt.entity_id).await?;

    if is_empty {
        return Err(errors::Error::DataNotAvaliable(format!(
            "user {} not available",
            &jwt.entity_id
        )));
    }

    let data = svc.profile_details(jwt.entity_id).await?;
    let payload = PayloadGymSeekerProfileResponse {
        birth_date: data.birth_date,
        sex: data.sex,
        profile_picture: data.profile_picture,
        fitness_goals: data.fitness_goals,
        preferred_workout_time: data.preferred_workout_time,
        gym_preferences: data.gym_preferences,
        membership_status: data.membership_status,
        bio: data.bio,
        created_at: data.created_at,
        updated_at: data.updated_at,
        name: data.name,
    };

    Ok(Json(json!({
        "status": "success",
        "data":payload
    })))
}

pub async fn update_profile(
    State(app_state): State<Arc<AppState>>,
    Extension(jwt): Extension<JWTAuthMiddleware>,
    payload: Json<PayloadGymSeekerRequest>,
) -> Result<impl IntoResponse> {
    let svc = &app_state.gymseeker_services;
    let (is_empty, _) = svc.is_gym_seeker_data_empty_by_id(&jwt.entity_id).await?;

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
