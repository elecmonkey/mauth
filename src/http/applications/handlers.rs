use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    error::AppError,
    http::applications::dto::{ApplicationResponse, CreateApplicationRequest},
    state::AppState,
};

pub async fn list_applications(
    State(_state): State<AppState>,
) -> Result<Json<ApplicationResponse>, AppError> {
    Ok(Json(ApplicationResponse {
        module: "applications",
        action: "list",
        status: "not_implemented",
    }))
}

pub async fn create_application(
    State(_state): State<AppState>,
    Json(payload): Json<CreateApplicationRequest>,
) -> Result<Json<ApplicationResponse>, AppError> {
    if payload.name.trim().is_empty() {
        return Err(AppError::Validation("application name is required".into()));
    }

    Ok(Json(ApplicationResponse {
        module: "applications",
        action: "create",
        status: "not_implemented",
    }))
}

pub async fn get_application(
    State(_state): State<AppState>,
    Path(_application_id): Path<String>,
) -> Result<Json<ApplicationResponse>, AppError> {
    Ok(Json(ApplicationResponse {
        module: "applications",
        action: "detail",
        status: "not_implemented",
    }))
}
