use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::extract::Path;
use axum::http::{header, HeaderMap, Response};
use axum::Extension;
use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use serde_json::json;

use authorization;
use authorization::jwt::{
    delete_token_data_in_redis, generate_jwt_token, save_token_data_to_redis,
};
use database::model::LoginUserSchema;
use environment::Environment;
use state::axum_state::AppState;

use errors::Result;

use super::midleware::jwt_auth::JWTAuthMiddleware;

pub async fn verify_user(
    State(app_state): State<Arc<AppState>>,
    Path(token): Path<String>,
) -> Result<impl IntoResponse> {
    let auth_svc = &app_state.auth_services;

    let verify_user = auth_svc.user_verification(token).await?;

    if verify_user {
        let response = Response::new(json!({"status": "success"}).to_string());

        Ok(response)
    } else {
        Err(errors::Error::UserUnauthorized(
            json!({"status": "failed", "error": "Invalid verification token"}).to_string(),
        ))
    }
}

pub async fn login_user(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse> {
    let auth_svc = &app_state.auth_services;
    let gym_svc = &app_state.gym_services;
    let gymseeker_svc = &app_state.gymseeker_services;
    let trainer_svc = &app_state.trainer_services;
    let env = app_state.environment.clone();

    let user = auth_svc.login(body.email).await?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        return Err(errors::Error::LoginFail);
    }

    let user_id = {
        if user.user_type == "gym" {
            let data = gym_svc.profile_details(user.id.to_string()).await?;
            data.id
        } else if user.user_type == "gym_seeker" {
            let data = gymseeker_svc.profile_details(user.id.to_string()).await?;
            data.id
        } else if user.user_type == "trainer" {
            let data = trainer_svc.profile_details(user.id.to_string()).await?;
            data.id
        } else {
            return Err(errors::Error::DataExist(format!("{} not found", user.id)));
        }
    };
    let main_user_id = user.id.to_string();
    let access_token_details = generate_jwt_token(
        user_id.clone(),
        60,
        env.access_token_private_key.to_owned(),
        &user.user_type,
        &main_user_id,
    )
    .await?;

    let refresh_token_details = generate_jwt_token(
        user_id,
        60,
        env.refresh_token_private_key.to_owned(),
        &user.user_type,
        &main_user_id,
    )
    .await?;

    save_token_data_to_redis(&app_state, &access_token_details, 60).await?;
    save_token_data_to_redis(&app_state, &refresh_token_details, 60).await?;

    let access_cookie = Cookie::build((
        "access_token",
        access_token_details.token.clone().unwrap_or_default(),
    ))
    .path("/")
    .max_age(time::Duration::minutes(60 * 60))
    .same_site(SameSite::Lax)
    .http_only(true);

    let refresh_cookie = Cookie::build((
        "refresh_token",
        refresh_token_details.token.unwrap_or_default(),
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
                "user_type":&user.user_type ,
                "access_token": access_token_details.token.unwrap()
            }
        })
        .to_string(),
    );
    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        access_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        refresh_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        logged_in_cookie.to_string().parse().unwrap(),
    );

    response.headers_mut().extend(headers);
    Ok(response)
}

pub async fn logout_user(
    cookie_jar: CookieJar,
    Extension(jwt): Extension<JWTAuthMiddleware>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    let message = "Token is invalid or session has expired";
    let environment = Environment::new();

    let refresh_token = cookie_jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": message
            });
            errors::Error::TokenError(error_response.to_string())
        })?;

    let refresh_token_details =
        authorization::jwt::verify_jwt_token(environment.refresh_token_public_key, &refresh_token)
            .await?;

    delete_token_data_in_redis(&data, refresh_token_details.token_uuid.to_string()).await?;
    delete_token_data_in_redis(&data, jwt.access_token_uuid.to_string()).await?;

    let access_cookie = Cookie::build(("access_token", ""))
        .path("/")
        .max_age(time::Duration::minutes(-1))
        .same_site(SameSite::Lax)
        .http_only(true);
    let refresh_cookie = Cookie::build(("refresh_token", ""))
        .path("/")
        .max_age(time::Duration::minutes(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let logged_in_cookie = Cookie::build(("logged_in", "true"))
        .path("/")
        .max_age(time::Duration::minutes(-1))
        .same_site(SameSite::Lax)
        .http_only(false);

    let mut headers = HeaderMap::new();
    headers.append(
        header::SET_COOKIE,
        access_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        refresh_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        logged_in_cookie.to_string().parse().unwrap(),
    );

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response.headers_mut().extend(headers);
    Ok(response)
}
