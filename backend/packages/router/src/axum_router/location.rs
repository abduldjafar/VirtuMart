use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use serde_json::json;

use database::model::PayloadLocationRequest;

use super::midleware::jwt_auth::JWTAuthMiddleware;
use errors::Result;
use state::axum_state::AppState;

pub async fn update_location(
    State(app_state): State<Arc<AppState>>,
    Extension(jwt): Extension<JWTAuthMiddleware>,
    payload: Json<PayloadLocationRequest>,
) -> Result<impl IntoResponse> {
    let gym_svc = &app_state.gym_services;
    let location_svc = &app_state.location_services;
    println!("{}", &jwt.entity_id);

    let (is_empty, gym_temp) = gym_svc.is_gym_data_empty_by_id(&jwt.entity_id).await?;

    if is_empty {
        return Err(errors::Error::DataNotAvaliable(format!(
            "user {} not available",
            &jwt.entity_id
        )));
    }

    let gym = gym_temp.first().unwrap();

    location_svc
        .update_location(&payload, &gym.id.clone().unwrap().to_string())
        .await?;

    Ok(Json(json!({
        "status": "success",
        "data":{}
    })))
}
