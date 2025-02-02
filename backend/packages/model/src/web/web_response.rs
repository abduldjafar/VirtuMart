use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WebResponse<T> {
    pub code: i32,
    pub status: String,
    pub data: T,
}
