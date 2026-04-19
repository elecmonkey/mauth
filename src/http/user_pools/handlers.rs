use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    error::AppError,
    http::user_pools::dto::{CreateUserPoolRequest, UserPoolResponse},
    state::AppState,
};

pub async fn list_user_pools(
    State(_state): State<AppState>,
) -> Result<Json<UserPoolResponse>, AppError> {
    Ok(Json(UserPoolResponse {
        module: "user_pools",
        action: "list",
        status: "not_implemented",
    }))
}

pub async fn create_user_pool(
    State(_state): State<AppState>,
    Json(payload): Json<CreateUserPoolRequest>,
) -> Result<Json<UserPoolResponse>, AppError> {
    if payload.name.trim().is_empty() {
        return Err(AppError::Validation("user pool name is required".into()));
    }

    Ok(Json(UserPoolResponse {
        module: "user_pools",
        action: "create",
        status: "not_implemented",
    }))
}

pub async fn get_user_pool(
    State(_state): State<AppState>,
    Path(_user_pool_id): Path<String>,
) -> Result<Json<UserPoolResponse>, AppError> {
    Ok(Json(UserPoolResponse {
        module: "user_pools",
        action: "detail",
        status: "not_implemented",
    }))
}
