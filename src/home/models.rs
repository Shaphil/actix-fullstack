use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HomeResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub is_active: Option<bool>,
    pub last_login: Option<DateTime>,
    pub date_joined: Option<DateTime>,
    pub password: String,
}