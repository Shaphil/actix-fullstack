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
    pub last_login: Option<String>,
    pub date_joined: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}