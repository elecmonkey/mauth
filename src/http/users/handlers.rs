use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    error::AppError,
    http::users::dto::{CreateUserRequest, UserResponse},
    state::AppState,
};

pub async fn list_users(State(_state): State<AppState>) -> Result<Json<UserResponse>, AppError> {
    Ok(Json(UserResponse {
        module: "users",
        action: "list",
        status: "not_implemented",
    }))
}

pub async fn create_user(
    State(_state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    if payload.username.trim().is_empty() {
        return Err(AppError::Validation("username is required".into()));
    }

    Ok(Json(UserResponse {
        module: "users",
        action: "create",
        status: "not_implemented",
    }))
}

pub async fn get_user(
    State(_state): State<AppState>,
    Path(_user_id): Path<String>,
) -> Result<Json<UserResponse>, AppError> {
    Ok(Json(UserResponse {
        module: "users",
        action: "detail",
        status: "not_implemented",
    }))
}
