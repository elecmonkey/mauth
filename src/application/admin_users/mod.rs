use anyhow::{Context, bail};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString},
};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::entity::admin_user;

pub struct CreateAdminInput {
    pub email: String,
    pub nickname: String,
    pub password: String,
}

pub async fn create_admin(
    db: &DatabaseConnection,
    input: CreateAdminInput,
) -> anyhow::Result<admin_user::Model> {
    let email = normalize_email(&input.email)?;
    let nickname = normalize_nickname(&input.nickname)?;
    let password = normalize_password(&input.password)?;

    let existing = admin_user::Entity::find()
        .filter(admin_user::COLUMN.email.eq(email.clone()))
        .one(db)
        .await
        .context("failed to check existing admin email")?;

    if existing.is_some() {
        bail!("admin email already exists: {email}");
    }

    let password_hash = hash_password(&password)?;

    let admin = admin_user::ActiveModel {
        email: Set(email),
        nickname: Set(nickname),
        password_hash: Set(password_hash),
        ..Default::default()
    }
    .insert(db)
    .await
    .context("failed to insert admin user")?;

    Ok(admin)
}

fn normalize_email(input: &str) -> anyhow::Result<String> {
    let email = input.trim().to_ascii_lowercase();
    if email.is_empty() {
        bail!("email is required");
    }
    if !email.contains('@') {
        bail!("email must contain `@`");
    }
    Ok(email)
}

fn normalize_nickname(input: &str) -> anyhow::Result<String> {
    let nickname = input.trim();
    if nickname.is_empty() {
        bail!("nickname is required");
    }
    Ok(nickname.to_string())
}

fn normalize_password(input: &str) -> anyhow::Result<String> {
    let password = input.trim();
    if password.len() < 8 {
        bail!("password must be at least 8 characters");
    }
    Ok(password.to_string())
}

fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::encode_b64(&Uuid::new_v4().into_bytes())
        .map_err(|err| anyhow::anyhow!("failed to generate password salt: {err}"))?;

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|err| anyhow::anyhow!("failed to hash password: {err}"))
}
