use file_storage::interface::FileStorage;
use file_storage::interface::StoragePlatform;
use database::{
    self,
    db::{Connection, Sources},
};
use environment::Environment;
use errors::Result;
use redis::Client;
use state::axum_state::AppState;
use std::sync::Arc;

pub async fn run() -> Result<()> {
    let mut surreal_db = database::db::DatabaseSource {
        db_type: database::db::DatabaseType::SurrealDB,
    };
    
    let _cloud_storage = FileStorage {
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


    let _environment_cloned = environment.clone();

    let app_state = AppState {
        redis_client,
    };

    let _shared_state = Arc::new(app_state);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let _listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &environment.app_port))
        .await
        .unwrap();
    //axum::serve(listener, build_routes(shared_state))
        //.await
       // .unwrap();

    Ok(())
}
