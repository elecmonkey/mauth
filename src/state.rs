use std::sync::Arc;

use axum::extract::FromRef;

use crate::config::Config;

#[allow(dead_code)]
#[derive(Clone)]
pub struct AppState {
    pub app_name: Arc<str>,
    pub auth: AuthState,
}

#[allow(dead_code)]
#[derive(Clone, FromRef)]
pub struct AuthState {
    pub issuer: Arc<str>,
}

pub async fn build_app_state(_config: &Config) -> anyhow::Result<AppState> {
    Ok(AppState {
        app_name: Arc::from("mauth"),
        auth: AuthState {
            issuer: Arc::from("mauth-local"),
        },
    })
}
