use environment::Environment;
use file_storage::interface::FileStorage;
use redis::Client;

#[derive(Clone)]
pub struct AppState {
    pub redis_client: Client,
}
