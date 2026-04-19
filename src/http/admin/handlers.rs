use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::{
    application::admin_users::{
        ChangePasswordInput, CreateAdminInput, ListAdminUsersInput, UpdateAdminInput,
        authenticate_admin, change_own_password, create_admin,
        delete_admin_user as delete_admin_user_service,
        list_admin_users as list_admin_users_service, touch_last_login,
        update_admin_user as update_admin_user_service,
    },
    error::AppError,
    http::admin::{
        auth::AdminSession,
        dto::{
            AdminLoginRequest, AdminLoginResponse, AdminUsersListResponse,
            ChangeMyPasswordRequest, CreateAdminUserRequest, ListAdminUsersQuery,
            PaginationResponse, SuccessResponse, UpdateAdminUserRequest,
        },
    },
    infrastructure::auth,
    state::AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AdminLoginRequest>,
) -> Result<Json<AdminLoginResponse>, AppError> {
    let admin = authenticate_admin(&state.db, &payload.email, &payload.password)
        .await
        .map_err(map_login_error)?;
    let admin = touch_last_login(&state.db, admin.id)
        .await
        .map_err(map_internal_error)?;

    let claims = auth::issue_admin_jwt(&state.auth, &admin).map_err(map_internal_error)?;
    let access_token =
        auth::encode_admin_jwt(&state.auth, &claims).map_err(map_internal_error)?;

    Ok(Json(AdminLoginResponse {
        access_token,
        token_type: "Bearer",
        expires_in: state.auth.admin_jwt_expires_days * 24 * 60 * 60,
        user: admin.into(),
    }))
}

pub async fn list_admin_users(
    State(state): State<AppState>,
    _session: AdminSession,
    Query(query): Query<ListAdminUsersQuery>,
) -> Result<Json<AdminUsersListResponse>, AppError> {
    let result = list_admin_users_service(
        &state.db,
        ListAdminUsersInput {
            keyword: query.keyword,
            page: query.page.unwrap_or(1),
            page_size: query.page_size.unwrap_or(20),
        },
    )
    .await
    .map_err(map_internal_error)?;

    Ok(Json(AdminUsersListResponse {
        items: result.items.into_iter().map(Into::into).collect(),
        pagination: PaginationResponse {
            page: result.page,
            page_size: result.page_size,
            total: result.total,
        },
    }))
}

pub async fn create_admin_user(
    State(state): State<AppState>,
    _session: AdminSession,
    Json(payload): Json<CreateAdminUserRequest>,
) -> Result<Json<crate::http::admin::dto::AdminUserResponse>, AppError> {
    let admin = create_admin(
        &state.db,
        CreateAdminInput {
            email: payload.email,
            nickname: payload.nickname,
            password: payload.password,
            is_active: payload.is_active.unwrap_or(true),
        },
    )
    .await
    .map_err(map_create_error)?;

    Ok(Json(admin.into()))
}

pub async fn update_admin_user(
    State(state): State<AppState>,
    session: AdminSession,
    Path(admin_user_id): Path<Uuid>,
    Json(payload): Json<UpdateAdminUserRequest>,
) -> Result<Json<crate::http::admin::dto::AdminUserResponse>, AppError> {
    let admin = update_admin_user_service(
        &state.db,
        session.admin_id,
        admin_user_id,
        UpdateAdminInput {
            nickname: payload.nickname,
            is_active: payload.is_active,
        },
    )
    .await
    .map_err(map_update_error)?;

    Ok(Json(admin.into()))
}

pub async fn delete_admin_user(
    State(state): State<AppState>,
    session: AdminSession,
    Path(admin_user_id): Path<Uuid>,
) -> Result<Json<SuccessResponse>, AppError> {
    delete_admin_user_service(&state.db, session.admin_id, admin_user_id)
        .await
        .map_err(map_delete_error)?;

    Ok(Json(SuccessResponse { success: true }))
}

pub async fn change_my_password(
    State(state): State<AppState>,
    session: AdminSession,
    Json(payload): Json<ChangeMyPasswordRequest>,
) -> Result<Json<SuccessResponse>, AppError> {
    change_own_password(
        &state.db,
        session.admin_id,
        ChangePasswordInput {
            current_password: payload.current_password,
            new_password: payload.new_password,
        },
    )
    .await
    .map_err(map_password_error)?;

    Ok(Json(SuccessResponse { success: true }))
}

fn map_login_error(error: anyhow::Error) -> AppError {
    let message = error.to_string();
    if message == "invalid email or password" {
        return AppError::Unauthorized(message);
    }
    if message == "admin account is disabled" {
        return AppError::Forbidden(message);
    }
    if message.contains("email is required") || message.contains("password must be") {
        return AppError::Validation(message);
    }
    AppError::Internal
}

fn map_create_error(error: anyhow::Error) -> AppError {
    let message = error.to_string();
    if message.starts_with("admin email already exists") {
        return AppError::Conflict(message);
    }
    if message.contains("email") || message.contains("nickname") || message.contains("password") {
        return AppError::Validation(message);
    }
    AppError::Internal
}

fn map_update_error(error: anyhow::Error) -> AppError {
    let message = error.to_string();
    if message == "admin user not found" {
        return AppError::NotFound;
    }
    if message.contains("cannot disable") || message.contains("nickname") {
        return AppError::Validation(message);
    }
    AppError::Internal
}

fn map_delete_error(error: anyhow::Error) -> AppError {
    let message = error.to_string();
    if message == "admin user not found" {
        return AppError::NotFound;
    }
    if message.contains("cannot delete") {
        return AppError::Validation(message);
    }
    AppError::Internal
}

fn map_password_error(error: anyhow::Error) -> AppError {
    let message = error.to_string();
    if message == "admin user not found" {
        return AppError::NotFound;
    }
    if message.contains("current password is incorrect") {
        return AppError::Unauthorized(message);
    }
    if message.contains("password") {
        return AppError::Validation(message);
    }
    AppError::Internal
}

fn map_internal_error(_error: anyhow::Error) -> AppError {
    AppError::Internal
}
