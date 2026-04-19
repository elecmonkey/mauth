use std::{env, net::SocketAddr, sync::Once};

use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

static TRACING_INIT: Once = Once::new();

#[derive(Clone, Debug)]
pub struct Config {
    pub server_addr: SocketAddr,
    pub rust_log: String,
}

impl Config {
    pub fn from_env() -> Self {
        let server_addr = env::var("SERVER_ADDR")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or_else(|| SocketAddr::from(([127, 0, 0, 1], 5610)));

        let rust_log = env::var("RUST_LOG")
            .unwrap_or_else(|_| "mauth=debug,tower_http=info,axum::rejection=trace".to_string());

        Self {
            server_addr,
            rust_log,
        }
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
