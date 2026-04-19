use axum::{
    Router,
    routing::{get, post},
};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(super::handlers::login))
        .route("/authorize", get(super::handlers::authorize))
        .route("/token", post(super::handlers::issue_token))
}
