use axum::{
    middleware,
    routing::{post, put},
    Router,
};
use controller::axum::user::register;
use model::utoipa::user::User as UserUtoipa;
use model::web::user_request::User as UserRequest;
use state::axum::AppState;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(controller::axum::user::register),
    components(schemas(UserUtoipa, UserRequest))
)]
struct ApiDoc;

/// Defines user-related routes.
pub fn user_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/v1/user", post(register))
        .route("/api/v1/login", post(controller::axum::user::login))
        .route("/api/v1/user", put(controller::axum::user::update_profile))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            controller::axum::jwt::jwt_auth,
        ))
        .with_state(app_state)
}

/// Get health of the API.
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

/// Builds the complete application router with tracing enabled.
pub fn build_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    let (router, api) = OpenApiRouter::<Arc<AppState>>::with_openapi(ApiDoc::openapi())
        .routes(routes!(health))
        .split_for_parts();

    let swagger_router = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone());

    let router: Router<Arc<AppState>> = router
        .merge(user_routes(app_state.clone())) // Ensure correct type
        .merge(swagger_router)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    router
}
