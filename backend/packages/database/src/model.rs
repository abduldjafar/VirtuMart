use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

/* Struct representing a User in the database */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct User {
    pub id: Option<Thing>,
    /// Username of the user
    pub username: String,
    /// Type of the user (e.g., admin, regular user)
    pub user_type: String,
    /// Email of the user
    pub email: String,
    /// Timestamp when the user was created
    pub created_at: Option<Datetime>,
    /// Timestamp when the user was last updated
    pub updated_at: Option<Datetime>,
    /// Password of the user
    pub password: String,
    /// Whether the user is verified
    pub verified: bool,
    /// Verification token for the user
    pub verified_token: Option<String>,
}

/* Struct representing a Gym in the database */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gym {
    /// Primary Key
    pub id: Option<Thing>,
    /// Name of the gym
    pub name: String,
    /// Detailed description of gym facilities and services
    pub description: String,
    /// References location_id for gym's location
    pub location_id: Option<Thing>,
    /// References user_id of the gym owner
    pub owner_id: Thing,
    /// Path or URL to profile picture
    pub profile_picture: String,
    /// Timestamp when the gym was added
    pub created_at: Option<Datetime>, // Timestamp when the gym was added
    /// Timestamp of the last update
    pub updated_at: Option<Datetime>, // Timestamp of the last update
}

/* Struct representing an ID in the database */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Id {
    /// Primary Key
    pub id: Thing,
}

/* Struct representing a User in the database */
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PayloadUser {
    /// Username of the user
    pub username: String,
    /// Email of the user
    pub email: String,
    /// Password of the user
    pub password: String,
}

/* Struct representing a User verification payload in the database */
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PayloadUserVerify {
    /// Email of the user to be verified
    pub email: String,
}

/* Struct for deserialization of records */
#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    /// Primary Key for the record
    id: Thing,
}

/* Struct representing responses containing ID */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadIdResponses {
    /// Response containing the ID as a string
    pub id: String,
}

/* Struct representing a Gym response in the database */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadGymResponses {
    /// Primary Key
    pub id: String,
    /// Name of the gym
    pub name: String,
    /// Detailed description of gym facilities and services
    pub description: String,
    /// References location_id for gym's location
    pub location_id: Option<Thing>,
    /// Path or URL to profile picture of the gym
    pub profile_picture: String,
    /// References user_id of the gym owner
    pub owner_id: Option<Thing>,
    /// Timestamp when the gym was added
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update
    pub updated_at: Option<Datetime>,
}

/* Struct representing responses for gym profile */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadGymProfileResponses {
    /// Name of the gym
    pub name: String,
    /// Detailed description of gym facilities and services
    pub description: String,
    /// Path or URL to the profile picture of the gym
    pub profile_picture: String,
    /// Timestamp when the gym was added
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update
    pub updated_at: Option<Datetime>,
}

/* Struct representing requests for gym information */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadGymRequest {
    /// Name of the gym (optional)
    pub name: Option<String>,
    /// Detailed description of gym facilities and services (optional)
    pub description: Option<String>,
    /// Path or URL to the profile picture of the gym (optional)
    pub profile_picture: Option<String>,
    /// References user_id of the gym owner (optional)
    pub owner_id: Option<Thing>,
    /// References location_id for gym's location (optional)
    pub location_id: Option<Thing>,
    /// Timestamp when the gym was added (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
}

/* Struct representing user responses in the database */
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PayloadUserResponse {
    /// Primary Key of the user
    pub id: Thing,
    /// Username of the user
    pub username: String,
    /// Type of the user (e.g., admin, regular user)
    pub user_type: String,
    /// Email of the user
    pub email: String,
    /// Password of the user
    pub password: String,
    /// Whether the user is verified
    pub verified: bool,
    /// Verification token for the user (optional)
    pub verified_token: Option<String>,
    /// Timestamp when the user was created
    pub created_at: Option<Datetime>,
    /// Timestamp when the user was last updated
    pub updated_at: Option<Datetime>,
}

/* Struct representing a Gym Seeker in the database */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GymSeeker {
    /// Primary Key of the gym seeker
    pub id: Option<Thing>,
    /// Name of the gym
    pub name: String,
    /// Date of birth of the gym seeker (optional)
    pub birth_date: Option<Datetime>,
    /// Gender of the gym seeker
    pub sex: String,
    /// References user_id of the gym seeker (optional)
    pub user_id: Option<Thing>,
    /// Path or URL to the profile picture of the gym seeker
    pub profile_picture: String,
    /// Description of fitness goals
    pub fitness_goals: String,
    /// Preferred times for workouts
    pub preferred_workout_time: String,
    /// Preferences for gyms
    pub gym_preferences: String,
    /// Current membership status
    pub membership_status: String,
    /// Short biography of the gym seeker
    pub bio: String,
    /// Timestamp when the seeker registered (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
}

