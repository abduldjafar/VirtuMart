use app::router::engine::{Cmd, EngineType};
use environment::Environment;
use errors::{Error, Result};
use tracing::{debug, info}; // Assume Error is a custom error type

#[tokio::main]
async fn main() -> Result<()> {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    
    let env = Environment::new();
    let app_engine = env.app_engine;

    let engine = match app_engine.as_str() {
        "axum" => EngineType::Axum,
        _ => return Err(Error::UnsupportedEngine(app_engine)),
    };

    info!("app starting...");
    engine.run().await?;

    
    Ok(())
}
