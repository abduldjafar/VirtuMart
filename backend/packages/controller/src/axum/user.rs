use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use errors::Result;
use model::web::user_request::User as UserRequest;
use serde_json::json;
use service::user::user_service::UserServiceTrait;
use state::axum::AppState;

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
