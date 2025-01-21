use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};
use errors::Result;
use serde_json::json;
use state::axum_state::AppState;

use super::midleware::jwt_auth::JWTAuthMiddleware;
use database::model::{
    Pagination, PayloadIdResponses, PayloadPostRequest, PayloadPostUpdateRequest,
};

pub async fn create(
    State(app_state): State<Arc<AppState>>,
    Extension(jwt): Extension<JWTAuthMiddleware>,
    payload: Json<PayloadPostRequest>,
) -> Result<impl IntoResponse> {
    // Register user profile
    let post_svc = &app_state.post_services;
    let content = payload.clone().content.clone().unwrap();
    let content_id = post_svc
        .create(jwt.entity_id, &jwt.user_type, content)
        .await?
        .unwrap();

    // Create response payload
    let payload_id_responses = PayloadIdResponses {
        id: format!("{}:{}", content_id.id.tb, content_id.id.id),
    };

    Ok(Json(json!({
        "status": "success",
        "data": payload_id_responses
    })))
}

pub async fn update(
    State(app_state): State<Arc<AppState>>,
    Extension(jwt): Extension<JWTAuthMiddleware>,
    payload: Json<PayloadPostUpdateRequest>,
) -> Result<impl IntoResponse> {
    let post_svc = &app_state.post_services;

    post_svc
        .update(
            jwt.user_id,
            payload.content_id.clone(),
            payload.content.clone(),
        )
        .await?;

    Ok(Json(json!({
        "status": "success",
        "data": {}
    })))
}

pub async fn delete(
    State(app_state): State<Arc<AppState>>,
    Extension(jwt): Extension<JWTAuthMiddleware>,
    Path(content_id): Path<String>,
) -> Result<impl IntoResponse> {
    let post_svc = &app_state.post_services;

    let is_success = post_svc.delete(jwt.user_id, content_id).await?;

    Ok(Json(json!({
        "status": "success",
        "data": is_success
    })))
}

pub async fn get(
    State(app_state): State<Arc<AppState>>, // Extract application state
    Extension(jwt): Extension<JWTAuthMiddleware>, // Extract JWT authentication details
    Query(params): Query<Pagination>,       // Extract the `page` query parameter
) -> Result<impl IntoResponse> {
    // Retrieve the gym service from the application state
    let svc = &app_state.post_services;

    // Default to page 0 if `page` parameter is not provided
    let page = params.page.unwrap_or(0);

    // Fetch data based on the specified page
    let data = svc.get_list(jwt.user_id, page).await?;

    // Return a JSON response with the profile data
    Ok(Json(json!({
        "status": "success",
        "data": data
    })))
}
