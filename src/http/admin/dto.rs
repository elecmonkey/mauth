use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::admin_user;

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
