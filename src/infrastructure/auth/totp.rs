use anyhow::{Context, anyhow};
use qrcode::{QrCode, render::svg};
use totp_rs::{Algorithm, Secret, TOTP};

use crate::state::AuthState;

const TOTP_DIGITS: usize = 6;
const TOTP_SKEW: u8 = 1;
const TOTP_STEP: u64 = 30;

pub struct TotpSetup {
    pub secret: String,
    pub otpauth_uri: String,
    pub qr_svg: String,
}

fn build_totp(
    secret: Secret,
    issuer: Option<String>,
    account_name: String,
) -> anyhow::Result<TOTP> {
    TOTP::new(
        Algorithm::SHA1,
        TOTP_DIGITS,
        TOTP_SKEW,
        TOTP_STEP,
        secret.to_bytes().context("failed to decode TOTP secret")?,
        issuer,
        account_name,
    )
    .context("failed to build TOTP")
}

pub fn generate_totp_setup(state: &AuthState, account_name: &str) -> anyhow::Result<TotpSetup> {
    let secret = Secret::generate_secret();
    let secret_base32 = secret.to_encoded().to_string();
    let totp = build_totp(
        Secret::Encoded(secret_base32.clone()),
        Some(state.issuer.to_string()),
        account_name.to_string(),
    )?;
    let otpauth_uri = totp.get_url();
    let qr_svg = QrCode::new(otpauth_uri.as_bytes())
        .context("failed to generate QR code")?
        .render::<svg::Color>()
        .min_dimensions(220, 220)
        .build();

    Ok(TotpSetup {
        secret: secret_base32,
        otpauth_uri,
        qr_svg,
    })
}

pub fn verify_totp_code(secret_base32: &str, code: &str) -> anyhow::Result<bool> {
    let normalized = code.trim();
    if normalized.len() != TOTP_DIGITS || !normalized.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(anyhow!("totp code must be 6 digits"));
    }

    let totp = build_totp(
        Secret::Encoded(secret_base32.to_string()),
        None,
        "admin".to_string(),
    )?;

    totp.check_current(normalized)
        .context("failed to verify TOTP code")
}
