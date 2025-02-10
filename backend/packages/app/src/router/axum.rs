use std::sync::Arc;

use database::database::{Connection, Sources};
use environment::Environment;
use errors::Result;
use redis::Client;
use repository::user::user_repository::UserRepository;
use service::user::user_service::UserService;
use state::axum::AppState;

use super::axum_routes::build_routes;

pub async fn run() -> Result<()> {
    let mut surreal_db = database::database::DatabaseSource {
        db_type: database::database::DatabaseType::SurrealDB,
    };

    let environment = Environment::new();

    let redis_url = format!(
        "redis://{}:{}@{}:{}",
        environment.redis_username,
        environment.redis_password,
        environment.redis_host,
        environment.redis_port
    );

    let _redis_client = match Client::open(redis_url) {
        Ok(client) => {
            println!("✅ Connection to Redis is successful!");
            client
        }
        Err(e) => {
            println!("🔥 Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    };

    let conn = Arc::new(surreal_db.connect().await?);
    let ping_db = conn.ping();

    if ping_db == *"Pong!" {
        println!("✅ {} from database!", ping_db);
    } else {
        println!("🔥 {} from database!", ping_db);
        std::process::exit(1);
    }

    let user_repository = UserRepository { db: conn.clone() };
    let user_service = UserService {
        user_repo: user_repository
    };

    let app_state = AppState {
        user_service,
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
