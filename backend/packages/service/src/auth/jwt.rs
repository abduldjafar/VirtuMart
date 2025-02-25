use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use errors::{Error::DatabaseErrorExecution, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use model::authorization::token::{TokenClaims, TokenDetails};
use redis::{AsyncCommands, Client};
use uuid::Uuid;

/// Generates a JWT token for a given user.
pub async fn generate_jwt_token(
    user_id: String,
    ttl: i64,
    private_key: String,
    user_role: &str,
) -> Result<TokenDetails> {
    let decoded_private_key = String::from_utf8(general_purpose::STANDARD.decode(private_key)?)?;
    let now = Utc::now();

    let token_details = TokenDetails {
        user_id: user_id.clone(),
        token_uuid: Uuid::new_v4(),
        expires_in: Some((now + chrono::Duration::minutes(ttl)).timestamp()),
        token: None,
        user_role: user_role.to_string(),
    };

    let claims = TokenClaims {
        sub: user_id,
        token_uuid: token_details.token_uuid.to_string(),
        exp: token_details.expires_in.unwrap(),
        iat: now.timestamp(),
        nbf: now.timestamp(),
        user_role: user_role.to_string(),
    };

    let token = encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(decoded_private_key.as_bytes())?,
    )?;

    Ok(TokenDetails {
        token: Some(token),
        ..token_details
    })
}

/// Verifies a JWT token and extracts token details.
pub async fn verify_jwt_token(public_key: String, token: &str) -> Result<TokenDetails> {
    let decoded_public_key = String::from_utf8(general_purpose::STANDARD.decode(public_key)?)?;
    let validation = Validation::new(Algorithm::RS256);

    let decoded = decode::<TokenClaims>(
        token,
        &DecodingKey::from_rsa_pem(decoded_public_key.as_bytes())?,
        &validation,
    )?;

    Ok(TokenDetails {
        user_id: decoded.claims.sub,
        token_uuid: Uuid::parse_str(&decoded.claims.token_uuid)?,
        expires_in: None,
        token: None,
        user_role: decoded.claims.user_role,
    })
}

/// Saves token data to Redis with a specified expiration time.
pub async fn save_token_data_to_redis(
    client: &Client,
    token_details: &TokenDetails,
    max_age: i64,
) -> Result<()> {
    let mut redis_client = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| DatabaseErrorExecution("Failed to connect to Redis".to_string()))?;

    redis_client
        .set_ex(
            token_details.token_uuid.to_string(), // Key
            token_details.user_id.to_string(),    // Value
            (max_age * 60) as u64,                // Expiration time in seconds
        )
        .await
        .map_err(|e| DatabaseErrorExecution(format!("Failed to store token in Redis: {:?}", e)))
}

/// Deletes a token from Redis.
pub async fn delete_token_data_in_redis(client: &Client, token: String) -> Result<()> {
    let mut redis_client = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| DatabaseErrorExecution("Failed to connect to Redis".to_string()))?;

    redis_client
        .del(token)
        .await
        .map_err(|e| DatabaseErrorExecution(format!("Failed to delete token from Redis: {:?}", e)))
}
