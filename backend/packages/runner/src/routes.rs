use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use router::axum_router::{
    auth, feed, gym, gymseeker, location, midleware::jwt_auth::auth, post, trainer, upload,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use state::axum_state::AppState;

pub fn gym_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/gym", post(gym::register))
        .route(
            "/api/v1/gym",
            get(gym::get_profile)
                .put(gym::update_profile)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .with_state(app_state)
}

pub fn trainer_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/v1/trainer",
            post(trainer::register)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/api/v1/trainer",
            get(trainer::get_profile)
                .put(trainer::update_profile)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .with_state(app_state)
}

pub fn auth_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/login", post(auth::login_user))
        .route("/api/v1/verify/:id", get(auth::verify_user))
        .route(
            "/api/v1/logout",
            get(auth::logout_user)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .with_state(app_state)
}

pub fn gymnast_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/gymseeker", post(gymseeker::register))
        .route(
            "/api/v1/gymseeker",
            get(gymseeker::get_profile)
                .put(gymseeker::update_profile)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .with_state(app_state)
}

pub fn upload_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/upload/:gallery_type", post(upload::upload_images))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .with_state(app_state)
}

pub fn content_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/v1/content",
            post(post::create)
                .put(post::update)
                .delete(post::delete)
                .get(post::get),
        )
        .route("/api/v1/content/:content_id", delete(post::delete))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .with_state(app_state)
}

pub fn feed_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/feed", get(feed::get_profile))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .with_state(app_state)
}

pub fn location_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/location", put(location::update_location))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .with_state(app_state)
}

pub fn build_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(gym_routes(app_state.clone()))
        .merge(auth_routes(app_state.clone()))
        .merge(gymnast_routes(app_state.clone()))
        .merge(upload_routes(app_state.clone()))
        .merge(trainer_routes(app_state.clone()))
        .merge(content_routes(app_state.clone()))
        .merge(feed_routes(app_state.clone()))
        .merge(location_routes(app_state))
        .layer(TraceLayer::new_for_http())
}
