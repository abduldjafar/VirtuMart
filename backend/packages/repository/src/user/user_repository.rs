use std::sync::Arc;
use database::database::DatabaseClient;
use errors::Result;
use model::domain::user::User;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct UserRepository {
    pub repo: Arc<DatabaseClient>,
}


#[async_trait]
pub trait UserRepositoryTrait {
    async fn insert_data(&self, data:User ) -> Result<String>;
    async fn is_data_empty_by_username(
        &self,
        data: &User,
    ) -> Result<(bool, Vec<User>)> ;
    
}