use errors::Result;
use app_runner::engine::{Cmd, EngineType};

#[tokio::main]
async fn main() -> Result<()> {
    let engine = EngineType::Axum;
    engine.run().await?;
    Ok(())
}
