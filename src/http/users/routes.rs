use axum::{Router, routing::get};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(super::handlers::list_users).post(super::handlers::create_user),
        )
        .route("/{user_id}", get(super::handlers::get_user))
}
