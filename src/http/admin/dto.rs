use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::entity::{
    admin_user, application, application_redirect_uri, user_pool, user_pool_profile_field,
};

#[derive(Debug, Deserialize)]
pub struct AdminLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminLoginTotpRequest {
    pub challenge_token: String,
    pub totp_code: String,
}

#[derive(Debug, Deserialize)]
pub struct ListAdminUsersQuery {
    pub keyword: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAdminUserRequest {
    pub email: String,
    pub nickname: String,
    pub password: String,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAdminUserRequest {
    pub nickname: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ListUserPoolsQuery {
    pub keyword: Option<String>,
    pub is_active: Option<bool>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserPoolRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub allow_self_signup: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserPoolRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub allow_self_signup: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProfileFieldRequest {
    pub field_key: String,
    pub field_name: String,
    pub field_type: String,
    pub is_required: Option<bool>,
    pub is_unique: Option<bool>,
    pub is_searchable: Option<bool>,
    pub sort_order: Option<i32>,
    pub options: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileFieldRequest {
    pub field_name: Option<String>,
    pub is_required: Option<bool>,
    pub is_searchable: Option<bool>,
    pub sort_order: Option<i32>,
    pub options: Option<Option<Value>>,
}

#[derive(Debug, Deserialize)]
pub struct ListApplicationsQuery {
    pub keyword: Option<String>,
    pub user_pool_id: Option<Uuid>,
    pub is_active: Option<bool>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateApplicationRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub application_type: String,
    pub user_pool_id: Uuid,
    pub homepage_url: Option<String>,
    pub redirect_uris: Vec<CreateRedirectUriRequest>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateApplicationRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub homepage_url: Option<Option<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRedirectUriRequest {
    pub redirect_uri: String,
    pub is_primary: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRedirectUriRequest {
    pub is_primary: bool,
}

#[derive(Debug, Deserialize)]
pub struct ResetApplicationSecretRequest {
    pub current_admin_password: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangeMyPasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmTotpRequest {
    pub totp_code: String,
}

#[derive(Debug, Deserialize)]
pub struct DisableTotpRequest {
    pub current_password: String,
    pub totp_code: String,
}

#[derive(Debug, Serialize)]
pub struct AdminUserResponse {
    pub id: Uuid,
    pub email: String,
    pub nickname: String,
    pub is_active: bool,
    pub totp_enabled: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminLoginResponse {
    pub status: &'static str,
    pub requires_totp: bool,
    pub access_token: Option<String>,
    pub token_type: Option<&'static str>,
    pub expires_in: Option<i64>,
    pub challenge_token: Option<String>,
    pub challenge_expires_in: Option<i64>,
    pub user: AdminUserResponse,
}

#[derive(Debug, Serialize)]
pub struct TotpStatusResponse {
    pub enabled: bool,
}

#[derive(Debug, Serialize)]
pub struct TotpSetupResponse {
    pub secret: String,
    pub otpauth_uri: String,
    pub qr_svg: String,
}

#[derive(Debug, Serialize)]
pub struct TotpUpdateResponse {
    pub success: bool,
    pub user: AdminUserResponse,
}

#[derive(Debug, Serialize)]
pub struct PaginationResponse {
    pub page: u64,
    pub page_size: u64,
    pub total: u64,
}

#[derive(Debug, Serialize)]
pub struct AdminUsersListResponse {
    pub items: Vec<AdminUserResponse>,
    pub pagination: PaginationResponse,
}

#[derive(Debug, Serialize)]
pub struct UserPoolSummaryResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub allow_self_signup: bool,
    pub application_count: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserPoolListResponse {
    pub items: Vec<UserPoolSummaryResponse>,
    pub pagination: PaginationResponse,
}

#[derive(Debug, Serialize)]
pub struct ProfileFieldResponse {
    pub id: Uuid,
    pub user_pool_id: Uuid,
    pub field_key: String,
    pub field_name: String,
    pub field_type: String,
    pub is_required: bool,
    pub is_unique: bool,
    pub is_searchable: bool,
    pub sort_order: i32,
    pub options: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ProfileFieldListResponse {
    pub items: Vec<ProfileFieldResponse>,
}

#[derive(Debug, Serialize)]
pub struct UserPoolApplicationSummaryResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct UserPoolDetailResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub allow_self_signup: bool,
    pub profile_fields: Vec<ProfileFieldResponse>,
    pub applications: Vec<UserPoolApplicationSummaryResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ApplicationUserPoolResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ApplicationSummaryResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub application_type: String,
    pub client_id: String,
    pub user_pool: ApplicationUserPoolResponse,
    pub homepage_url: Option<String>,
    pub is_active: bool,
    pub redirect_uri_count: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ApplicationListResponse {
    pub items: Vec<ApplicationSummaryResponse>,
    pub pagination: PaginationResponse,
}

#[derive(Debug, Serialize)]
pub struct RedirectUriResponse {
    pub id: Uuid,
    pub application_id: Uuid,
    pub redirect_uri: String,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct RedirectUriListResponse {
    pub items: Vec<RedirectUriResponse>,
}

#[derive(Debug, Serialize)]
pub struct ApplicationDetailResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub application_type: String,
    pub client_id: String,
    pub user_pool: ApplicationUserPoolResponse,
    pub homepage_url: Option<String>,
    pub is_active: bool,
    pub redirect_uris: Vec<RedirectUriResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CreateApplicationResponse {
    pub id: Uuid,
    pub code: String,
    pub client_id: String,
    pub client_secret: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ResetApplicationSecretResponse {
    pub client_id: String,
    pub client_secret: String,
    pub secret_rotated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

impl From<admin_user::Model> for AdminUserResponse {
    fn from(value: admin_user::Model) -> Self {
        Self {
            id: value.id,
            email: value.email,
            nickname: value.nickname,
            is_active: value.is_active,
            totp_enabled: value.totp_enabled,
            last_login_at: value.last_login_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<user_pool_profile_field::Model> for ProfileFieldResponse {
    fn from(value: user_pool_profile_field::Model) -> Self {
        Self {
            id: value.id,
            user_pool_id: value.user_pool_id,
            field_key: value.field_key,
            field_name: value.field_name,
            field_type: value.field_type,
            is_required: value.is_required,
            is_unique: value.is_unique,
            is_searchable: value.is_searchable,
            sort_order: value.sort_order,
            options: value.options_json,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<user_pool::Model> for ApplicationUserPoolResponse {
    fn from(value: user_pool::Model) -> Self {
        Self {
            id: value.id,
            code: value.code,
            name: value.name,
        }
    }
}

impl From<application_redirect_uri::Model> for RedirectUriResponse {
    fn from(value: application_redirect_uri::Model) -> Self {
        Self {
            id: value.id,
            application_id: value.application_id,
            redirect_uri: value.redirect_uri,
            is_primary: value.is_primary,
            created_at: value.created_at,
        }
    }
}

impl From<(user_pool::Model, u64)> for UserPoolSummaryResponse {
    fn from((value, application_count): (user_pool::Model, u64)) -> Self {
        Self {
            id: value.id,
            code: value.code,
            name: value.name,
            description: value.description,
            is_active: value.is_active,
            allow_self_signup: value.allow_self_signup,
            application_count,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<(application::Model, user_pool::Model, u64)> for ApplicationSummaryResponse {
    fn from(
        (application, user_pool, redirect_uri_count): (application::Model, user_pool::Model, u64),
    ) -> Self {
        Self {
            id: application.id,
            code: application.code,
            name: application.name,
            description: application.description,
            application_type: application.application_type,
            client_id: application.client_id,
            user_pool: user_pool.into(),
            homepage_url: application.homepage_url,
            is_active: application.is_active,
            redirect_uri_count,
            created_at: application.created_at,
            updated_at: application.updated_at,
        }
    }
}
