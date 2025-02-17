use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, Response},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use errors::Result;
use model::web::user_request::{User as UserRequest, UserLogin};
use serde_json::{json, Value};
use service::{
    auth::jwt::{generate_jwt_token, save_token_data_to_redis},
    user::user_service::UserServiceTrait,
};
use state::axum::AppState;

use super::jwt::JWTAuthMiddleware;
use validator::Validate;

pub async fn register(
    State(app_state): State<Arc<AppState>>,
    payload: Json<UserRequest>,
) -> Result<impl IntoResponse> {
    let usvc = &app_state.user_service;

    payload.0.validate()?;
    let profile_registered = usvc.register_profile(payload.0).await?;

    Ok(Json(json!({
        "status": "success",
        "data":{
            "user":profile_registered
        }
    })))
}

pub async fn update_profile(
    State(app_state): State<Arc<AppState>>,
    Extension(jwt): Extension<JWTAuthMiddleware>,
    Path(user_id): Path<String>,
    payload: Json<Value>,
) -> Result<impl IntoResponse> {
    let usvc = &app_state.user_service;
    if user_id != jwt.user_id {
        return Err(errors::Error::UserUnauthorized(
            "your user id not autorized for this process".to_string(),
        ));
    }

    usvc.update_profile(&jwt.user_id, payload.0).await?;

    Ok(Json(json!({
        "status": "success",
        "data":{}
    })))
}

pub async fn login(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<UserLogin>,
) -> Result<impl IntoResponse> {
    let env = environment::Environment::new();
    let usvc = &app_state.user_service;

    let user = usvc.login(body.email).await?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        return Err(errors::Error::LoginFail);
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

    let access_cookie = Cookie::build((
        "access_token",
        access_token_details
            .token
            .clone()
            .ok_or_else(|| errors::Error::TokenError("error when extract token".to_string()))?,
    ))
    .path("/")
    .max_age(time::Duration::minutes(60 * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let refresh_cookie = Cookie::build((
        "refresh_token",
        refresh_token_details
            .token
            .ok_or_else(|| errors::Error::TokenError("error when extract token".to_string()))?,
    ))
    .path("/")
    .max_age(time::Duration::minutes(60 * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let logged_in_cookie = Cookie::build(("logged_in", "true"))
        .path("/")
        .max_age(time::Duration::minutes(60 * 60))
        .same_site(SameSite::Lax)
        .http_only(false);

    let mut response = Response::new(
        json!(
        {
            "status": "success",
            "data":{
                "user_id":user.id.to_string(),
                "user_role":&user.role ,
                "access_token": access_token_details.token.ok_or_else(|| errors::Error::TokenError("error when extract token".to_string()))?
            }
        })
        .to_string(),
    );
    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&access_cookie.to_string())
            .map_err(|err| errors::Error::StringError(err.to_string()))?,
    );
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&refresh_cookie.to_string())
            .map_err(|err| errors::Error::StringError(err.to_string()))?,
    );
    headers.append(
        header::SET_COOKIE,
        HeaderValue::from_str(&logged_in_cookie.to_string())
            .map_err(|err| errors::Error::StringError(err.to_string()))?,
    );

    response.headers_mut().extend(headers);
    Ok(response)
}
