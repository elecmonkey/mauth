use axum::{
    Json,
    extract::{Path, Query, State},
};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    application::admin_auth::{
        BeginAdminLoginOutput, begin_admin_login, complete_admin_totp_login, confirm_admin_totp,
        disable_admin_totp, get_admin_totp_status, setup_admin_totp,
    },
    application::admin_users::{
        ChangePasswordInput, CreateAdminInput, ListAdminUsersInput, UpdateAdminInput,
        change_own_password, create_admin, delete_admin_user as delete_admin_user_service,
        list_admin_users as list_admin_users_service,
        update_admin_user as update_admin_user_service,
    },
    application::applications::{
        CreateApplicationInput, CreateApplicationResult, CreateRedirectUriInput,
        ListApplicationsInput, UpdateApplicationInput, UpdateRedirectUriInput,
        create_application as create_application_service,
        create_redirect_uri as create_redirect_uri_service,
        delete_application as delete_application_service,
        delete_redirect_uri as delete_redirect_uri_service, get_application_detail,
        list_applications as list_applications_service,
        list_redirect_uris as list_redirect_uris_service,
        reset_application_secret as reset_application_secret_service, set_application_active,
        update_application as update_application_service,
        update_redirect_uri as update_redirect_uri_service,
    },
    application::user_pools::{
        CreateProfileFieldInput, CreateUserPoolInput, ListUserPoolsInput, UpdateProfileFieldInput,
        UpdateUserPoolInput, create_profile_field as create_profile_field_service,
        create_user_pool as create_user_pool_service,
        delete_profile_field as delete_profile_field_service,
        delete_user_pool as delete_user_pool_service, get_user_pool_detail,
        list_profile_fields as list_profile_fields_service,
        list_user_pools as list_user_pools_service, set_user_pool_active,
        update_profile_field as update_profile_field_service,
        update_user_pool as update_user_pool_service,
    },
    error::AppError,
    http::admin::{
        auth::AdminSession,
        dto::{
            AdminLoginRequest, AdminLoginResponse, AdminLoginTotpRequest, AdminUsersListResponse,
            ApplicationDetailResponse, ApplicationListResponse, ApplicationSummaryResponse,
            ChangeMyPasswordRequest, ConfirmTotpRequest, CreateAdminUserRequest,
            CreateApplicationRequest, CreateApplicationResponse, CreateProfileFieldRequest,
            CreateRedirectUriRequest, CreateUserPoolRequest, DisableTotpRequest,
            ListAdminUsersQuery, ListApplicationsQuery, ListUserPoolsQuery, PaginationResponse,
            ProfileFieldListResponse, ProfileFieldResponse, RedirectUriListResponse,
            RedirectUriResponse, ResetApplicationSecretRequest, ResetApplicationSecretResponse,
            SuccessResponse, TotpSetupResponse, TotpStatusResponse, TotpUpdateResponse,
            UpdateAdminUserRequest, UpdateApplicationRequest, UpdateProfileFieldRequest,
            UpdateRedirectUriRequest, UpdateUserPoolRequest, UserPoolDetailResponse,
            UserPoolListResponse, UserPoolSummaryResponse,
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

pub async fn list_user_pools(
    State(state): State<AppState>,
    _session: AdminSession,
    Query(query): Query<ListUserPoolsQuery>,
) -> Result<Json<UserPoolListResponse>, AppError> {
    let result = list_user_pools_service(
        &state.db,
        ListUserPoolsInput {
            keyword: query.keyword,
            is_active: query.is_active,
            page: query.page.unwrap_or(1),
            page_size: query.page_size.unwrap_or(20),
        },
    )
    .await
    .map_err(map_management_error)?;

    Ok(Json(UserPoolListResponse {
        items: result
            .items
            .into_iter()
            .map(|item| UserPoolSummaryResponse::from((item.user_pool, item.application_count)))
            .collect(),
        pagination: PaginationResponse {
            page: result.page,
            page_size: result.page_size,
            total: result.total,
        },
    }))
}

pub async fn create_user_pool(
    State(state): State<AppState>,
    _session: AdminSession,
    Json(payload): Json<CreateUserPoolRequest>,
) -> Result<Json<UserPoolSummaryResponse>, AppError> {
    let user_pool = create_user_pool_service(
        &state.db,
        CreateUserPoolInput {
            code: payload.code,
            name: payload.name,
            description: payload.description,
            allow_self_signup: payload.allow_self_signup.unwrap_or(false),
        },
    )
    .await
    .map_err(map_management_error)?;

    Ok(Json(UserPoolSummaryResponse::from((user_pool, 0))))
}

pub async fn get_user_pool(
    State(state): State<AppState>,
    _session: AdminSession,
    Path(user_pool_id): Path<Uuid>,
) -> Result<Json<UserPoolDetailResponse>, AppError> {
    let detail = get_user_pool_detail(&state.db, user_pool_id)
        .await
        .map_err(map_management_error)?;

    Ok(Json(UserPoolDetailResponse {
        id: detail.user_pool.id,
        code: detail.user_pool.code,
        name: detail.user_pool.name,
        description: detail.user_pool.description,
        is_active: detail.user_pool.is_active,
        allow_self_signup: detail.user_pool.allow_self_signup,
        profile_fields: detail.profile_fields.into_iter().map(Into::into).collect(),
        applications: detail
            .applications
            .into_iter()
            .map(
                |application| crate::http::admin::dto::UserPoolApplicationSummaryResponse {
                    id: application.id,
                    code: application.code,
                    name: application.name,
                    is_active: application.is_active,
                },
            )
            .collect(),
        created_at: detail.user_pool.created_at,
        updated_at: detail.user_pool.updated_at,
    }))
}

pub async fn update_user_pool(
    State(state): State<AppState>,
    _session: AdminSession,
    Path(user_pool_id): Path<Uuid>,
    Json(payload): Json<UpdateUserPoolRequest>,
) -> Result<Json<UserPoolSummaryResponse>, AppError> {
    let user_pool = update_user_pool_service(
        &state.db,
        user_pool_id,
        UpdateUserPoolInput {
            name: payload.name,
            description: payload.description,
            allow_self_signup: payload.allow_self_signup,
        },
    )
    .await
    .map_err(map_management_error)?;

    let application_count = crate::entity::application::Entity::find()
        .filter(crate::entity::application::Column::UserPoolId.eq(user_pool.id))
        .count(&state.db)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(UserPoolSummaryResponse::from((
        user_pool,
        application_count,
    ))))
}

pub async fn enable_user_pool(
    State(state): State<AppState>,
    _session: AdminSession,
    Path(user_pool_id): Path<Uuid>,
) -> Result<Json<UserPoolSummaryResponse>, AppError> {
    let user_pool = set_user_pool_active(&state.db, user_pool_id, true)
        .await
        .map_err(map_management_error)?;
    let application_count = crate::entity::application::Entity::find()
        .filter(crate::entity::application::Column::UserPoolId.eq(user_pool.id))
        .count(&state.db)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(UserPoolSummaryResponse::from((
        user_pool,
        application_count,
    ))))
}

