mod app;
mod application;
mod config;
mod domain;
mod error;
mod http;
mod infrastructure;
mod state;

use anyhow::Context;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = config::Config::from_env().context("failed to load config")?;
    config.init_tracing();

    let state = state::build_app_state(&config)
        .await
        .context("failed to build app state")?;
    let app = app::build_app(state);

    let listener = tokio::net::TcpListener::bind(config.server_addr)
        .await
        .with_context(|| format!("failed to bind {}", config.server_addr))?;

    info!(addr = %config.server_addr, "mauth backend listening");
    axum::serve(listener, app)
        .await
        .context("axum server exited unexpectedly")?;

    Ok(())
}
