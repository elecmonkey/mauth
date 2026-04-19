use axum::{
    Router,
    routing::{get, patch, post},
};

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
        .route(
            "/user-pools",
            get(super::handlers::list_user_pools).post(super::handlers::create_user_pool),
        )
        .route(
            "/user-pools/{user_pool_id}",
            get(super::handlers::get_user_pool)
                .patch(super::handlers::update_user_pool)
                .delete(super::handlers::delete_user_pool),
        )
        .route(
            "/user-pools/{user_pool_id}/enable",
            post(super::handlers::enable_user_pool),
        )
        .route(
            "/user-pools/{user_pool_id}/disable",
            post(super::handlers::disable_user_pool),
        )
        .route(
            "/user-pools/{user_pool_id}/profile-fields",
            get(super::handlers::list_profile_fields).post(super::handlers::create_profile_field),
        )
        .route(
            "/user-pools/{user_pool_id}/profile-fields/{field_id}",
            patch(super::handlers::update_profile_field)
                .delete(super::handlers::delete_profile_field),
        )
        .route(
            "/applications",
            get(super::handlers::list_applications).post(super::handlers::create_application),
        )
        .route(
            "/applications/{application_id}",
            get(super::handlers::get_application)
                .patch(super::handlers::update_application)
                .delete(super::handlers::delete_application),
        )
        .route(
            "/applications/{application_id}/enable",
            post(super::handlers::enable_application),
        )
        .route(
            "/applications/{application_id}/disable",
            post(super::handlers::disable_application),
        )
        .route(
            "/applications/{application_id}/reset-secret",
            post(super::handlers::reset_application_secret),
        )
        .route(
            "/applications/{application_id}/redirect-uris",
            get(super::handlers::list_redirect_uris).post(super::handlers::create_redirect_uri),
        )
        .route(
            "/applications/{application_id}/redirect-uris/{redirect_uri_id}",
            patch(super::handlers::update_redirect_uri)
                .delete(super::handlers::delete_redirect_uri),
        )
}
