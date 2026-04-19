use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthActionResponse {
    pub module: &'static str,
    pub action: &'static str,
    pub status: &'static str,
}
