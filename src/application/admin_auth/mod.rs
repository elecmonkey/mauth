use anyhow::{Context, anyhow};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};
use uuid::Uuid;

use crate::{
    application::admin_users::{authenticate_admin, touch_last_login, verify_password},
    entity::admin_user,
    infrastructure::auth::{
        self, ADMIN_LOGIN_CHALLENGE_TTL_SECS,
        crypto::{decrypt_totp_secret, encrypt_totp_secret},
        totp::{generate_totp_setup, verify_totp_code},
    },
    state::AuthState,
};

pub struct AdminAuthenticationOutput {
    pub admin: admin_user::Model,
    pub access_token: String,
    pub expires_in: i64,
}

pub struct PendingTotpOutput {
    pub admin: admin_user::Model,
    pub challenge_token: String,
    pub challenge_expires_in: i64,
}

pub enum BeginAdminLoginOutput {
    Authenticated(AdminAuthenticationOutput),
    PendingTotp(PendingTotpOutput),
}

pub struct TotpSetupOutput {
    pub secret: String,
    pub otpauth_uri: String,
    pub qr_svg: String,
}

pub struct TotpStatusOutput {
    pub enabled: bool,
}

pub async fn begin_admin_login(
    db: &DatabaseConnection,
    auth_state: &AuthState,
    email: &str,
    password: &str,
) -> anyhow::Result<BeginAdminLoginOutput> {
    let admin = authenticate_admin(db, email, password).await?;

    if admin.totp_enabled {
        let claims = auth::issue_admin_login_challenge(auth_state, &admin)?;
        let challenge_token = auth::encode_admin_login_challenge(auth_state, &claims)?;

        return Ok(BeginAdminLoginOutput::PendingTotp(PendingTotpOutput {
            admin,
            challenge_token,
            challenge_expires_in: ADMIN_LOGIN_CHALLENGE_TTL_SECS,
        }));
    }

    let admin = touch_last_login(db, admin.id).await?;
    Ok(BeginAdminLoginOutput::Authenticated(
        authenticate_without_totp(auth_state, admin)?,
    ))
}

pub async fn complete_admin_totp_login(
    db: &DatabaseConnection,
    auth_state: &AuthState,
    challenge_token: &str,
    totp_code: &str,
) -> anyhow::Result<AdminAuthenticationOutput> {
    let claims = auth::decode_admin_login_challenge(challenge_token, auth_state)?;
    let admin_id = auth::parse_admin_id_from_challenge(&claims)?;
    let admin = find_admin_by_id(db, admin_id).await?;

    if !admin.is_active {
        anyhow::bail!("admin account is disabled");
    }
    if !admin.totp_enabled {
        anyhow::bail!("totp is not enabled for admin account");
    }
    if claims.pwd_changed_at != admin.password_changed_at.timestamp() {
        anyhow::bail!("login challenge is no longer valid");
    }

    let encrypted_secret = admin
        .totp_secret_encrypted
        .as_deref()
        .ok_or_else(|| anyhow!("totp secret not found"))?;
    let secret = decrypt_totp_secret(auth_state, encrypted_secret)?;
    let is_valid = verify_totp_code(&secret, totp_code)?;

    if !is_valid {
        anyhow::bail!("totp code is invalid");
    }

    let admin = touch_last_login(db, admin.id).await?;
    authenticate_without_totp(auth_state, admin)
}

pub async fn get_admin_totp_status(
    db: &DatabaseConnection,
    admin_id: Uuid,
) -> anyhow::Result<TotpStatusOutput> {
    let admin = find_admin_by_id(db, admin_id).await?;
    Ok(TotpStatusOutput {
        enabled: admin.totp_enabled,
    })
}

pub async fn setup_admin_totp(
    db: &DatabaseConnection,
    auth_state: &AuthState,
    admin_id: Uuid,
) -> anyhow::Result<TotpSetupOutput> {
    let admin = find_admin_by_id(db, admin_id).await?;
    if admin.totp_enabled {
        anyhow::bail!("totp is already enabled");
    }

    let setup = generate_totp_setup(auth_state, &admin.email)?;
    let encrypted_secret = encrypt_totp_secret(auth_state, &setup.secret)?;

    let mut active_model = admin.into_active_model();
    active_model.totp_secret_encrypted = Set(Some(encrypted_secret));
    active_model.totp_enabled = Set(false);
    active_model
        .update(db)
        .await
        .context("failed to save pending TOTP setup")?;

    Ok(TotpSetupOutput {
        secret: setup.secret,
        otpauth_uri: setup.otpauth_uri,
        qr_svg: setup.qr_svg,
    })
}

pub async fn confirm_admin_totp(
    db: &DatabaseConnection,
    auth_state: &AuthState,
    admin_id: Uuid,
    totp_code: &str,
) -> anyhow::Result<admin_user::Model> {
    let admin = find_admin_by_id(db, admin_id).await?;
    let encrypted_secret = admin
        .totp_secret_encrypted
        .as_deref()
        .ok_or_else(|| anyhow!("totp setup has not been initialized"))?;
    let secret = decrypt_totp_secret(auth_state, encrypted_secret)?;
    let is_valid = verify_totp_code(&secret, totp_code)?;

    if !is_valid {
        anyhow::bail!("totp code is invalid");
    }

    let mut active_model = admin.into_active_model();
    active_model.totp_enabled = Set(true);
    active_model
        .update(db)
        .await
        .context("failed to enable TOTP")
}

pub async fn disable_admin_totp(
    db: &DatabaseConnection,
    auth_state: &AuthState,
    admin_id: Uuid,
    current_password: &str,
    totp_code: &str,
) -> anyhow::Result<admin_user::Model> {
    let admin = find_admin_by_id(db, admin_id).await?;

    if !admin.totp_enabled {
        anyhow::bail!("totp is not enabled");
    }

    let encrypted_secret = admin
        .totp_secret_encrypted
        .as_deref()
        .ok_or_else(|| anyhow!("totp secret not found"))?;

    if !verify_password(current_password, &admin.password_hash)? {
        anyhow::bail!("current password is incorrect");
    }

    let secret = decrypt_totp_secret(auth_state, encrypted_secret)?;
    let is_valid = verify_totp_code(&secret, totp_code)?;

    if !is_valid {
        anyhow::bail!("totp code is invalid");
    }

    let mut active_model = admin.into_active_model();
    active_model.totp_enabled = Set(false);
    active_model.totp_secret_encrypted = Set(None);
    active_model
        .update(db)
        .await
        .context("failed to disable TOTP")
}

async fn find_admin_by_id(
    db: &DatabaseConnection,
    admin_id: Uuid,
) -> anyhow::Result<admin_user::Model> {
    admin_user::Entity::find_by_id(admin_id)
        .one(db)
        .await
        .context("failed to query admin user")?
        .ok_or_else(|| anyhow!("admin user not found"))
}

fn authenticate_without_totp(
    auth_state: &AuthState,
    admin: admin_user::Model,
) -> anyhow::Result<AdminAuthenticationOutput> {
    let claims = auth::issue_admin_jwt(auth_state, &admin)?;
    let access_token = auth::encode_admin_jwt(auth_state, &claims)?;

    Ok(AdminAuthenticationOutput {
        admin,
        access_token,
        expires_in: auth_state.admin_jwt_expires_days * 24 * 60 * 60,
    })
}
