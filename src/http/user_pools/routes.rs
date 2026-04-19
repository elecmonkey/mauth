use axum::{Router, routing::get};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(super::handlers::list_user_pools).post(super::handlers::create_user_pool),
        )
        .route("/{user_pool_id}", get(super::handlers::get_user_pool))
}
