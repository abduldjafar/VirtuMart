use axum::{
    middleware,
    routing::{post, put},
    Router,
};
use state::axum::AppState;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

/// Defines user-related routes.
pub fn user_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/user", post(controller::axum::user::register))
        .route(
            "/api/v1/user",
            put(controller::axum::user::update_profile).route_layer(
                middleware::from_fn_with_state(app_state.clone(), controller::axum::jwt::auth),
            ),
        )
        .with_state(app_state)
}

/// Builds the complete application router with tracing enabled.
pub fn build_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(user_routes(app_state.clone()))
        .layer(TraceLayer::new_for_http())
}
