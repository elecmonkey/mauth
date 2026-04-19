pub mod applications;
pub mod auth;
pub mod health;
pub mod middleware;
pub mod user_pools;
pub mod users;

use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;

use crate::{error::AppError, state::AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/health", health::routes())
        .nest("/auth", auth::routes())
        .nest("/applications", applications::routes())
        .nest("/user-pools", user_pools::routes())
        .nest("/users", users::routes())
        .route("/", get(root))
}

async fn root() -> impl IntoResponse {
    Json(json!({
        "service": "mauth",
        "status": "ok"
    }))
}

pub async fn not_found() -> Result<impl IntoResponse, AppError> {
    Ok((
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": "route not found"
        })),
    ))
}
