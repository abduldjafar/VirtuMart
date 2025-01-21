use environment::Environment;
use file_storage::interface::FileStorage;
use redis::Client;
use services::{
    auth::AuthServices, email::EmailServices, feed::FeedServices, gym::GymServices,
    gymseeker::GymSeekerServices, location::LocationServices, post::PostServices,
    trainer::TrainerServices,
};

#[derive(Clone)]
pub struct AppState {
    pub gym_services: GymServices,
    pub gymseeker_services: GymSeekerServices,
    pub auth_services: AuthServices,
    pub location_services: LocationServices,
    pub trainer_services: TrainerServices,
    pub post_services: PostServices,
    pub feed_services: FeedServices,
    pub email_services: EmailServices,
    pub redis_client: Client,
    pub environment: Environment,
    pub cloud_storage: FileStorage,
}
