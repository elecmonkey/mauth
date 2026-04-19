use anyhow::Context;
use chrono::Utc;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{entity::admin_user, state::AuthState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminJwtClaims {
    pub sub: String,
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn issue_admin_jwt(
    auth_state: &AuthState,
    admin: &admin_user::Model,
) -> anyhow::Result<AdminJwtClaims> {
    let now = Utc::now().timestamp();
    let exp = now + auth_state.admin_jwt_expires_days * 24 * 60 * 60;

    Ok(AdminJwtClaims {
        sub: admin.id.to_string(),
        iss: auth_state.issuer.to_string(),
        iat: now,
        exp,
    })
}

pub fn encode_admin_jwt(auth_state: &AuthState, claims: &AdminJwtClaims) -> anyhow::Result<String> {
    encode(
        &Header::new(Algorithm::HS256),
        claims,
        &EncodingKey::from_secret(auth_state.admin_jwt_secret.as_bytes()),
    )
    .context("failed to encode admin jwt")
}

pub fn decode_admin_jwt(token: &str, auth_state: &AuthState) -> anyhow::Result<AdminJwtClaims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[auth_state.issuer.as_ref()]);
    validation.validate_exp = true;
    validation.required_spec_claims.extend(["exp".into(), "iat".into(), "iss".into(), "sub".into()]);

    decode::<AdminJwtClaims>(
        token,
        &DecodingKey::from_secret(auth_state.admin_jwt_secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .context("failed to decode admin jwt")
}

pub fn parse_admin_id(claims: &AdminJwtClaims) -> anyhow::Result<Uuid> {
    Uuid::parse_str(&claims.sub).context("invalid admin jwt subject")
}
