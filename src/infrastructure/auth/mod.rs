use anyhow::Context;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{entity::admin_user, state::AuthState};

pub mod crypto;
pub mod totp;

pub const ADMIN_LOGIN_CHALLENGE_TTL_SECS: i64 = 300;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminJwtClaims {
    pub sub: String,
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
    #[serde(default)]
    pub amr: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminLoginChallengeClaims {
    pub sub: String,
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
    pub purpose: String,
    pub pwd_changed_at: i64,
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
        amr: vec![
            "pwd".to_string(),
            if admin.totp_enabled {
                "totp".to_string()
            } else {
                "single_factor".to_string()
            },
        ],
    })
}

pub fn issue_admin_login_challenge(
    auth_state: &AuthState,
    admin: &admin_user::Model,
) -> anyhow::Result<AdminLoginChallengeClaims> {
    let now = Utc::now().timestamp();
    Ok(AdminLoginChallengeClaims {
        sub: admin.id.to_string(),
        iss: auth_state.issuer.to_string(),
        iat: now,
        exp: now + ADMIN_LOGIN_CHALLENGE_TTL_SECS,
        purpose: "admin_login_totp".to_string(),
        pwd_changed_at: admin.password_changed_at.timestamp(),
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

pub fn encode_admin_login_challenge(
    auth_state: &AuthState,
    claims: &AdminLoginChallengeClaims,
) -> anyhow::Result<String> {
    encode(
        &Header::new(Algorithm::HS256),
        claims,
        &EncodingKey::from_secret(auth_state.admin_jwt_secret.as_bytes()),
    )
    .context("failed to encode admin login challenge")
}

pub fn decode_admin_jwt(token: &str, auth_state: &AuthState) -> anyhow::Result<AdminJwtClaims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[auth_state.issuer.as_ref()]);
    validation.validate_exp = true;
    validation.required_spec_claims.extend([
        "exp".into(),
        "iat".into(),
        "iss".into(),
        "sub".into(),
    ]);

    decode::<AdminJwtClaims>(
        token,
        &DecodingKey::from_secret(auth_state.admin_jwt_secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .context("failed to decode admin jwt")
}

pub fn decode_admin_login_challenge(
    token: &str,
    auth_state: &AuthState,
) -> anyhow::Result<AdminLoginChallengeClaims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[auth_state.issuer.as_ref()]);
    validation.validate_exp = true;
    validation.required_spec_claims.extend([
        "exp".into(),
        "iat".into(),
        "iss".into(),
        "sub".into(),
        "purpose".into(),
        "pwd_changed_at".into(),
    ]);

    let claims = decode::<AdminLoginChallengeClaims>(
        token,
        &DecodingKey::from_secret(auth_state.admin_jwt_secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .context("failed to decode admin login challenge")?;

    if claims.purpose != "admin_login_totp" {
        anyhow::bail!("invalid admin login challenge purpose");
    }

    Ok(claims)
}

pub fn parse_admin_id(claims: &AdminJwtClaims) -> anyhow::Result<Uuid> {
    Uuid::parse_str(&claims.sub).context("invalid admin jwt subject")
}

pub fn parse_admin_id_from_challenge(claims: &AdminLoginChallengeClaims) -> anyhow::Result<Uuid> {
    Uuid::parse_str(&claims.sub).context("invalid admin challenge subject")
}