/* Struct representing requests for gym seeker information */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadGymSeekerRequest {
    /// Name of the gym
    pub name: Option<String>,
    /// Date of birth of the gym seeker (optional)
    pub birth_date: Option<Datetime>,
    /// Gender of the gym seeker (optional)
    pub sex: Option<String>,
    /// References user_id of the gym seeker (optional)
    pub user_id: Option<Thing>,
    /// Path or URL to the profile picture of the gym seeker (optional)
    pub profile_picture: Option<String>,
    /// Description of fitness goals (optional)
    pub fitness_goals: Option<String>,
    /// Preferred times for workouts (optional)
    pub preferred_workout_time: Option<String>,
    /// Preferences for gyms (optional)
    pub gym_preferences: Option<String>,
    /// Current membership status (optional)
    pub membership_status: Option<String>,
    /// Short biography of the gym seeker (optional)
    pub bio: Option<String>,
    /// Timestamp when the seeker registered (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
}

/* Struct representing responses for gym seeker information */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadGymSeekerResponse {
    /// Primary Key of the gym seeker
    pub id: String,
    /// Name of the gym
    pub name: String,
    /// Date of birth of the gym seeker (optional)
    pub birth_date: Option<Datetime>,
    /// Gender of the gym seeker
    pub sex: String,
    /// References user_id of the gym seeker (optional)
    pub user_id: Option<Thing>,
    /// Path or URL to the profile picture of the gym seeker
    pub profile_picture: String,
    /// Description of fitness goals
    pub fitness_goals: String,
    /// Preferred times for workouts
    pub preferred_workout_time: String,
    /// Preferences for gyms
    pub gym_preferences: String,
    /// Current membership status
    pub membership_status: String,
    /// Short biography of the gym seeker
    pub bio: String,
    /// Timestamp when the seeker registered (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
}

/* Struct representing a gym seeker profile response */
#[derive(Debug, Serialize, Deserialize)]
pub struct PayloadGymSeekerProfileResponse {
    /// Name of the gym
    pub name: String,
    /// Date of birth of the gym seeker (optional)
    pub birth_date: Option<Datetime>,
    /// Gender of the gym seeker
    pub sex: String,
    /// Path or URL to the profile picture of the gym seeker
    pub profile_picture: String,
    /// Description of fitness goals
    pub fitness_goals: String,
    /// Preferred times for workouts
    pub preferred_workout_time: String,
    /// Preferences for gyms
    pub gym_preferences: String,
    /// Current membership status
    pub membership_status: String,
    /// Short biography of the gym seeker
    pub bio: String,
    /// Timestamp when the seeker registered (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
}

/* Struct representing login user credentials */
#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    /// Email of the user
    pub email: String,
    /// Password of the user
    pub password: String,
}

/* Struct representing a Location in the database */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Location {
    /// Primary Key of the location (optional)
    pub id: Option<Thing>,
    /// Full address of the location
    pub address: String,
    /// Latitude coordinate of the location
    pub latitude: f64,
    /// Longitude coordinate of the location
    pub longitude: f64,
    /// Timestamp when the location was added (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
}

/* Struct representing requests for location information */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadLocationRequest {
    /// Full address of the location
    pub address: String,
    /// Latitude coordinate of the location
    pub latitude: f64,
    /// Longitude coordinate of the location
    pub longitude: f64,
    /// Timestamp when the location was added (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
}

/* Struct representing responses for location information */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadLocationResponse {
    /// Primary Key of the location (optional)
    pub id: Option<Thing>,
    /// Full address of the location (optional)
    pub address: Option<String>,
    /// Latitude coordinate of the location (optional)
    pub latitude: Option<String>,
    /// Longitude coordinate of the location (optional)
    pub longitude: Option<String>,
    /// Timestamp when the location was added (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
}

