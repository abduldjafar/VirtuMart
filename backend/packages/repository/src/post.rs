use super::{DBClient, PostId, PostModel, RepositoryPostResponse, RepositoryResult};
use database::interface::DBInterface as _;
use database::model::{Id, PostGallery};
use std::sync::Arc;

#[derive(Clone)]
pub struct PostRepository {
    pub repo: Arc<DBClient>,
}

impl PostRepository {
    pub async fn is_post_empty_by_id(&self, id: &str) -> RepositoryResult<(bool, Vec<PostModel>)> {
        let repo = &self.repo;

        let data_exists = {
            let data: Vec<PostModel> = repo
                .select_where("post".to_owned(), format!("id = {}", id), "*".to_string())
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }

    pub async fn is_post_empty_by_user_id(
        &self,
        user_id: &str,
    ) -> RepositoryResult<(bool, Vec<PostModel>)> {
        let repo = &self.repo;

        let data_exists = {
            let data: Vec<PostModel> = repo
                .select_where(
                    "post".to_owned(),
                    format!("user_id = {}", user_id),
                    "*".to_string(),
                )
                .await?;
            (data.is_empty(), data)
        };

        Ok(data_exists)
    }

    pub async fn insert_data(&self, data: PostModel) -> RepositoryResult<Option<PostId>> {
        let repo = &self.repo;
        let insert_into_gym_tb: Option<PostId> =
            repo.insert_record(String::from("post"), data).await?;
        Ok(insert_into_gym_tb)
    }

    pub async fn insert_post_gallery(&self, data: PostGallery) -> RepositoryResult<Option<Id>> {
        let repo = &self.repo;
        let insert_into_ontent_gallery_tb: Option<Id> = repo
            .insert_record(String::from("post_gallery"), data)
            .await?;
        Ok(insert_into_ontent_gallery_tb)
    }

    pub async fn update_data(&self, post_id: String, data: PostModel) -> RepositoryResult<bool> {
        let repo = &self.repo;

        let update_data = repo
            .update_record(post_id, "post".to_string(), data)
            .await?;

        Ok(update_data)
    }

    pub async fn delete_data(&self, post_id: String) -> RepositoryResult<bool> {
        let repo = &self.repo;

        let update_data = repo.delete(post_id).await?;

        Ok(update_data)
    }

    pub async fn get_list(
        &self,
        page: i64,
        user_id: String,
    ) -> RepositoryResult<(bool, Vec<RepositoryPostResponse>)> {
        // Calculate start page based on given page number
        let start_page = match page {
            1 => 0,
            p if p >= 1 => ((page - 1) * 5) + 1,
            _ => 0,
        };

        let repo = &self.repo;

        let data = {
            let data: Vec<RepositoryPostResponse> = repo
                .select_where(
                    "post".to_owned(),
                    format!("user_id = {} LIMIT 5 START {}", user_id, start_page),
                    "*, (SELECT id,media_url as link FROM post_gallery WHERE content_id == $parent.id) AS post_gallery ".to_string(),
                )
                .await?;
            (data.is_empty(), data)
        };

        Ok(data)
    }
}
