use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use chrono::Utc;
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::{entity::admin_user, error::AppError, infrastructure::auth, state::AppState};

pub struct AdminSession {
    pub admin_id: Uuid,
}

impl FromRequestParts<AppState> for AdminSession {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or_else(|| AppError::Unauthorized("missing bearer token".into()))?;

        let header = header
            .to_str()
            .map_err(|_| AppError::Unauthorized("invalid authorization header".into()))?;

        let token = header
            .strip_prefix("Bearer ")
            .or_else(|| header.strip_prefix("bearer "))
            .ok_or_else(|| AppError::Unauthorized("invalid bearer token".into()))?;

        let claims = auth::decode_admin_jwt(token, &state.auth)
            .map_err(|_| AppError::Unauthorized("token is invalid or expired".into()))?;
        let admin_id = auth::parse_admin_id(&claims)
            .map_err(|_| AppError::Unauthorized("token subject is invalid".into()))?;

        let admin = admin_user::Entity::find_by_id(admin_id)
            .one(&state.db)
            .await
            .map_err(|_| AppError::Internal)?
            .ok_or_else(|| AppError::Unauthorized("admin user not found".into()))?;

        if !admin.is_active {
            return Err(AppError::Forbidden("admin account is disabled".into()));
        }

        if claims.iat < admin.password_changed_at.timestamp() {
            return Err(AppError::Unauthorized(
                "token was issued before password change".into(),
            ));
        }

        if claims.exp <= Utc::now().timestamp() {
            return Err(AppError::Unauthorized("token is expired".into()));
        }
        Ok(Self { admin_id })
    }
}
