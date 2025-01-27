use database::{
    db::DatabaseClient,
    model::User,
};
use errors::Result;


pub mod user;

type DBClient = DatabaseClient;
type RepositoryResult<T> = Result<T>;


type UserModel = User;