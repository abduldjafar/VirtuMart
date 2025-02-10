use std::sync::Arc;

use axum::{routing::post, Router};
use state::axum::AppState;
use tower_http::trace::TraceLayer;


pub fn user_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/gym", post(controller::axum::user::register))
        .with_state(app_state)
}


pub fn build_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(user_routes(app_state.clone()))
        .layer(TraceLayer::new_for_http())
}