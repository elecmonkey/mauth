use std::sync::Arc;

use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

use crate::{config::Config, infrastructure};

#[allow(dead_code)]
#[derive(Clone)]
pub struct AppState {
    pub app_name: Arc<str>,
    pub db: DatabaseConnection,
    pub auth: AuthState,
}

#[allow(dead_code)]
#[derive(Clone, FromRef)]
pub struct AuthState {
    pub issuer: Arc<str>,
}

pub async fn build_app_state(config: &Config) -> anyhow::Result<AppState> {
    let db = infrastructure::db::connect_database(config).await?;

    Ok(AppState {
        app_name: Arc::from(config.app_name.clone()),
        db,
        auth: AuthState {
            issuer: Arc::from(config.auth_issuer.clone()),
        },
    })
}
