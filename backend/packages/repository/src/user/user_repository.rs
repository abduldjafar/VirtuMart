use std::sync::Arc;
use database::database::DatabaseClient;
use errors::Result;
use model::domain::user::User;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct UserRepository {
    pub db: Arc<DatabaseClient>,
}


#[async_trait]
pub trait UserRepositoryTrait {
    async fn insert_data(&self, data:User ) -> Result<String>;
    
}