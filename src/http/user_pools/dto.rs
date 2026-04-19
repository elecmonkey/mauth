use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateUserPoolRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct UserPoolResponse {
    pub module: &'static str,
    pub action: &'static str,
    pub status: &'static str,
}
