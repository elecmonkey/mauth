use axum::Router;

use crate::{http, state::AppState};

pub fn build_app(state: AppState) -> Router {
    http::middleware::apply_common_layers(
        Router::new()
            .merge(http::routes())
            .fallback(http::not_found),
    )
    .with_state(state)
}