/* Struct representing a Trainer */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trainer {
    /// Primary Key - Unique identifier for the trainer (optional)
    pub id: Option<Thing>,
    /// Name of the trainer
    pub name: Option<String>,
    /// Gender of the gym seeker
    pub sex: Option<String>,
    /// year of the trainer exeperience
    pub experience: Option<i64>,
    /// Field of specialization (e.g., yoga, strength training) (optional)
    pub expertise: Option<String>,
    /// Foreign Key - References the gym where the trainer is based
    pub gym_id: Thing,
    /// Foreign Key - References the user_id of the trainer
    pub user_id: Thing,
    /// Timestamp when the trainer was added (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadTrainerRequest {
    /// Name of the trainer
    pub name: Option<String>,
    /// Gender of the gym seeker
    pub sex: Option<String>,
    /// year of the trainer exeperience
    pub experience: Option<i64>,
    /// Field of specialization (e.g., yoga, strength training) (optional)
    pub expertise: Option<String>,
    /// Foreign Key - References the gym where the trainer is based
    pub gym_id: Option<Thing>,
    /// Foreign Key - References the user_id of the trainer
    pub user_id: Option<Thing>,
    /// Timestamp when the trainer was added (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadTrainerResponse {
    /// Primary Key of the location (optional)
    pub id: String,
    /// Name of the trainer
    pub name: Option<String>,
    /// Gender of the gym seeker
    pub sex: Option<String>,
    /// year of the trainer exeperience
    pub experience: Option<i64>,
    /// Field of specialization (e.g., yoga, strength training) (optional)
    pub expertise: Option<String>,
    /// Foreign Key - References the gym where the trainer is based
    /// Timestamp when the trainer was added (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadTrainerProfileResponse {
    pub name: Option<String>,
    /// Gender of the gym seeker
    pub sex: Option<String>,
    /// year of the trainer exeperience
    pub experience: Option<i64>,
    /// Field of specialization (e.g., yoga, strength training) (optional)
    pub expertise: Option<String>,
    /// Foreign Key - References the gym where the trainer is based
    /// Timestamp when the trainer was added (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Post {
    /// Primary Key
    pub id: Option<Thing>,
    /// References the user creating the post
    pub user_id: Option<Thing>,
    /// Foreign Key - Related gym ID if post is gym-specific (optional)
    pub gym_id: Option<Thing>,
    /// Foreign Key - ID if post relates to a gym seeker (optional)
    pub gym_seeker_id: Option<Thing>,
    /// Foreign Key - ID if post relates to a trainer (optional)
    pub trainer_id: Option<Thing>,
    /// Flag indicating if the post is from a gym (optional)
    pub is_gym: Option<bool>,
    /// Flag indicating if the post is from a gym seeker (optional)
    pub is_gym_seeker: Option<bool>,
    /// Flag indicating if the post is from a trainer (optional)
    pub is_trainer: Option<bool>,
    /// Content of the post (text, images, etc.)
    pub content: Option<String>,
    /// Timestamp when the post was created (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadPostRequest {
    pub content: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadPostResponse {
    /// Primary Key
    pub id: Option<Thing>,
    /// References the user creating the post
    pub user_id: Option<Thing>,
    /// Foreign Key - Related gym ID if post is gym-specific (optional)
    pub gym_id: Option<Thing>,
    /// Foreign Key - ID if post relates to a gym seeker (optional)
    pub gym_seeker_id: Option<Thing>,
    /// Foreign Key - ID if post relates to a trainer (optional)
    pub trainer_id: Option<Thing>,
    /// Flag indicating if the post is from a gym (optional)
    pub is_gym: Option<bool>,
    /// Flag indicating if the post is from a gym seeker (optional)
    pub is_gym_seeker: Option<bool>,
    /// Flag indicating if the post is from a trainer (optional)
    pub is_trainer: Option<bool>,
    /// Content of the post (text, images, etc.)
    pub content: Option<String>,
    /// Timestamp when the post was created (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
    pub post_gallery: Option<Vec<PostGalleryForFeed>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadPostResponseDeserilize {
    /// Primary Key
    pub id: Option<String>,
    /// References the user creating the post
    pub user_id: Option<String>,
    /// Foreign Key - Related gym ID if post is gym-specific (optional)
    pub gym_id: Option<String>,
    /// Foreign Key - ID if post relates to a gym seeker (optional)
    pub gym_seeker_id: Option<String>,
    /// Foreign Key - ID if post relates to a trainer (optional)
    pub trainer_id: Option<String>,
    /// Flag indicating if the post is from a gym (optional)
    pub is_gym: Option<bool>,
    /// Flag indicating if the post is from a gym seeker (optional)
    pub is_gym_seeker: Option<bool>,
    /// Flag indicating if the post is from a trainer (optional)
    pub is_trainer: Option<bool>,
    /// Content of the post (text, images, etc.)
    pub content: Option<String>,
    /// Timestamp when the post was created (optional)
    pub created_at: Option<Datetime>,
    /// Timestamp of the last update (optional)
    pub updated_at: Option<Datetime>,
    pub post_gallery: Option<Vec<PostGalleryForFeedDeserialize>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PayloadPostUpdateRequest {
    pub content: String,
    pub content_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedLocation {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    pub content: String,
    pub gym_name: Option<String>, // Use Option because it can be NONE
    pub location: Option<FeedLocation>, // Use Option because it can be NULL
    pub profile_picture: Option<String>,
    pub post_gallery: Option<Vec<PostGalleryForFeed>>, // Use Option because it can be NONE
    pub updated_at: Option<Datetime>, // Assuming you're using the chrono crate for date/time
    pub user_type: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadFeed {
    pub content: String,
    pub gym_name: Option<String>, // Use Option because it can be NONE
    pub location: Option<FeedLocation>, // Use Option because it can be NULL
    pub profile_picture: Option<String>, // Use Option because it can be NONE
    pub updated_at: Option<Datetime>, // Assuming you're using the chrono crate for date/time
    pub post_gallery: Option<Vec<PostGalleryForFeedDeserialize>>,
    pub user_type: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentGalery {
    pub content_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostGallery {
    /// Unique identifier for the gallery post
    pub id: Option<Thing>,
    /// Identifier linking this gallery post to the main feed content
    pub content_id: Option<Thing>,
    /// URL or path to the image associated with the post
    pub media_url: Option<String>,
    /// Timestamp indicating when the post was created
    pub created_at: Option<Datetime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostGalleryForFeed {
    pub id: Option<Thing>,
    pub link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostGalleryForFeedDeserialize {
    pub id: String,
    pub link: String,
}
