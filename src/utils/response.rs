use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
}