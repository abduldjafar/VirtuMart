use super::midleware::jwt_auth::JWTAuthMiddleware;
use axum::{
    extract::{Multipart, Path, Query, State},
    response::{IntoResponse, Json},
    Extension,
};
use chrono::Utc;
use database::model::{ContentGalery, PayloadGymRequest, PayloadGymSeekerRequest, PostGallery};
use environment::Environment;
use errors::Result;
use file_storage::interface::FileStorageInterface;
use futures::{stream::FuturesUnordered, StreamExt};
use serde_json::json;
use state::axum_state::AppState;
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

pub async fn upload_images(
    State(app_state): State<Arc<AppState>>,
    Extension(jwt): Extension<JWTAuthMiddleware>,
    Path(gallery_type): Path<String>,
    Query(params): Query<ContentGalery>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    // Retrieve environment and user type
    let environment = Environment::new().env;
    let user_type = jwt.user_type;
    let user_id = jwt.entity_id.replace(":", "_");
    let id = jwt.entity_id;
    let main_user_id = jwt.user_id;
    let gym_svc = &app_state.gym_services;
    let gymseeker_svc = &app_state.gymseeker_services;
    let post_svc = &app_state.post_services;

    // Vector to collect uploaded file names for the response
    let mut uploaded_files = Vec::new();

    // Process each field in the multipart form (supports multiple file uploads)

    while let Some(field) = multipart.next_field().await.unwrap() {
        let storage_client = &app_state.cloud_storage;

        let uuid = Uuid::new_v4().to_string().replace("-", "");

        // Get the current timestamp as an integer (seconds since UNIX epoch)
        let timestamp = Utc::now().timestamp().to_string();

        // Extract file details
        let file_name = format!("{}{}", uuid, timestamp);
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        let content = field.bytes().await.unwrap();

        // Construct the object path based on user type and environment
        let object_name = format!(
            "{}/{}/{}/{}/{}",
            environment, user_type, user_id, gallery_type, file_name
        );

        // Upload the file to storage
        let link = storage_client
            .upload(content, &content_type, &object_name)
            .await?;

        // Store uploaded file name for the response
        uploaded_files.push(link);
    }

    let profile_picture = uploaded_files.first().unwrap().to_string();
    let time_now: surrealdb::sql::Datetime = surrealdb::sql::Datetime::from(Utc::now());

    match gallery_type.as_str() {
        "profile" => match user_type.as_str() {
            "gym" => {
                let gym_profile = gym_svc.profile_details(id.clone()).await?;
                let payload = PayloadGymRequest {
                    name: Some(gym_profile.name),
                    description: Some(gym_profile.description),
                    profile_picture: Some(profile_picture.clone()),
                    owner_id: gym_profile.owner_id,
                    location_id: gym_profile.location_id,
                    created_at: gym_profile.created_at,
                    updated_at: Some(time_now),
                };
                gym_svc.update_profile(&payload, &id).await?;
            }
            "gym_seeker" => {
                let gymseeker_profile = gymseeker_svc.profile_details(id.clone()).await?;
                let payload = PayloadGymSeekerRequest {
                    birth_date: gymseeker_profile.birth_date,
                    sex: Some(gymseeker_profile.sex),
                    user_id: gymseeker_profile.user_id,
                    profile_picture: Some(profile_picture.clone()),
                    fitness_goals: Some(gymseeker_profile.fitness_goals),
                    preferred_workout_time: Some(gymseeker_profile.preferred_workout_time),
                    gym_preferences: Some(gymseeker_profile.gym_preferences),
                    membership_status: Some(gymseeker_profile.membership_status),
                    bio: Some(gymseeker_profile.bio),
                    created_at: gymseeker_profile.created_at,
                    updated_at: Some(time_now),
                    name: Some(gymseeker_profile.name),
                };
                gymseeker_svc.update_profile(&payload, &id).await?;
            }
            _ => (),
        },
        "gallery" => {
            let content_id = params.content_id.unwrap_or("".to_string());
            let (is_empty, post) = post_svc.is_post_data_empty_by_id(&content_id).await?;
            if is_empty {
                return Err(errors::Error::DataNotAvaliable(format!(
                    "post:{} not available",
                    &content_id
                )));
            }

            println!("{}", id);
            let post_id = post.first().unwrap().id.clone();
            let post_user_id = post.first().unwrap().user_id.clone().unwrap().to_string();

            if main_user_id != post_user_id {
                return Err(errors::Error::UserUnauthorized(
                    "You are not authorized to update this post".to_string(),
                ));
            }

            uploaded_files
                .iter()
                .map(|link| {
                    let post_gallery = PostGallery {
                        id: None,
                        content_id: post_id.clone(),
                        media_url: Some(link.clone()),
                        created_at: None,
                    };

                    // Asynchronously insert each `post_gallery` entry
                    post_svc.insert_post_gallery(post_gallery)
                })
                .collect::<FuturesUnordered<_>>() // Collect to handle concurrent futures
                .for_each(|result| async move {
                    match result {
                        Ok(_) => info!("Insert successful"), // Log success
                        Err(e) => error!("Error inserting post gallery: {:?}", e),
                    }
                })
                .await;
        }
        _ => {
            println!("Unknown type");
        }
    }

    // Return a success response with the names of the uploaded files
    Ok(Json(json!({
        "status": "success",
        "data":{
            "uploaded_files": uploaded_files,
        }

    })))
}
