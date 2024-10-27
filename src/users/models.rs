use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRequest {
    pub username: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub is_active: Option<bool>,
    pub last_login: Option<NaiveDateTime>,
    pub date_joined: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_admin: Option<bool>,
    pub is_superadmin: Option<bool>,
}