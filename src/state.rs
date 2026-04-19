use std::sync::Arc;

use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

use crate::{config::Config, infrastructure};

#[derive(Clone)]
pub struct AppState {
    pub app_name: Arc<str>,
    pub db: DatabaseConnection,
    pub auth: AuthState,
}

#[derive(Clone)]
pub struct AuthState {
    pub issuer: Arc<str>,
    pub admin_jwt_secret: Arc<str>,
    pub admin_jwt_expires_days: i64,
    pub admin_totp_encryption_key: Arc<str>,
}

impl FromRef<AppState> for AuthState {
    fn from_ref(input: &AppState) -> Self {
        input.auth.clone()
    }
}

pub async fn build_app_state(config: &Config) -> anyhow::Result<AppState> {
    let db = infrastructure::db::connect_database(config).await?;
    infrastructure::db::sync_schema(&db).await?;

    Ok(AppState {
        app_name: Arc::from(config.app_name.clone()),
        db,
        auth: AuthState {
            issuer: Arc::from(config.auth_issuer.clone()),
            admin_jwt_secret: Arc::from(config.admin_jwt_secret.clone()),
            admin_jwt_expires_days: config.admin_jwt_expires_days,
            admin_totp_encryption_key: Arc::from(config.admin_totp_encryption_key.clone()),
        },
    })
}
