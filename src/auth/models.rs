use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct RefreshToken {
    pub token: String,
}