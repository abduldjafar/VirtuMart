use futures::{stream, StreamExt};

use database::model::{PayloadFeed, PostGalleryForFeedDeserialize};
use errors::Result;
use repository::feed::FeedRepository;

#[derive(Clone)]
pub struct FeedServices {
    pub feed_repository: FeedRepository,
}

impl FeedServices {
    pub async fn get_list(&self, page: i64) -> Result<Vec<PayloadFeed>> {
        let feeds = self.feed_repository.get_list(page).await?;

        let deserialized_feeds = stream::iter(feeds)
            .map(|payload| {
                let post_gallery_deserialized: Vec<PostGalleryForFeedDeserialize> = payload
                    .clone()
                    .post_gallery
                    .unwrap()
                    .iter()
                    .map(|gallery| PostGalleryForFeedDeserialize {
                        id: gallery.clone().id.unwrap().to_string(),
                        link: gallery.clone().link,
                    })
                    .collect();

                PayloadFeed {
                    content: payload.content,
                    gym_name: payload.gym_name,
                    location: payload.location,
                    profile_picture: payload.profile_picture,
                    updated_at: payload.updated_at,
                    post_gallery: Some(post_gallery_deserialized),
                    user_type: payload.user_type,
                    username: payload.username,
                }
            })
            .collect::<Vec<_>>()
            .await;
        Ok(deserialized_feeds)
    }
}
