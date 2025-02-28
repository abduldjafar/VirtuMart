use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Store {
    #[validate(length(min = 5))]
    pub name: String,
    #[validate(length(min = 5))]
    pub description: Option<String>,
    #[validate(length(min = 5))]
    pub address: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[validate(length(min = 10))]
    pub phone_number: Option<String>,
}