pub async fn disable_user_pool(
    State(state): State<AppState>,
    _session: AdminSession,
    Path(user_pool_id): Path<Uuid>,
) -> Result<Json<UserPoolSummaryResponse>, AppError> {
    let user_pool = set_user_pool_active(&state.db, user_pool_id, false)
        .await
        .map_err(map_management_error)?;
    let application_count = crate::entity::application::Entity::find()
        .filter(crate::entity::application::Column::UserPoolId.eq(user_pool.id))
        .count(&state.db)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(UserPoolSummaryResponse::from((
        user_pool,
        application_count,
    ))))
}

pub async fn delete_user_pool(
    State(state): State<AppState>,
    _session: AdminSession,
    Path(user_pool_id): Path<Uuid>,
) -> Result<Json<SuccessResponse>, AppError> {
    delete_user_pool_service(&state.db, user_pool_id)
        .await
        .map_err(map_management_error)?;
    Ok(Json(SuccessResponse { success: true }))
}

pub async fn list_profile_fields(
    State(state): State<AppState>,
    _session: AdminSession,
    Path(user_pool_id): Path<Uuid>,
) -> Result<Json<ProfileFieldListResponse>, AppError> {
    let items = list_profile_fields_service(&state.db, user_pool_id)
        .await
        .map_err(map_management_error)?;
    Ok(Json(ProfileFieldListResponse {
        items: items.into_iter().map(Into::into).collect(),
    }))
}

pub async fn create_profile_field(
    State(state): State<AppState>,
    _session: AdminSession,
    Path(user_pool_id): Path<Uuid>,
    Json(payload): Json<CreateProfileFieldRequest>,
) -> Result<Json<ProfileFieldResponse>, AppError> {
    let field = create_profile_field_service(
        &state.db,
        user_pool_id,
        CreateProfileFieldInput {
            field_key: payload.field_key,
            field_name: payload.field_name,
            field_type: payload.field_type,
            is_required: payload.is_required.unwrap_or(false),
            is_unique: payload.is_unique.unwrap_or(false),
            is_searchable: payload.is_searchable.unwrap_or(false),
            sort_order: payload.sort_order.unwrap_or(0),
            options: payload.options,
        },
    )
    .await
    .map_err(map_management_error)?;
    Ok(Json(field.into()))
}

