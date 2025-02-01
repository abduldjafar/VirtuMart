use async_trait::async_trait;
use model::domain::user::User;
use model::surreal_db::user::User as UserSurreal;
use super::user_repository::{UserRepository, UserRepositoryTrait};
use errors::Result;
use database::interface::DBInterface as _;


#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn insert_data(&self, data:User ) -> Result<String> {
        let result:Option<UserSurreal> = self.repo.insert_record("user",data).await?;
        Ok(result.unwrap().id.unwrap().to_string())
    }
    async fn is_data_empty_by_username(
        &self,
        data: &User,
    ) -> Result<(bool, Vec<User>)> {
        
        let data_exists = {
            let data: Vec<User> = self.repo
                .select_where(
                    "user",
                    format!("username = '{}'", data.username).as_str(),
                    "*",
                )
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }
    
}