use axum::{Json, extract::State};

use crate::{
    error::AppError,
    http::auth::dto::{AuthActionResponse, LoginRequest},
    state::AppState,
};

pub async fn login(
    State(_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthActionResponse>, AppError> {
    if payload.username.trim().is_empty() || payload.password.trim().is_empty() {
        return Err(AppError::Validation(
            "username and password are required".into(),
        ));
    }

    Ok(Json(AuthActionResponse {
        module: "auth",
        action: "login",
        status: "not_implemented",
    }))
}

pub async fn authorize(
    State(_state): State<AppState>,
) -> Result<Json<AuthActionResponse>, AppError> {
    Ok(Json(AuthActionResponse {
        module: "auth",
        action: "authorize",
        status: "not_implemented",
    }))
}

pub async fn issue_token(
    State(_state): State<AppState>,
) -> Result<Json<AuthActionResponse>, AppError> {
    Ok(Json(AuthActionResponse {
        module: "auth",
        action: "token",
        status: "not_implemented",
    }))
}
