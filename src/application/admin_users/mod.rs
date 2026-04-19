use anyhow::{Context, bail};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::entity::admin_user;

pub struct CreateAdminInput {
    pub email: String,
    pub nickname: String,
    pub password: String,
    pub is_active: bool,
}

pub struct UpdateAdminInput {
    pub nickname: Option<String>,
    pub is_active: Option<bool>,
}

pub struct ChangePasswordInput {
    pub current_password: String,
    pub new_password: String,
}

pub struct ListAdminUsersInput {
    pub keyword: Option<String>,
    pub page: u64,
    pub page_size: u64,
}

pub struct ListAdminUsersResult {
    pub items: Vec<admin_user::Model>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

pub async fn authenticate_admin(
    db: &DatabaseConnection,
    email: &str,
    password: &str,
) -> anyhow::Result<admin_user::Model> {
    let email = normalize_email(email)?;
    let password = normalize_password(password)?;

    let admin = admin_user::Entity::find()
        .filter(admin_user::COLUMN.email.eq(email.clone()))
        .one(db)
        .await
        .context("failed to query admin by email")?
        .ok_or_else(|| anyhow::anyhow!("invalid email or password"))?;

    if !verify_password(&password, &admin.password_hash)? {
        bail!("invalid email or password");
    }

    if !admin.is_active {
        bail!("admin account is disabled");
    }

    Ok(admin)
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
        is_active: Set(input.is_active),
        ..Default::default()
    }
    .insert(db)
    .await
    .context("failed to insert admin user")?;

    Ok(admin)
}

pub async fn list_admin_users(
    db: &DatabaseConnection,
    input: ListAdminUsersInput,
) -> anyhow::Result<ListAdminUsersResult> {
    let keyword = input
        .keyword
        .as_deref()
        .map(str::trim)
        .filter(|it| !it.is_empty());
    let page = input.page.max(1);
    let page_size = input.page_size.clamp(1, 100);
    let mut query = admin_user::Entity::find().order_by_desc(admin_user::COLUMN.created_at);

    if let Some(keyword) = keyword {
        query = query.filter(
            Condition::any()
                .add(
                    admin_user::COLUMN
                        .email
                        .contains(keyword.to_ascii_lowercase()),
                )
                .add(admin_user::COLUMN.nickname.contains(keyword)),
        );
    }

    let paginator = query.paginate(db, page_size);
    let total = paginator
        .num_items()
        .await
        .context("failed to count admin users")?;
    let items = paginator
        .fetch_page(page - 1)
        .await
        .context("failed to list admin users")?;

    Ok(ListAdminUsersResult {
        items,
        total,
        page,
        page_size,
    })
}

pub async fn get_admin_user(
    db: &DatabaseConnection,
    admin_id: Uuid,
) -> anyhow::Result<admin_user::Model> {
    admin_user::Entity::find_by_id(admin_id)
        .one(db)
        .await
        .context("failed to get admin user by id")?
        .ok_or_else(|| anyhow::anyhow!("admin user not found"))
}

pub async fn update_admin_user(
    db: &DatabaseConnection,
    actor_admin_id: Uuid,
    target_admin_id: Uuid,
    input: UpdateAdminInput,
) -> anyhow::Result<admin_user::Model> {
    let admin = get_admin_user(db, target_admin_id).await?;
    let mut active_model = admin.into_active_model();

    if let Some(nickname) = input.nickname {
        active_model.nickname = Set(normalize_nickname(&nickname)?);
    }

    if let Some(is_active) = input.is_active {
        if actor_admin_id == target_admin_id && !is_active {
            bail!("cannot disable current logged-in admin");
        }
        active_model.is_active = Set(is_active);
    }

    active_model.updated_at = Set(Utc::now());

    active_model
        .update(db)
        .await
        .context("failed to update admin user")
}

pub async fn delete_admin_user(
    db: &DatabaseConnection,
    actor_admin_id: Uuid,
    target_admin_id: Uuid,
) -> anyhow::Result<()> {
    if actor_admin_id == target_admin_id {
        bail!("cannot delete current logged-in admin");
    }

    let admin = get_admin_user(db, target_admin_id).await?;
    admin_user::Entity::delete(admin.into_active_model())
        .exec(db)
        .await
        .context("failed to delete admin user")?;

    Ok(())
}

pub async fn change_own_password(
    db: &DatabaseConnection,
    admin_id: Uuid,
    input: ChangePasswordInput,
) -> anyhow::Result<()> {
    let current_password = normalize_password(&input.current_password)?;
    let new_password = normalize_password(&input.new_password)?;

    let admin = get_admin_user(db, admin_id).await?;
    if !verify_password(&current_password, &admin.password_hash)? {
        bail!("current password is incorrect");
    }

    let mut active_model = admin.into_active_model();
    active_model.password_hash = Set(hash_password(&new_password)?);
    active_model.password_changed_at = Set(Utc::now());
    active_model.updated_at = Set(Utc::now());

    active_model
        .update(db)
        .await
        .context("failed to change own password")?;

    Ok(())
}

pub async fn touch_last_login(
    db: &DatabaseConnection,
    admin_id: Uuid,
) -> anyhow::Result<admin_user::Model> {
    let admin = get_admin_user(db, admin_id).await?;
    let now = Utc::now();
    let mut active_model = admin.into_active_model();
    active_model.last_login_at = Set(Some(now));
    active_model.updated_at = Set(now);

    active_model
        .update(db)
        .await
        .context("failed to update last login timestamp")
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

pub(crate) fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::encode_b64(&Uuid::new_v4().into_bytes())
        .map_err(|err| anyhow::anyhow!("failed to generate password salt: {err}"))?;

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|err| anyhow::anyhow!("failed to hash password: {err}"))
}

pub fn verify_password(password: &str, password_hash: &str) -> anyhow::Result<bool> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|err| anyhow::anyhow!("invalid password hash: {err}"))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
