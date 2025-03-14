use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::State,
    http::{header, HeaderMap, HeaderValue, Response},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde_json::{json, Value};
use validator::Validate;

use errors::{
    Error::{LoginFail, StringError, TokenError},
    Result,
};
use model::web::user::user_request::{User as UserRequest, UserLogin};
use service::{
    auth::jwt::{generate_jwt_token, save_token_data_to_redis},
    user::user_service::UserServiceTrait,
};
use state::axum::AppState;

use super::jwt::JWTAuthMiddleware;

#[utoipa::path(
    post,
    path = "/api/v1/user",
    request_body = UserRequest,
    tag = "user",
    responses(
        (status = 200, description = "User found", content_type = "text/plain", example = super::data_example::user_registered),
        (status = 404, description = "User not found", content_type = "text/plain")
    ),
)]
pub async fn register(
    State(app_state): State<Arc<AppState>>,
    payload: Json<UserRequest>,
) -> Result<impl IntoResponse> {
    let usvc = &app_state.user_service;
    payload.0.validate()?;

    let profile_registered = usvc.register_profile(payload.0).await?;

    Ok(Json(json!({
        "status": "success",
        "data": { "user": profile_registered }
    })))
}

#[utoipa::path(
    put,
    path = "/api/v1/user",
    request_body = UserRequest,
    tag = "user",
    responses(
        (status = 200, description = "User updated", content_type = "text/plain", example = super::data_example::user_registered),
        (status = 404, description = "User not found", content_type = "text/plain")
    ),
    description = "Update user information. You can update all fields or select specific fields."
)]
pub async fn update_profile(
    State(app_state): State<Arc<AppState>>,
    Extension(jwt): Extension<JWTAuthMiddleware>,
    payload: Json<Value>,
) -> Result<impl IntoResponse> {
    let usvc = &app_state.user_service;
    usvc.update_profile(&jwt.user_id, payload.0).await?;

    Ok(Json(json!({
        "status": "success",
        "data": {}
    })))
}

#[utoipa::path(
    post,
    path = "/api/v1/login",
    request_body = UserLogin,
    tag = "user",
    responses(
        (status = 200, description = "User authenticated", content_type = "text/plain", example = super::data_example::user_registered),
        (status = 404, description = "User not found", content_type = "text/plain")
    ),
    description = "Authenticate a user and generate access & refresh tokens."
)]
pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UserLogin>,
) -> Result<impl IntoResponse> {
    let env = environment::Environment::new();
    let usvc = &app_state.user_service;

    let user = usvc.login(body.email).await?;
    let is_valid = PasswordHash::new(&user.password)
        .ok()
        .and_then(|parsed_hash| {
            Argon2::default()
                .verify_password(body.password.as_bytes(), &parsed_hash)
                .ok()
        })
        .is_some();

    if !is_valid {
        return Err(LoginFail);
    }

    let user_id = user.id.to_string();

    let access_token_details = generate_jwt_token(
        user_id.clone(),
        60,
        env.access_token_private_key.to_owned(),
        &user.role,
    )
    .await?;

    let refresh_token_details = generate_jwt_token(
        user_id,
        60,
        env.refresh_token_private_key.to_owned(),
        &user.role,
    )
    .await?;

    save_token_data_to_redis(&app_state.redis_client, &access_token_details, 60).await?;
    save_token_data_to_redis(&app_state.redis_client, &refresh_token_details, 60).await?;

    let access_token = access_token_details
        .token
        .clone()
        .ok_or_else(|| TokenError("Error extracting access token".to_string()))?;

    let refresh_token = refresh_token_details
        .token
        .ok_or_else(|| TokenError("Error extracting refresh token".to_string()))?;

    let cookies = vec![
        Cookie::build(("access_token", access_token.clone()))
            .path("/")
            .max_age(time::Duration::minutes(60 * 60))
            .same_site(SameSite::Lax)
            .http_only(true),
        Cookie::build(("refresh_token", refresh_token.clone()))
            .path("/")
            .max_age(time::Duration::minutes(60 * 60))
            .same_site(SameSite::Lax)
            .http_only(true),
        Cookie::build(("logged_in", "true"))
            .path("/")
            .max_age(time::Duration::minutes(60 * 60))
            .same_site(SameSite::Lax)
            .http_only(false),
    ];

    let mut response = Response::new(
        json!({
            "status": "success",
            "data": {
                "user_id": user.id.to_string(),
                "user_role": &user.role,
                "access_token": access_token
            }
        })
        .to_string(),
    );

    let mut headers = HeaderMap::new();
    for cookie in cookies {
        headers.append(
            header::SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string())
                .map_err(|err| StringError(err.to_string()))?,
        );
    }

    response.headers_mut().extend(headers);
    Ok(response)
}
