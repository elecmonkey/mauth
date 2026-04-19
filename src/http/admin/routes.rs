use axum::{Router, routing::{get, patch, post}};

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(super::handlers::login))
        .route("/login/totp", post(super::handlers::login_totp))
        .route(
            "/users",
            get(super::handlers::list_admin_users).post(super::handlers::create_admin_user),
        )
        .route(
            "/users/{admin_user_id}",
            patch(super::handlers::update_admin_user).delete(super::handlers::delete_admin_user),
        )
        .route("/me/password", post(super::handlers::change_my_password))
        .route("/me/totp", get(super::handlers::get_my_totp_status))
        .route("/me/totp/setup", post(super::handlers::setup_my_totp))
        .route("/me/totp/confirm", post(super::handlers::confirm_my_totp))
        .route("/me/totp/disable", post(super::handlers::disable_my_totp))
}
