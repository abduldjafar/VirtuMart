use database::{
    db::DatabaseClient,
    model::{
        Feed, Gym, GymSeeker, Id, Location, PayloadGymRequest, PayloadGymSeekerRequest,
        PayloadLocationRequest, PayloadPostResponse, PayloadTrainerRequest, Post, Trainer, User,
    },
};
use errors::Result;


pub mod user;

type DBClient = DatabaseClient;
type RepositoryResult<T> = Result<T>;


type UserModel = User;
type UserId = Id;