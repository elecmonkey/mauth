mod app;
mod application;
mod cli;
mod config;
mod domain;
mod entity;
mod error;
mod http;
mod infrastructure;
mod state;

use anyhow::Context;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let command = cli::parse_args().context("failed to parse command line")?;
    if matches!(command, cli::Command::Help | cli::Command::CreateAdminHelp) {
        return Ok(());
    }

    let config = config::Config::from_env().context("failed to load config")?;
    config.init_tracing();

    match command {
        cli::Command::Serve => {
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
        }
        cli::Command::CreateAdmin(args) => {
            let db = infrastructure::db::connect_database(&config)
                .await
                .context("failed to connect database for create-admin")?;
            infrastructure::db::sync_schema(&db)
                .await
                .context("failed to synchronize schema for create-admin")?;
            cli::run_create_admin(&db, args)
                .await
                .context("failed to create admin user")?;
        }
        cli::Command::Help | cli::Command::CreateAdminHelp => {}
    }

    Ok(())
}
