use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateApplicationRequest {
    pub name: String,
    #[allow(dead_code)]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApplicationResponse {
    pub module: &'static str,
    pub action: &'static str,
    pub status: &'static str,
}
