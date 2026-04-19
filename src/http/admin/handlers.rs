use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::{
    application::admin_auth::{
        BeginAdminLoginOutput, begin_admin_login, complete_admin_totp_login,
        confirm_admin_totp, disable_admin_totp, get_admin_totp_status, setup_admin_totp,
    },
    application::admin_users::{
        ChangePasswordInput, CreateAdminInput, ListAdminUsersInput, UpdateAdminInput,
        change_own_password, create_admin, delete_admin_user as delete_admin_user_service,
        list_admin_users as list_admin_users_service,
        update_admin_user as update_admin_user_service,
    },
    error::AppError,
    http::admin::{
        auth::AdminSession,
        dto::{
            AdminLoginRequest, AdminLoginResponse, AdminLoginTotpRequest, AdminUsersListResponse,
            ChangeMyPasswordRequest, ConfirmTotpRequest, CreateAdminUserRequest,
            DisableTotpRequest, ListAdminUsersQuery, PaginationResponse, SuccessResponse,
            TotpSetupResponse, TotpStatusResponse, TotpUpdateResponse, UpdateAdminUserRequest,
        },
    },
    state::AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AdminLoginRequest>,
) -> Result<Json<AdminLoginResponse>, AppError> {
    let result = begin_admin_login(&state.db, &state.auth, &payload.email, &payload.password)
        .await
        .map_err(map_login_error)?;

    match result {
        BeginAdminLoginOutput::Authenticated(result) => Ok(Json(AdminLoginResponse {
            status: "authenticated",
            requires_totp: false,
            access_token: Some(result.access_token),
            token_type: Some("Bearer"),
            expires_in: Some(result.expires_in),
            challenge_token: None,
            challenge_expires_in: None,
            user: result.admin.into(),
        })),
        BeginAdminLoginOutput::PendingTotp(result) => Ok(Json(AdminLoginResponse {
            status: "pending_totp",
            requires_totp: true,
            access_token: None,
            token_type: None,
            expires_in: None,
            challenge_token: Some(result.challenge_token),
            challenge_expires_in: Some(result.challenge_expires_in),
            user: result.admin.into(),
        })),
    }
}

pub async fn login_totp(
    State(state): State<AppState>,
    Json(payload): Json<AdminLoginTotpRequest>,
) -> Result<Json<AdminLoginResponse>, AppError> {
    let result = complete_admin_totp_login(
        &state.db,
        &state.auth,
        &payload.challenge_token,
        &payload.totp_code,
    )
    .await
    .map_err(map_login_error)?;

    Ok(Json(AdminLoginResponse {
        status: "authenticated",
        requires_totp: false,
        access_token: Some(result.access_token),
        token_type: Some("Bearer"),
        expires_in: Some(result.expires_in),
        challenge_token: None,
        challenge_expires_in: None,
        user: result.admin.into(),
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

pub async fn get_my_totp_status(
    State(state): State<AppState>,
    session: AdminSession,
) -> Result<Json<TotpStatusResponse>, AppError> {
    let result = get_admin_totp_status(&state.db, session.admin_id)
        .await
        .map_err(map_internal_error)?;

    Ok(Json(TotpStatusResponse {
        enabled: result.enabled,
    }))
}

pub async fn setup_my_totp(
    State(state): State<AppState>,
    session: AdminSession,
) -> Result<Json<TotpSetupResponse>, AppError> {
    let result = setup_admin_totp(&state.db, &state.auth, session.admin_id)
        .await
        .map_err(map_totp_setup_error)?;

    Ok(Json(TotpSetupResponse {
        secret: result.secret,
        otpauth_uri: result.otpauth_uri,
        qr_svg: result.qr_svg,
    }))
}

pub async fn confirm_my_totp(
    State(state): State<AppState>,
    session: AdminSession,
    Json(payload): Json<ConfirmTotpRequest>,
) -> Result<Json<TotpUpdateResponse>, AppError> {
    let admin = confirm_admin_totp(&state.db, &state.auth, session.admin_id, &payload.totp_code)
        .await
        .map_err(map_totp_confirm_error)?;

    Ok(Json(TotpUpdateResponse {
        success: true,
        user: admin.into(),
    }))
}

pub async fn disable_my_totp(
    State(state): State<AppState>,
    session: AdminSession,
    Json(payload): Json<DisableTotpRequest>,
) -> Result<Json<TotpUpdateResponse>, AppError> {
    let admin = disable_admin_totp(
        &state.db,
        &state.auth,
        session.admin_id,
        &payload.current_password,
        &payload.totp_code,
    )
    .await
    .map_err(map_totp_disable_error)?;

    Ok(Json(TotpUpdateResponse {
        success: true,
        user: admin.into(),
    }))
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
    if message.contains("totp code must be") {
        return AppError::Validation(message);
    }
    if message.contains("totp code is invalid") {
        return AppError::Unauthorized(message);
    }
    if message.contains("login challenge") {
        return AppError::Unauthorized(message);
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

fn map_totp_setup_error(error: anyhow::Error) -> AppError {
    let message = error.to_string();
    if message.contains("already enabled") {
        return AppError::Validation(message);
    }
    if message == "admin user not found" {
        return AppError::NotFound;
    }
    AppError::Internal
}

fn map_totp_confirm_error(error: anyhow::Error) -> AppError {
    let message = error.to_string();
    if message == "admin user not found" {
        return AppError::NotFound;
    }
    if message.contains("totp setup has not been initialized")
        || message.contains("totp code must be")
    {
        return AppError::Validation(message);
    }
    if message.contains("totp code is invalid") {
        return AppError::Unauthorized(message);
    }
    AppError::Internal
}

fn map_totp_disable_error(error: anyhow::Error) -> AppError {
    let message = error.to_string();
    if message == "admin user not found" {
        return AppError::NotFound;
    }
    if message.contains("current password is incorrect") || message.contains("totp code is invalid")
    {
        return AppError::Unauthorized(message);
    }
    if message.contains("totp is not enabled") || message.contains("totp code must be") {
        return AppError::Validation(message);
    }
    AppError::Internal
}

fn map_internal_error(_error: anyhow::Error) -> AppError {
    AppError::Internal
}
