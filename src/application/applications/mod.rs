use anyhow::{Context, bail};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DatabaseTransaction, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};
use uuid::Uuid;

use crate::{
    application::admin_users::{get_admin_user, hash_password, verify_password},
    entity::{application, application_redirect_uri, user_pool},
};

pub struct CreateApplicationInput {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub application_type: String,
    pub user_pool_id: Uuid,
    pub homepage_url: Option<String>,
    pub redirect_uris: Vec<CreateRedirectUriInput>,
}

pub struct UpdateApplicationInput {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub homepage_url: Option<Option<String>>,
}

pub struct ListApplicationsInput {
    pub keyword: Option<String>,
    pub user_pool_id: Option<Uuid>,
    pub is_active: Option<bool>,
    pub page: u64,
    pub page_size: u64,
}

pub struct ListApplicationsItem {
    pub application: application::Model,
    pub user_pool: user_pool::Model,
    pub redirect_uri_count: u64,
}

pub struct ListApplicationsResult {
    pub items: Vec<ListApplicationsItem>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

pub struct ApplicationDetail {
    pub application: application::Model,
    pub user_pool: user_pool::Model,
    pub redirect_uris: Vec<application_redirect_uri::Model>,
}

pub struct CreateApplicationResult {
    pub application: application::Model,
    pub client_secret: Option<String>,
}

pub struct ResetApplicationSecretResult {
    pub application: application::Model,
    pub client_secret: String,
}

pub struct CreateRedirectUriInput {
    pub redirect_uri: String,
    pub is_primary: bool,
}

pub struct UpdateRedirectUriInput {
    pub is_primary: bool,
}

pub async fn list_applications(
    db: &DatabaseConnection,
    input: ListApplicationsInput,
) -> anyhow::Result<ListApplicationsResult> {
    let keyword = input
        .keyword
        .as_deref()
        .map(str::trim)
        .filter(|it| !it.is_empty());
    let page = input.page.max(1);
    let page_size = input.page_size.clamp(1, 100);

    let mut query = application::Entity::find().order_by_desc(application::Column::CreatedAt);

    if let Some(keyword) = keyword {
        query = query.filter(
            Condition::any()
                .add(application::Column::Code.contains(keyword))
                .add(application::Column::Name.contains(keyword))
                .add(application::Column::ClientId.contains(keyword)),
        );
    }
    if let Some(user_pool_id) = input.user_pool_id {
        query = query.filter(application::Column::UserPoolId.eq(user_pool_id));
    }
    if let Some(is_active) = input.is_active {
        query = query.filter(application::Column::IsActive.eq(is_active));
    }

    let paginator = query.paginate(db, page_size);
    let total = paginator
        .num_items()
        .await
        .context("failed to count applications")?;
    let models = paginator
        .fetch_page(page - 1)
        .await
        .context("failed to list applications")?;

    let mut items = Vec::with_capacity(models.len());
    for model in models {
        let user_pool = user_pool::Entity::find_by_id(model.user_pool_id)
            .one(db)
            .await
            .context("failed to load application user pool")?
            .ok_or_else(|| anyhow::anyhow!("bound user pool not found"))?;
        let redirect_uri_count = application_redirect_uri::Entity::find()
            .filter(application_redirect_uri::Column::ApplicationId.eq(model.id))
            .count(db)
            .await
            .context("failed to count redirect uris")?;
        items.push(ListApplicationsItem {
            application: model,
            user_pool,
            redirect_uri_count,
        });
    }

    Ok(ListApplicationsResult {
        items,
        total,
        page,
        page_size,
    })
}

pub async fn create_application(
    db: &DatabaseConnection,
    input: CreateApplicationInput,
) -> anyhow::Result<CreateApplicationResult> {
    let code = normalize_code(&input.code)?;
    let name = normalize_name(&input.name, "application name")?;
    let description = normalize_optional_text(input.description, 255);
    let application_type = normalize_application_type(&input.application_type)?;
    let homepage_url = normalize_optional_url(input.homepage_url)?;
    let redirect_uris = normalize_redirect_uris(input.redirect_uris)?;

    ensure_user_pool_exists(db, input.user_pool_id).await?;

    let existing = application::Entity::find()
        .filter(application::Column::Code.eq(code.clone()))
        .one(db)
        .await
        .context("failed to check existing application code")?;
    if existing.is_some() {
        bail!("application code already exists: {code}");
    }

    let client_id = format!("app_{}", &Uuid::new_v4().simple().to_string()[..16]);
    let plain_secret = if application_type == "confidential" {
        Some(generate_secret())
    } else {
        None
    };
    let secret_hash = plain_secret.as_deref().map(hash_password).transpose()?;

    let txn = db
        .begin()
        .await
        .context("failed to start transaction for creating application")?;

    let application = application::ActiveModel {
        code: Set(code),
        name: Set(name),
        description: Set(description),
        application_type: Set(application_type),
        client_id: Set(client_id),
        client_secret_hash: Set(secret_hash),
        user_pool_id: Set(input.user_pool_id),
        homepage_url: Set(homepage_url),
        secret_rotated_at: Set(Some(Utc::now())),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .context("failed to insert application")?;

    insert_redirect_uris(&txn, application.id, redirect_uris).await?;
    txn.commit()
        .await
        .context("failed to commit application transaction")?;

    Ok(CreateApplicationResult {
        application,
        client_secret: plain_secret,
    })
}

pub async fn get_application_detail(
    db: &DatabaseConnection,
    application_id: Uuid,
) -> anyhow::Result<ApplicationDetail> {
    let application = get_application(db, application_id).await?;
    let user_pool = user_pool::Entity::find_by_id(application.user_pool_id)
        .one(db)
        .await
        .context("failed to load application user pool")?
        .ok_or_else(|| anyhow::anyhow!("bound user pool not found"))?;
    let redirect_uris = application_redirect_uri::Entity::find()
        .filter(application_redirect_uri::Column::ApplicationId.eq(application_id))
        .order_by_desc(application_redirect_uri::Column::IsPrimary)
        .order_by_asc(application_redirect_uri::Column::CreatedAt)
        .all(db)
        .await
        .context("failed to list application redirect uris")?;

    Ok(ApplicationDetail {
        application,
        user_pool,
        redirect_uris,
    })
}

pub async fn get_application(
    db: &DatabaseConnection,
    application_id: Uuid,
) -> anyhow::Result<application::Model> {
    application::Entity::find_by_id(application_id)
        .one(db)
        .await
        .context("failed to get application by id")?
        .ok_or_else(|| anyhow::anyhow!("application not found"))
}

pub async fn update_application(
    db: &DatabaseConnection,
    application_id: Uuid,
    input: UpdateApplicationInput,
) -> anyhow::Result<application::Model> {
    let model = get_application(db, application_id).await?;
    let mut active_model = model.into_active_model();

    if let Some(name) = input.name {
        active_model.name = Set(normalize_name(&name, "application name")?);
    }
    if let Some(description) = input.description {
        active_model.description = Set(normalize_optional_text(description, 255));
    }
    if let Some(homepage_url) = input.homepage_url {
        active_model.homepage_url = Set(normalize_optional_url(homepage_url)?);
    }

    active_model.updated_at = Set(Utc::now());
    active_model
        .update(db)
        .await
        .context("failed to update application")
}

pub async fn set_application_active(
    db: &DatabaseConnection,
    application_id: Uuid,
    is_active: bool,
) -> anyhow::Result<application::Model> {
    let model = get_application(db, application_id).await?;
    let mut active_model = model.into_active_model();
    active_model.is_active = Set(is_active);
    active_model.updated_at = Set(Utc::now());
    active_model
        .update(db)
        .await
        .context("failed to update application active state")
}

pub async fn delete_application(
    db: &DatabaseConnection,
    application_id: Uuid,
) -> anyhow::Result<()> {
    let model = get_application(db, application_id).await?;
    application::Entity::delete(model.into_active_model())
        .exec(db)
        .await
        .context("failed to delete application")?;
    Ok(())
}

pub async fn reset_application_secret(
    db: &DatabaseConnection,
    actor_admin_id: Uuid,
    application_id: Uuid,
    current_admin_password: &str,
) -> anyhow::Result<ResetApplicationSecretResult> {
    let actor = get_admin_user(db, actor_admin_id).await?;
    if !verify_password(current_admin_password.trim(), &actor.password_hash)? {
        bail!("current admin password is incorrect");
    }

    let application = get_application(db, application_id).await?;
    if application.application_type != "confidential" {
        bail!("application secret can only be reset for confidential applications");
    }

    let client_secret = generate_secret();
    let mut active_model = application.into_active_model();
    active_model.client_secret_hash = Set(Some(hash_password(&client_secret)?));
    active_model.secret_rotated_at = Set(Some(Utc::now()));
    active_model.updated_at = Set(Utc::now());

    let application = active_model
        .update(db)
        .await
        .context("failed to reset application secret")?;

    Ok(ResetApplicationSecretResult {
        application,
        client_secret,
    })
}

pub async fn list_redirect_uris(
    db: &DatabaseConnection,
    application_id: Uuid,
) -> anyhow::Result<Vec<application_redirect_uri::Model>> {
    get_application(db, application_id).await?;
    application_redirect_uri::Entity::find()
        .filter(application_redirect_uri::Column::ApplicationId.eq(application_id))
        .order_by_desc(application_redirect_uri::Column::IsPrimary)
        .order_by_asc(application_redirect_uri::Column::CreatedAt)
        .all(db)
        .await
        .context("failed to list redirect uris")
}

pub async fn create_redirect_uri(
    db: &DatabaseConnection,
    application_id: Uuid,
    input: CreateRedirectUriInput,
) -> anyhow::Result<application_redirect_uri::Model> {
    get_application(db, application_id).await?;
    let redirect_uri = normalize_redirect_uri(&input.redirect_uri)?;

    let existing = application_redirect_uri::Entity::find()
        .filter(application_redirect_uri::Column::ApplicationId.eq(application_id))
        .filter(application_redirect_uri::Column::RedirectUri.eq(redirect_uri.clone()))
        .one(db)
        .await
        .context("failed to check existing redirect uri")?;
    if existing.is_some() {
        bail!("redirect uri already exists for application");
    }

    let existing_count = application_redirect_uri::Entity::find()
        .filter(application_redirect_uri::Column::ApplicationId.eq(application_id))
        .count(db)
        .await
        .context("failed to count existing redirect uris")?;
    let is_primary = input.is_primary || existing_count == 0;

    let txn = db
        .begin()
        .await
        .context("failed to start transaction for creating redirect uri")?;
    if is_primary {
        clear_primary_redirect_uri(&txn, application_id).await?;
    }

    let redirect_uri = application_redirect_uri::ActiveModel {
        application_id: Set(application_id),
        redirect_uri: Set(redirect_uri),
        is_primary: Set(is_primary),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .context("failed to insert redirect uri")?;
    txn.commit()
        .await
        .context("failed to commit redirect uri transaction")?;

    Ok(redirect_uri)
}

pub async fn update_redirect_uri(
    db: &DatabaseConnection,
    application_id: Uuid,
    redirect_uri_id: Uuid,
    input: UpdateRedirectUriInput,
) -> anyhow::Result<application_redirect_uri::Model> {
    let redirect_uri = get_redirect_uri(db, application_id, redirect_uri_id).await?;
    let total = application_redirect_uri::Entity::find()
        .filter(application_redirect_uri::Column::ApplicationId.eq(application_id))
        .count(db)
        .await
        .context("failed to count redirect uris")?;

    if !input.is_primary && redirect_uri.is_primary && total == 1 {
        bail!("application must keep one primary redirect uri");
    }

    let txn = db
        .begin()
        .await
        .context("failed to start transaction for updating redirect uri")?;
    if input.is_primary {
        clear_primary_redirect_uri(&txn, application_id).await?;
    }

    let mut active_model = redirect_uri.into_active_model();
    active_model.is_primary = Set(input.is_primary);
    let redirect_uri = active_model
        .update(&txn)
        .await
        .context("failed to update redirect uri")?;
    txn.commit()
        .await
        .context("failed to commit redirect uri update")?;

    Ok(redirect_uri)
}

pub async fn delete_redirect_uri(
    db: &DatabaseConnection,
    application_id: Uuid,
    redirect_uri_id: Uuid,
) -> anyhow::Result<()> {
    let redirect_uri = get_redirect_uri(db, application_id, redirect_uri_id).await?;
    let total = application_redirect_uri::Entity::find()
        .filter(application_redirect_uri::Column::ApplicationId.eq(application_id))
        .count(db)
        .await
        .context("failed to count redirect uris before delete")?;
    if total <= 1 {
        bail!("application must keep at least one redirect uri");
    }

    application_redirect_uri::Entity::delete(redirect_uri.into_active_model())
        .exec(db)
        .await
        .context("failed to delete redirect uri")?;
    Ok(())
}

async fn insert_redirect_uris(
    txn: &DatabaseTransaction,
    application_id: Uuid,
    redirect_uris: Vec<CreateRedirectUriInput>,
) -> anyhow::Result<()> {
    for redirect_uri in redirect_uris {
        application_redirect_uri::ActiveModel {
            application_id: Set(application_id),
            redirect_uri: Set(redirect_uri.redirect_uri),
            is_primary: Set(redirect_uri.is_primary),
            ..Default::default()
        }
        .insert(txn)
        .await
        .context("failed to insert application redirect uri")?;
    }
    Ok(())
}

async fn clear_primary_redirect_uri(
    txn: &DatabaseTransaction,
    application_id: Uuid,
) -> anyhow::Result<()> {
    let redirect_uris = application_redirect_uri::Entity::find()
        .filter(application_redirect_uri::Column::ApplicationId.eq(application_id))
        .all(txn)
        .await
        .context("failed to load redirect uris for primary reset")?;

    for redirect_uri in redirect_uris {
        if redirect_uri.is_primary {
            let mut active_model = redirect_uri.into_active_model();
            active_model.is_primary = Set(false);
            active_model
                .update(txn)
                .await
                .context("failed to clear existing primary redirect uri")?;
        }
    }
    Ok(())
}

async fn get_redirect_uri(
    db: &DatabaseConnection,
    application_id: Uuid,
    redirect_uri_id: Uuid,
) -> anyhow::Result<application_redirect_uri::Model> {
    application_redirect_uri::Entity::find_by_id(redirect_uri_id)
        .filter(application_redirect_uri::Column::ApplicationId.eq(application_id))
        .one(db)
        .await
        .context("failed to get redirect uri by id")?
        .ok_or_else(|| anyhow::anyhow!("redirect uri not found"))
}

async fn ensure_user_pool_exists(
    db: &DatabaseConnection,
    user_pool_id: Uuid,
) -> anyhow::Result<user_pool::Model> {
    user_pool::Entity::find_by_id(user_pool_id)
        .one(db)
        .await
        .context("failed to get user pool by id for application")?
        .ok_or_else(|| anyhow::anyhow!("user pool not found"))
}

fn normalize_code(input: &str) -> anyhow::Result<String> {
    let code = input.trim().to_ascii_lowercase();
    if code.len() < 2 || code.len() > 64 {
        bail!("code length must be between 2 and 64 characters");
    }
    if !code
        .chars()
        .enumerate()
        .all(|(idx, ch)| ch.is_ascii_lowercase() || ch.is_ascii_digit() || (idx > 0 && ch == '-'))
    {
        bail!("code may only contain lowercase letters, numbers, and `-`");
    }
    Ok(code)
}

fn normalize_name(input: &str, label: &str) -> anyhow::Result<String> {
    let value = input.trim();
    if value.is_empty() {
        bail!("{label} is required");
    }
    if value.len() > 64 {
        bail!("{label} must be at most 64 characters");
    }
    Ok(value.to_string())
}

fn normalize_optional_text(input: Option<String>, max_len: usize) -> Option<String> {
    input.and_then(|value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.chars().take(max_len).collect())
        }
    })
}

