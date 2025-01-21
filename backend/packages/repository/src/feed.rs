use super::{DBClient, FeedModel, RepositoryResult};
use database::interface::DBInterface;
use std::sync::Arc;

#[derive(Clone)]
pub struct FeedRepository {
    pub repo: Arc<DBClient>,
}

impl FeedRepository {
    pub async fn get_list(&self, page: i64) -> RepositoryResult<Vec<FeedModel>> {
        // Calculate start page based on given page number
        let start_page = match page {
            1 => 0,
            p if p >= 1 => ((page - 1) * 5) + 1,
            _ => 0,
        };

        let repo = &self.repo;

        // Construct the filter query
        let filter = format!(
            r#"
            ORDER BY updated_at DESC 
            LIMIT 5 START {}
            "#,
            start_page
        );

        // Define the data selection structure
        let data: Vec<FeedModel> = repo
            .select_where(
                format!("post {}", filter),
                "".to_string(),
                r#"
                    content,
                    user_id.user_type AS user_type,
                    user_id.username AS username,
                    gym_id.name AS gym_name,
                    
                    IF is_gym_seeker = true THEN
                        gym_seeker_id.profile_picture
                    ELSE IF is_gym = true THEN
                        gym_id.profile_picture
                    ELSE IF is_trainer = true THEN
                        trainer.profile_picture
                    ELSE
                        NULL
                    END AS profile_picture,
                    
                    IF is_gym = true THEN
                        gym_id.location_id.{latitude, longitude}
                    ELSE
                        NULL
                    END AS location,
                    (SELECT id,media_url as link FROM post_gallery WHERE content_id == $parent.id) AS post_gallery,
                    updated_at
                "#
                .to_string(),
            )
            .await?;

        Ok(data)
    }
}
