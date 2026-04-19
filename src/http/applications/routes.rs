use axum::{Router, routing::get};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(super::handlers::list_applications).post(super::handlers::create_application),
        )
        .route("/{application_id}", get(super::handlers::get_application))
}
