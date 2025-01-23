use database::{
    db::DatabaseClient,
    model::{Id, User},
};
use errors::Result;


pub mod user;

type DBClient = DatabaseClient;
type RepositoryResult<T> = Result<T>;


type UserModel = User;
type UserId = Id;