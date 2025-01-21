use database::{
    db::DatabaseClient,
    model::{
        Feed, Gym, GymSeeker, Id, Location, PayloadGymRequest, PayloadGymSeekerRequest,
        PayloadLocationRequest, PayloadPostResponse, PayloadTrainerRequest, Post, Trainer, User,
    },
};
use errors::Result;

pub mod feed;
pub mod gym;
pub mod gymseeker;
pub mod location;
pub mod post;
pub mod trainer;
pub mod user;

type DBClient = DatabaseClient;
type RepositoryResult<T> = Result<T>;

type GymModel = Gym;
type GymId = Id;
type RepositoryGymRequest = PayloadGymRequest;

type GymSeekerModel = GymSeeker;
type GymSeekerId = Id;
type RepositoryGymSeekerRequest = PayloadGymSeekerRequest;

type UserModel = User;
type UserId = Id;

type TrainerModel = Trainer;
type TrainerId = Id;
type RepositoryTrainerRequests = PayloadTrainerRequest;

type PostModel = Post;
type PostId = Id;
type RepositoryPostResponse = PayloadPostResponse;
//type RepositoryPostRequests = PayloadPostRequest;

type LocationModel = Location;
type RepositoryLocationRequest = PayloadLocationRequest;
type LocationId = Id;

type FeedModel = Feed;
