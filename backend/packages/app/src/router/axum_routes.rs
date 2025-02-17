use axum::{
    middleware,
    routing::{post, put},
    Router,
};
use state::axum::AppState;
use utoipa::OpenApi;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use model::web::user_request::User as UserRequest;
use model::utoipa::user::User as UserUtoipa;
use utoipa_axum::{router::OpenApiRouter, routes};


#[derive(OpenApi)]
#[openapi(
    paths(controller::axum::user::register),
    components(schemas(UserUtoipa,UserRequest))
)]
struct ApiDoc;

/// Defines user-related routes.
pub fn user_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/user", post(controller::axum::user::register))
        .route("/api/v1/login", post(controller::axum::user::login))
        .route(
            "/api/v1/user",
            put(controller::axum::user::update_profile).route_layer(
                middleware::from_fn_with_state(app_state.clone(), controller::axum::jwt::jwt_auth),
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
