use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{header, Request},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use environment::Environment;
use errors::{
    Error::{DatabaseErrorExecution, TokenError},
    Result,
};
use state::axum::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthMiddleware {
    pub entity_id: String,
    pub access_token_uuid: Uuid,
    pub user_type: String,
    pub user_id: String,
}

#[tracing::instrument(err, skip_all)]
pub async fn jwt_auth(
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
                .and_then(|auth_value| auth_value.strip_prefix("Bearer ").map(String::from))
        });

    // Ensure access token is present, otherwise return an error
    let access_token = option_access_token
        .ok_or_else(|| TokenError("You are not logged in, please provide a token".to_string()))?;

    // Verify JWT token using public key from environment
    let env = Environment::new();
    let access_token_details =
        service::auth::jwt::verify_jwt_token(env.access_token_public_key.to_owned(), &access_token)
            .await
            .map_err(|e| TokenError(format!("fail: {}", e)))?;

    // Parse UUID from token details
    let access_token_uuid = Uuid::parse_str(&access_token_details.token_uuid.to_string())
        .map_err(|_| TokenError("fail: Invalid token".to_string()))?;

    // Connect to Redis and retrieve user ID associated with the token UUID
    let mut redis_client = data
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| DatabaseErrorExecution(format!("Redis error: {}", e)))?;

    // Retrieve user ID from Redis based on access token UUID
    let entity_id = redis_client
        .get::<_, String>(access_token_uuid.to_string())
        .await
        .map_err(|_| TokenError("fail: Token is invalid or session has expired".to_string()))?;

    let user_type = access_token_details.user_role;
    let user_id = access_token_details.user_id;

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
