use super::axum;
use async_trait::async_trait;
use errors::Result;

pub enum EngineType {
    Axum,
    // Add other engine types here, e.g., Actix
}

#[async_trait]
pub trait Cmd {
    async fn run(&self) -> Result<()>;
}

#[async_trait]
impl Cmd for EngineType {
    async fn run(&self) -> Result<()> {
        match &self {
            EngineType::Axum => {
                axum::run().await?;
                Ok(())
            }
        }
    }
}