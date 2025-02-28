use std::sync::Arc;

use axum::{
    middleware,
    routing::{post, put},
    Router,
};
use tower_http::trace::TraceLayer;

use controller::axum::{
    jwt::jwt_auth,
    user::{login, register, update_profile},
};
use model::{
    utoipa::user::User as UserUtoipa,
    web::user::user_request::{User as UserRequest, UserLogin},
};
use state::axum::AppState;

use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        controller::axum::user::register,
        controller::axum::user::update_profile,
        controller::axum::user::login,
    ),
    components(schemas(UserUtoipa, UserRequest, UserLogin))
)]
struct ApiDoc;

/// Defines user-related routes.
pub fn user_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/user", post(register))
        .route("/api/v1/login", post(login))
        .route(
            "/api/v1/user",
            put(update_profile).layer(middleware::from_fn_with_state(app_state.clone(), jwt_auth)),
        )
        .with_state(app_state)
}

/// API health check endpoint.
#[utoipa::path(
    method(get, head),
    path = "/api/health",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
async fn health() -> &'static str {
    "ok"
}

/// Builds the complete application router with tracing and OpenAPI documentation.
pub fn build_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    let (router, api) = OpenApiRouter::<Arc<AppState>>::with_openapi(ApiDoc::openapi())
        .routes(routes!(health))
        .split_for_parts();

    let swagger_router = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone());

    router
        .merge(user_routes(app_state.clone()))
        .merge(swagger_router)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
