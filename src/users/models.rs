use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRequest {
    pub username: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: String,
    pub password: String,
    pub is_active: Option<bool>,
    pub last_login: Option<NaiveDateTime>,
    pub date_joined: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_admin: Option<bool>,
    pub is_superadmin: Option<bool>,
}