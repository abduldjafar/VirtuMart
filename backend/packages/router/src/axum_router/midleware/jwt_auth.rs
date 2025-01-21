use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{header, Request},
    middleware::Next,
    response::IntoResponse,
};

use authorization::{self};
use axum_extra::extract::cookie::CookieJar;
use environment::Environment;
use errors::{self, Result};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use state::axum_state::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthMiddleware {
    pub entity_id: String,
    pub access_token_uuid: uuid::Uuid,
    pub user_type: String,
    pub user_id: String,
}

pub async fn auth(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse> {
    // Attempt to retrieve the access token from cookie or authorization header
    let option_access_token = cookie_jar
        .get("access_token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    auth_value
                        .strip_prefix("Bearer ")
                        .map(|token| token.to_owned())
                })
        });

    // Ensure access token is present, otherwise return an error
    let access_token = match option_access_token {
        Some(token) => token,
        None => {
            return Err(errors::Error::TokenError(
                "You are not logged in, please provide token".to_string(),
            ))
        }
    };

    // Verify JWT token using public key from environment
    let env = Environment::new();
    let access_token_details = match authorization::jwt::verify_jwt_token(
        env.access_token_public_key.to_owned(),
        &access_token,
    )
    .await
    {
        Ok(token_details) => token_details,
        Err(e) => return Err(errors::Error::TokenError(format!("fail: {}", e))),
    };

    // Parse UUID from token details
    let access_token_uuid =
        match uuid::Uuid::parse_str(&access_token_details.token_uuid.to_string()) {
            Ok(token) => token,
            Err(_) => return Err(errors::Error::TokenError("fail: Invalid token".to_string())),
        };

    // Connect to Redis and retrieve user ID associated with the token UUID
    let mut redis_client = match data.redis_client.get_multiplexed_async_connection().await {
        Ok(client) => client,
        Err(error) => {
            return Err(errors::Error::DatabaseError(format!(
                "Redis error: {}",
                error
            )))
        }
    };

    // Retrieve user ID from Redis based on access token UUID
    let entity_id = match redis_client
        .get::<_, String>(access_token_uuid.clone().to_string())
        .await
    {
        Ok(token) => token,
        Err(_) => {
            return Err(errors::Error::TokenError(
                "fail: Token is invalid or session has expired".to_string(),
            ))
        }
    };

    let user_type = access_token_details.user_type;
    let user_id = access_token_details.main_user_id;
    // Insert authenticated user details into request extensions
    req.extensions_mut().insert(JWTAuthMiddleware {
        access_token_uuid,
        entity_id,
        user_type,
        user_id,
    });

    // Continue handling the request with the next middleware or handler
    Ok(next.run(req).await)
}
