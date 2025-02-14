use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Extension, Json};
use errors::Result;
use model::web::user_request::User as UserRequest;
use serde_json::{json, Value};
use service::user::user_service::UserServiceTrait;
use state::axum::AppState;

use super::jwt::JWTAuthMiddleware;

pub async fn register(
    State(app_state): State<Arc<AppState>>,
    payload: Json<UserRequest>,
) -> Result<impl IntoResponse> {
    let usvc = &app_state.user_service;

    match usvc.register_profile(payload.into()).await {
        Ok(response) => Ok(Json(json!({
            "status": "success",
            "data": response
        }))),
        Err(e) => {
            let error_message = format!("{:?}", e);
            Ok(Json(json!({
                "status": "error",
                "error_message": error_message,
            })))
        }
    }
}

pub async fn update_profile(
    State(app_state): State<Arc<AppState>>,
    Extension(jwt): Extension<JWTAuthMiddleware>,
    payload: Json<Value>,
) -> Result<impl IntoResponse> {
    let usvc = &app_state.user_service;

    match usvc.update_profile(&jwt.user_id, payload.0).await {
        Ok(response) => Ok(Json(json!({
            "status": "success",
            "data": response
        }))),
        Err(e) => {
            let error_message = format!("{:?}", e);
            Ok(Json(json!({
                "status": "error",
                "error_message": error_message,
            })))
        }
    }
}
