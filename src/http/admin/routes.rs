use axum::{Router, routing::{get, patch, post}};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(super::handlers::login))
        .route(
            "/users",
            get(super::handlers::list_admin_users).post(super::handlers::create_admin_user),
        )
        .route(
            "/users/{admin_user_id}",
            patch(super::handlers::update_admin_user).delete(super::handlers::delete_admin_user),
        )
        .route("/me/password", post(super::handlers::change_my_password))
}
