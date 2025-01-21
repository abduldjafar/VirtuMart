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

}
