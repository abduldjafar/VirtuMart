use app::router::engine::{Cmd, EngineType};
use environment::Environment;
use errors::{Error, Result};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Load environment configuration
    let env = Environment::new();
    let app_engine = env.app_engine;

    // Determine the engine type
    let engine = match app_engine.as_str() {
        "axum" => EngineType::Axum,
        _ => return Err(Error::UnsupportedEngine(app_engine)),
    };

    info!("App starting...");

    // Run the selected engine
    engine.run().await?;

    Ok(())
}
