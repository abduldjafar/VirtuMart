use super::{routes::build_routes, FileStorage, StoragePlatform};
use database::{
    self,
    db::{Connection, Sources},
};
use environment::Environment;
use errors::Result;
use redis::Client;
use services::{
    auth::AuthServices, email::EmailServices, feed::FeedServices, gym::GymServices,
    gymseeker::GymSeekerServices, location::LocationServices, post::PostServices,
    trainer::TrainerServices,
};
use state::axum_state::AppState;
use std::sync::Arc;

use repository::{
    feed::FeedRepository, gym::GymRepository, gymseeker::GymSeekerRepository,
    location::LocationRepository, post::PostRepository, trainer::TrainerRepository,
    user::UserRepository,
};

pub async fn run() -> Result<()> {
    let mut surreal_db = database::db::DatabaseSource {
        db_type: database::db::DatabaseType::SurrealDB,
    };

    let cloud_storage = FileStorage {
        platform: StoragePlatform::Google,
    };

    let environment = Environment::new();

    let redis_url = format!(
        "redis://{}:{}@{}:{}",
        environment.redis_username,
        environment.redis_password,
        environment.redis_host,
        environment.redis_port
    );

    let redis_client = match Client::open(redis_url) {
        Ok(client) => {
            println!("✅ Connection to Redis is successful!");
            client
        }
        Err(e) => {
            println!("🔥 Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    };
    // Connect to the database
    let conn = Arc::new(surreal_db.connect().await?);
    let ping_db = conn.ping();

    if ping_db == *"Pong!" {
        println!("✅ {} from database!", ping_db);
    } else {
        println!("🔥 {} from database!", ping_db);
        std::process::exit(1);
    }

    let gym_repository = GymRepository { repo: conn.clone() };
    let user_repository = UserRepository { repo: conn.clone() };
    let gymseeker_repository = GymSeekerRepository { repo: conn.clone() };
    let location_repository = LocationRepository { repo: conn.clone() };
    let trainer_repository = TrainerRepository { repo: conn.clone() };
    let post_repository = PostRepository { repo: conn.clone() };
    let feed_repository = FeedRepository { repo: conn.clone() };

    let gym_services = GymServices {
        gym_repository: gym_repository.clone(),
        user_repository: user_repository.clone(),
        location_repository: location_repository.clone(),
    };

    let gymseeker_services = GymSeekerServices {
        repository: gymseeker_repository.clone(),
        user_repository: user_repository.clone(),
    };
    let auth_services = AuthServices {
        repo: conn.clone(),
        user_repository: user_repository.clone(),
    };

    let location_services = LocationServices {
        location_repository: location_repository.clone(),
        user_repository: user_repository.clone(),
        gym_repository: gym_repository.clone(),
    };

    let trainer_services = TrainerServices {
        trainer_repository: trainer_repository.clone(),
        user_repository: user_repository.clone(),
        gym_repository: gym_repository.clone(),
    };

    let post_services = PostServices {
        trainer_repository: trainer_repository.clone(),
        user_repository: user_repository.clone(),
        gym_repository: gym_repository.clone(),
        gym_seeker_repository: gymseeker_repository.clone(),
        post_repository: post_repository.clone(),
        location_repository: location_repository.clone(),
    };

    let feed_services = FeedServices { feed_repository };

    let environment_cloned = environment.clone();

    let email_services = EmailServices {};
    let app_state = AppState {
        gym_services,
        gymseeker_services,
        auth_services,
        location_services,
        trainer_services,
        post_services,
        feed_services,
        email_services,
        redis_client,
        environment: environment_cloned,
        cloud_storage,
    };

    let shared_state = Arc::new(app_state);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &environment.app_port))
        .await
        .unwrap();
    axum::serve(listener, build_routes(shared_state))
        .await
        .unwrap();

    Ok(())
}