pub async fn update_profile_field(
    State(state): State<AppState>,
    _session: AdminSession,
    Path((user_pool_id, field_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateProfileFieldRequest>,
) -> Result<Json<ProfileFieldResponse>, AppError> {
    let field = update_profile_field_service(
        &state.db,
        user_pool_id,
        field_id,
        UpdateProfileFieldInput {
            field_name: payload.field_name,
            is_required: payload.is_required,
            is_searchable: payload.is_searchable,
            sort_order: payload.sort_order,
            options: payload.options,
        },
    )
    .await
    .map_err(map_management_error)?;
    Ok(Json(field.into()))
}

pub async fn delete_profile_field(
    State(state): State<AppState>,
    _session: AdminSession,
    Path((user_pool_id, field_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<SuccessResponse>, AppError> {
    delete_profile_field_service(&state.db, user_pool_id, field_id)
        .await
        .map_err(map_management_error)?;
    Ok(Json(SuccessResponse { success: true }))
}

pub async fn list_applications(
    State(state): State<AppState>,
    _session: AdminSession,
    Query(query): Query<ListApplicationsQuery>,
) -> Result<Json<ApplicationListResponse>, AppError> {
    let result = list_applications_service(
        &state.db,
        ListApplicationsInput {
            keyword: query.keyword,
            user_pool_id: query.user_pool_id,
            is_active: query.is_active,
            page: query.page.unwrap_or(1),
            page_size: query.page_size.unwrap_or(20),
        },
    )
    .await
    .map_err(map_management_error)?;

    Ok(Json(ApplicationListResponse {
        items: result
            .items
            .into_iter()
            .map(|item| {
                ApplicationSummaryResponse::from((
                    item.application,
                    item.user_pool,
                    item.redirect_uri_count,
                ))
            })
            .collect(),
        pagination: PaginationResponse {
            page: result.page,
            page_size: result.page_size,
            total: result.total,
        },
    }))
}

pub async fn create_application(
    State(state): State<AppState>,
    _session: AdminSession,
    Json(payload): Json<CreateApplicationRequest>,
) -> Result<Json<CreateApplicationResponse>, AppError> {
    let result = create_application_service(
        &state.db,
        CreateApplicationInput {
            code: payload.code,
            name: payload.name,
            description: payload.description,
            application_type: payload.application_type,
            user_pool_id: payload.user_pool_id,
            homepage_url: payload.homepage_url,
            redirect_uris: payload
                .redirect_uris
                .into_iter()
                .map(|item| CreateRedirectUriInput {
                    redirect_uri: item.redirect_uri,
                    is_primary: item.is_primary.unwrap_or(false),
                })
                .collect(),
        },
    )
    .await
    .map_err(map_management_error)?;

    Ok(Json(map_created_application(result)))
}

pub async fn get_application(
    State(state): State<AppState>,
    _session: AdminSession,
    Path(application_id): Path<Uuid>,
) -> Result<Json<ApplicationDetailResponse>, AppError> {
    let detail = get_application_detail(&state.db, application_id)
        .await
        .map_err(map_management_error)?;

    Ok(Json(ApplicationDetailResponse {
        id: detail.application.id,
        code: detail.application.code,
        name: detail.application.name,
        description: detail.application.description,
        application_type: detail.application.application_type,
        client_id: detail.application.client_id,
        user_pool: detail.user_pool.into(),
        homepage_url: detail.application.homepage_url,
        is_active: detail.application.is_active,
        redirect_uris: detail.redirect_uris.into_iter().map(Into::into).collect(),
        created_at: detail.application.created_at,
        updated_at: detail.application.updated_at,
    }))
}

pub async fn update_application(
    State(state): State<AppState>,
    _session: AdminSession,
    Path(application_id): Path<Uuid>,
    Json(payload): Json<UpdateApplicationRequest>,
) -> Result<Json<ApplicationDetailResponse>, AppError> {
    update_application_service(
        &state.db,
        application_id,
        UpdateApplicationInput {
            name: payload.name,
            description: payload.description,
            homepage_url: payload.homepage_url,
        },
    )
    .await
    .map_err(map_management_error)?;
    load_application_detail_response(&state, application_id).await
}

pub async fn enable_application(
    State(state): State<AppState>,
    _session: AdminSession,
    Path(application_id): Path<Uuid>,
) -> Result<Json<ApplicationDetailResponse>, AppError> {
    set_application_active(&state.db, application_id, true)
        .await
        .map_err(map_management_error)?;
    load_application_detail_response(&state, application_id).await
}

pub async fn disable_application(
    State(state): State<AppState>,
    _session: AdminSession,
    Path(application_id): Path<Uuid>,
) -> Result<Json<ApplicationDetailResponse>, AppError> {
    set_application_active(&state.db, application_id, false)
        .await
        .map_err(map_management_error)?;
    load_application_detail_response(&state, application_id).await
}

pub async fn delete_application(
    State(state): State<AppState>,
    _session: AdminSession,
    Path(application_id): Path<Uuid>,
) -> Result<Json<SuccessResponse>, AppError> {
    delete_application_service(&state.db, application_id)
        .await
        .map_err(map_management_error)?;
    Ok(Json(SuccessResponse { success: true }))
}

pub async fn reset_application_secret(
    State(state): State<AppState>,
    session: AdminSession,
    Path(application_id): Path<Uuid>,
    Json(payload): Json<ResetApplicationSecretRequest>,
) -> Result<Json<ResetApplicationSecretResponse>, AppError> {
    let result = reset_application_secret_service(
        &state.db,
        session.admin_id,
        application_id,
        &payload.current_admin_password,
    )
    .await
    .map_err(map_management_error)?;

    Ok(Json(ResetApplicationSecretResponse {
        client_id: result.application.client_id,
        client_secret: result.client_secret,
        secret_rotated_at: result.application.secret_rotated_at,
    }))
}

pub async fn list_redirect_uris(
    State(state): State<AppState>,
    _session: AdminSession,
    Path(application_id): Path<Uuid>,
) -> Result<Json<RedirectUriListResponse>, AppError> {
    let items = list_redirect_uris_service(&state.db, application_id)
        .await
        .map_err(map_management_error)?;
    Ok(Json(RedirectUriListResponse {
        items: items.into_iter().map(Into::into).collect(),
    }))
}

pub async fn create_redirect_uri(
    State(state): State<AppState>,
    _session: AdminSession,
    Path(application_id): Path<Uuid>,
    Json(payload): Json<CreateRedirectUriRequest>,
) -> Result<Json<RedirectUriResponse>, AppError> {
    let redirect_uri = create_redirect_uri_service(
        &state.db,
        application_id,
        CreateRedirectUriInput {
            redirect_uri: payload.redirect_uri,
            is_primary: payload.is_primary.unwrap_or(false),
        },
    )
    .await
    .map_err(map_management_error)?;
    Ok(Json(redirect_uri.into()))
}

pub async fn update_redirect_uri(
    State(state): State<AppState>,
    _session: AdminSession,
    Path((application_id, redirect_uri_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateRedirectUriRequest>,
) -> Result<Json<RedirectUriResponse>, AppError> {
    let redirect_uri = update_redirect_uri_service(
        &state.db,
        application_id,
        redirect_uri_id,
        UpdateRedirectUriInput {
            is_primary: payload.is_primary,
        },
    )
    .await
    .map_err(map_management_error)?;
    Ok(Json(redirect_uri.into()))
}

pub async fn delete_redirect_uri(
    State(state): State<AppState>,
    _session: AdminSession,
    Path((application_id, redirect_uri_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<SuccessResponse>, AppError> {
    delete_redirect_uri_service(&state.db, application_id, redirect_uri_id)
        .await
        .map_err(map_management_error)?;
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

fn map_management_error(error: anyhow::Error) -> AppError {
    let message = error.to_string();
    if message.contains("not found") {
        return AppError::NotFound;
    }
    if message.contains("already exists")
        || message.contains("must keep")
        || message.contains("bound applications")
    {
        return AppError::Conflict(message);
    }
    if message.contains("required")
        || message.contains("unsupported")
        || message.contains("length")
        || message.contains("may only contain")
        || message.contains("must start with")
        || message.contains("cannot delete")
        || message.contains("incorrect")
    {
        return AppError::Validation(message);
    }
    AppError::Internal
}

fn map_created_application(result: CreateApplicationResult) -> CreateApplicationResponse {
    CreateApplicationResponse {
        id: result.application.id,
        code: result.application.code,
        client_id: result.application.client_id,
        client_secret: result.client_secret,
    }
}

async fn load_application_detail_response(
    state: &AppState,
    application_id: Uuid,
) -> Result<Json<ApplicationDetailResponse>, AppError> {
    let detail = get_application_detail(&state.db, application_id)
        .await
        .map_err(map_management_error)?;

    Ok(Json(ApplicationDetailResponse {
        id: detail.application.id,
        code: detail.application.code,
        name: detail.application.name,
        description: detail.application.description,
        application_type: detail.application.application_type,
        client_id: detail.application.client_id,
        user_pool: detail.user_pool.into(),
        homepage_url: detail.application.homepage_url,
        is_active: detail.application.is_active,
        redirect_uris: detail.redirect_uris.into_iter().map(Into::into).collect(),
        created_at: detail.application.created_at,
        updated_at: detail.application.updated_at,
    }))
}

fn map_internal_error(_error: anyhow::Error) -> AppError {
    AppError::Internal
}
