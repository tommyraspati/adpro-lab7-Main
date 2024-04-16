use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::to_string;
use rocket::log;
use rocket::tokio;
use bambangshop::REQWEST_CLIENT;
use crate::model::notification::Notification;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub url : String,
    pub name: String,
}