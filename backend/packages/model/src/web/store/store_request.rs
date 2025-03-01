use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

static RE_USER_PREFIX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^user_[a-z]{2}$").unwrap());

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Store {
    #[validate(regex(path = *RE_USER_PREFIX))]
    pub user_id: String,
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
