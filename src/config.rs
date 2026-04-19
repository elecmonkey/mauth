use std::{env, net::SocketAddr, sync::Once};

use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

static TRACING_INIT: Once = Once::new();

#[derive(Clone, Debug)]
pub struct Config {
    pub app_name: String,
    pub server_addr: SocketAddr,
    pub rust_log: String,
    pub database_url: String,
    pub database_max_connections: u32,
    pub database_connect_timeout_secs: u64,
    pub database_sqlx_logging: bool,
    pub auth_issuer: String,
    pub admin_jwt_secret: String,
    pub admin_jwt_expires_days: i64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let app_name = env::var("APP_NAME").unwrap_or_else(|_| "mauth".to_string());
        let server_addr = env::var("SERVER_ADDR")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or_else(|| SocketAddr::from(([127, 0, 0, 1], 5610)));

        let rust_log = env::var("RUST_LOG")
            .unwrap_or_else(|_| "mauth=debug,tower_http=info,axum::rejection=trace".to_string());
        let database_url = required_env("DATABASE_URL")?;
        let database_max_connections = parse_env("DATABASE_MAX_CONNECTIONS", 20_u32)?;
        let database_connect_timeout_secs =
            parse_env("DATABASE_CONNECT_TIMEOUT_SECS", 8_u64)?;
        let database_sqlx_logging = parse_bool_env("DATABASE_SQLX_LOGGING", false)?;
        let auth_issuer = env::var("AUTH_ISSUER").unwrap_or_else(|_| app_name.clone());
        let admin_jwt_secret = required_env("ADMIN_JWT_SECRET")?;
        let admin_jwt_expires_days = parse_env("ADMIN_JWT_EXPIRES_DAYS", 7_i64)?;

        Ok(Self {
            app_name,
            server_addr,
            rust_log,
            database_url,
            database_max_connections,
            database_connect_timeout_secs,
            database_sqlx_logging,
            auth_issuer,
            admin_jwt_secret,
            admin_jwt_expires_days,
        })
    }

    pub fn init_tracing(&self) {
        TRACING_INIT.call_once(|| {
            tracing_subscriber::registry()
                .with(EnvFilter::new(self.rust_log.clone()))
                .with(tracing_subscriber::fmt::layer().with_target(false))
                .init();
        });
    }
}

fn required_env(key: &str) -> anyhow::Result<String> {
    env::var(key).map_err(|_| anyhow::anyhow!("missing required env var `{key}`"))
}

fn parse_env<T>(key: &str, default: T) -> anyhow::Result<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match env::var(key) {
        Ok(value) => value
            .parse::<T>()
            .map_err(|err| anyhow::anyhow!("invalid env var `{key}`: {err}")),
        Err(_) => Ok(default),
    }
}

fn parse_bool_env(key: &str, default: bool) -> anyhow::Result<bool> {
    match env::var(key) {
        Ok(value) => match value.trim().to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Ok(true),
            "0" | "false" | "no" | "off" => Ok(false),
            other => Err(anyhow::anyhow!("invalid boolean env var `{key}`: {other}")),
        },
        Err(_) => Ok(default),
    }
}