fn normalize_application_type(input: &str) -> anyhow::Result<String> {
    let application_type = input.trim().to_ascii_lowercase();
    match application_type.as_str() {
        "confidential" | "public" => Ok(application_type),
        _ => bail!("unsupported application type"),
    }
}

fn normalize_optional_url(input: Option<String>) -> anyhow::Result<Option<String>> {
    let Some(value) = input else {
        return Ok(None);
    };
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    if !(trimmed.starts_with("http://") || trimmed.starts_with("https://")) {
        bail!("url must start with http:// or https://");
    }
    Ok(Some(trimmed.to_string()))
}

fn normalize_redirect_uris(
    inputs: Vec<CreateRedirectUriInput>,
) -> anyhow::Result<Vec<CreateRedirectUriInput>> {
    if inputs.is_empty() {
        bail!("at least one redirect uri is required");
    }

    let mut normalized = Vec::with_capacity(inputs.len());
    let mut seen = std::collections::BTreeSet::new();
    let mut has_primary = false;
    for (idx, item) in inputs.into_iter().enumerate() {
        let redirect_uri = normalize_redirect_uri(&item.redirect_uri)?;
        if !seen.insert(redirect_uri.clone()) {
            bail!("redirect uris must be unique");
        }
        let is_primary = item.is_primary || idx == 0 && !has_primary;
        has_primary |= is_primary;
        normalized.push(CreateRedirectUriInput {
            redirect_uri,
            is_primary,
        });
    }

    if !has_primary {
        if let Some(first) = normalized.first_mut() {
            first.is_primary = true;
        }
    }
    Ok(normalized)
}

fn normalize_redirect_uri(input: &str) -> anyhow::Result<String> {
    let value = input.trim();
    if value.is_empty() {
        bail!("redirect uri is required");
    }
    if !(value.starts_with("http://") || value.starts_with("https://")) {
        bail!("redirect uri must start with http:// or https://");
    }
    Ok(value.to_string())
}

fn generate_secret() -> String {
    format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple())
}
