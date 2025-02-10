use app::router::engine::{Cmd, EngineType};
use environment::Environment;
use errors::{Result, Error}; // Assume Error is a custom error type

#[tokio::main]
async fn main() -> Result<()> {
    let env = Environment::new();
    let app_engine = env.app_engine;

    let engine = match app_engine.as_str() {
        "axum" => EngineType::Axum,
        _ => return Err(Error::UnsupportedEngine(app_engine)), // Proper error handling
    };

    engine.run().await?;
    Ok(())
}
