use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;

use database::model::Pagination;
use state::axum_state::AppState;

use super::midleware::jwt_auth::JWTAuthMiddleware;
use errors::Result;

pub async fn get_profile(
    State(app_state): State<Arc<AppState>>, // Extract application state
    Extension(_jwt): Extension<JWTAuthMiddleware>, // Extract JWT authentication details
    Query(params): Query<Pagination>,       // Extract the `page` query parameter
) -> Result<impl IntoResponse> {
    // Retrieve the gym service from the application state
    let svc = &app_state.feed_services;

    // Default to page 0 if `page` parameter is not provided
    let page = params.page.unwrap_or(0);

    // Fetch data based on the specified page
    let data = svc.get_list(page).await?;

    // Return a JSON response with the profile data
    Ok(Json(json!({
        "status": "success",
        "data": data
    })))
}
