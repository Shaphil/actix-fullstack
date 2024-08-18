use serde::Serialize;

#[derive(Serialize)]
pub struct HomeResponse {
    pub message: String,
}