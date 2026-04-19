use std::time::Duration;

use anyhow::Context;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::info;

use crate::config::Config;

pub async fn connect_database(config: &Config) -> anyhow::Result<DatabaseConnection> {
    let mut options = ConnectOptions::new(config.database_url.clone());
    options
        .max_connections(config.database_max_connections)
        .connect_timeout(Duration::from_secs(config.database_connect_timeout_secs))
        .sqlx_logging(config.database_sqlx_logging);

    let db = Database::connect(options)
        .await
        .with_context(|| format!("failed to connect database for `{}`", config.app_name))?;

    info!(
        max_connections = config.database_max_connections,
        sqlx_logging = config.database_sqlx_logging,
        "database connected"
    );

    Ok(db)
}
