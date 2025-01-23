use redis::Client;

#[derive(Clone)]
pub struct AppState {
    pub redis_client: Client,
}
