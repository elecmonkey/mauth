use anyhow::{Context, bail};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde_json::Value;
use uuid::Uuid;

use crate::entity::{application, user_pool, user_pool_profile_field};

pub struct CreateUserPoolInput {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub allow_self_signup: bool,
}

pub struct UpdateUserPoolInput {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub allow_self_signup: Option<bool>,
}

pub struct ListUserPoolsInput {
    pub keyword: Option<String>,
    pub is_active: Option<bool>,
    pub page: u64,
    pub page_size: u64,
}

pub struct ListUserPoolsItem {
    pub user_pool: user_pool::Model,
    pub application_count: u64,
}

pub struct ListUserPoolsResult {
    pub items: Vec<ListUserPoolsItem>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}

pub struct UserPoolDetail {
    pub user_pool: user_pool::Model,
    pub profile_fields: Vec<user_pool_profile_field::Model>,
    pub applications: Vec<application::Model>,
}

pub struct CreateProfileFieldInput {
    pub field_key: String,
    pub field_name: String,
    pub field_type: String,
    pub is_required: bool,
    pub is_unique: bool,
    pub is_searchable: bool,
    pub sort_order: i32,
    pub options: Option<Value>,
}

pub struct UpdateProfileFieldInput {
    pub field_name: Option<String>,
    pub is_required: Option<bool>,
    pub is_searchable: Option<bool>,
    pub sort_order: Option<i32>,
    pub options: Option<Option<Value>>,
}

pub async fn list_user_pools(
    db: &DatabaseConnection,
    input: ListUserPoolsInput,
) -> anyhow::Result<ListUserPoolsResult> {
    let keyword = input
        .keyword
        .as_deref()
        .map(str::trim)
        .filter(|it| !it.is_empty());
    let page = input.page.max(1);
    let page_size = input.page_size.clamp(1, 100);

    let mut query = user_pool::Entity::find().order_by_desc(user_pool::Column::CreatedAt);

    if let Some(keyword) = keyword {
        query = query.filter(
            Condition::any()
                .add(user_pool::Column::Code.contains(keyword))
                .add(user_pool::Column::Name.contains(keyword)),
        );
    }

    if let Some(is_active) = input.is_active {
        query = query.filter(user_pool::Column::IsActive.eq(is_active));
    }

    let paginator = query.paginate(db, page_size);
    let total = paginator
        .num_items()
        .await
        .context("failed to count user pools")?;
    let models = paginator
        .fetch_page(page - 1)
        .await
        .context("failed to list user pools")?;

    let mut items = Vec::with_capacity(models.len());
    for model in models {
        let application_count = application::Entity::find()
            .filter(application::Column::UserPoolId.eq(model.id))
            .count(db)
            .await
            .context("failed to count applications for user pool")?;
        items.push(ListUserPoolsItem {
            user_pool: model,
            application_count,
        });
    }

    Ok(ListUserPoolsResult {
        items,
        total,
        page,
        page_size,
    })
}

pub async fn create_user_pool(
    db: &DatabaseConnection,
    input: CreateUserPoolInput,
) -> anyhow::Result<user_pool::Model> {
    let code = normalize_code(&input.code)?;
    let name = normalize_name(&input.name, "user pool name")?;
    let description = normalize_optional_text(input.description, 255);

    let existing = user_pool::Entity::find()
        .filter(user_pool::Column::Code.eq(code.clone()))
        .one(db)
        .await
        .context("failed to check existing user pool code")?;
    if existing.is_some() {
        bail!("user pool code already exists: {code}");
    }

    user_pool::ActiveModel {
        code: Set(code),
        name: Set(name),
        description: Set(description),
        allow_self_signup: Set(input.allow_self_signup),
        ..Default::default()
    }
    .insert(db)
    .await
    .context("failed to insert user pool")
}

pub async fn get_user_pool_detail(
    db: &DatabaseConnection,
    user_pool_id: Uuid,
) -> anyhow::Result<UserPoolDetail> {
    let user_pool = get_user_pool(db, user_pool_id).await?;
    let profile_fields = user_pool_profile_field::Entity::find()
        .filter(user_pool_profile_field::Column::UserPoolId.eq(user_pool_id))
        .order_by_asc(user_pool_profile_field::Column::SortOrder)
        .order_by_asc(user_pool_profile_field::Column::CreatedAt)
        .all(db)
        .await
        .context("failed to list user pool profile fields")?;
    let applications = application::Entity::find()
        .filter(application::Column::UserPoolId.eq(user_pool_id))
        .order_by_desc(application::Column::CreatedAt)
        .all(db)
        .await
        .context("failed to list user pool applications")?;

    Ok(UserPoolDetail {
        user_pool,
        profile_fields,
        applications,
    })
}

pub async fn get_user_pool(
    db: &DatabaseConnection,
    user_pool_id: Uuid,
) -> anyhow::Result<user_pool::Model> {
    user_pool::Entity::find_by_id(user_pool_id)
        .one(db)
        .await
        .context("failed to get user pool by id")?
        .ok_or_else(|| anyhow::anyhow!("user pool not found"))
}

pub async fn update_user_pool(
    db: &DatabaseConnection,
    user_pool_id: Uuid,
    input: UpdateUserPoolInput,
) -> anyhow::Result<user_pool::Model> {
    let model = get_user_pool(db, user_pool_id).await?;
    let mut active_model = model.into_active_model();

    if let Some(name) = input.name {
        active_model.name = Set(normalize_name(&name, "user pool name")?);
    }
    if let Some(description) = input.description {
        active_model.description = Set(normalize_optional_text(description, 255));
    }
    if let Some(allow_self_signup) = input.allow_self_signup {
        active_model.allow_self_signup = Set(allow_self_signup);
    }

    active_model.updated_at = Set(Utc::now());
    active_model
        .update(db)
        .await
        .context("failed to update user pool")
}

pub async fn set_user_pool_active(
    db: &DatabaseConnection,
    user_pool_id: Uuid,
    is_active: bool,
) -> anyhow::Result<user_pool::Model> {
    let model = get_user_pool(db, user_pool_id).await?;
    let mut active_model = model.into_active_model();
    active_model.is_active = Set(is_active);
    active_model.updated_at = Set(Utc::now());
    active_model
        .update(db)
        .await
        .context("failed to update user pool active state")
}

pub async fn delete_user_pool(db: &DatabaseConnection, user_pool_id: Uuid) -> anyhow::Result<()> {
    let model = get_user_pool(db, user_pool_id).await?;
    let application_count = application::Entity::find()
        .filter(application::Column::UserPoolId.eq(user_pool_id))
        .count(db)
        .await
        .context("failed to count applications before deleting user pool")?;
    if application_count > 0 {
        bail!("cannot delete user pool with bound applications");
    }

    user_pool::Entity::delete(model.into_active_model())
        .exec(db)
        .await
        .context("failed to delete user pool")?;
    Ok(())
}

pub async fn list_profile_fields(
    db: &DatabaseConnection,
    user_pool_id: Uuid,
) -> anyhow::Result<Vec<user_pool_profile_field::Model>> {
    get_user_pool(db, user_pool_id).await?;
    user_pool_profile_field::Entity::find()
        .filter(user_pool_profile_field::Column::UserPoolId.eq(user_pool_id))
        .order_by_asc(user_pool_profile_field::Column::SortOrder)
        .order_by_asc(user_pool_profile_field::Column::CreatedAt)
        .all(db)
        .await
        .context("failed to list profile fields")
}

pub async fn create_profile_field(
    db: &DatabaseConnection,
    user_pool_id: Uuid,
    input: CreateProfileFieldInput,
) -> anyhow::Result<user_pool_profile_field::Model> {
    get_user_pool(db, user_pool_id).await?;
    let field_key = normalize_field_key(&input.field_key)?;
    let field_name = normalize_name(&input.field_name, "profile field name")?;
    let field_type = normalize_field_type(&input.field_type)?;
    validate_field_options(&field_type, &input.options)?;

    let existing = user_pool_profile_field::Entity::find()
        .filter(user_pool_profile_field::Column::UserPoolId.eq(user_pool_id))
        .filter(user_pool_profile_field::Column::FieldKey.eq(field_key.clone()))
        .one(db)
        .await
        .context("failed to check existing profile field key")?;
    if existing.is_some() {
        bail!("profile field key already exists in user pool: {field_key}");
    }

    user_pool_profile_field::ActiveModel {
        user_pool_id: Set(user_pool_id),
        field_key: Set(field_key),
        field_name: Set(field_name),
        field_type: Set(field_type),
        is_required: Set(input.is_required),
        is_unique: Set(input.is_unique),
        is_searchable: Set(input.is_searchable),
        sort_order: Set(input.sort_order),
        options_json: Set(input.options),
        ..Default::default()
    }
    .insert(db)
    .await
    .context("failed to insert profile field")
}

pub async fn update_profile_field(
    db: &DatabaseConnection,
    user_pool_id: Uuid,
    field_id: Uuid,
    input: UpdateProfileFieldInput,
) -> anyhow::Result<user_pool_profile_field::Model> {
    let field = get_profile_field(db, user_pool_id, field_id).await?;
    let mut active_model = field.into_active_model();

    if let Some(field_name) = input.field_name {
        active_model.field_name = Set(normalize_name(&field_name, "profile field name")?);
    }
    if let Some(is_required) = input.is_required {
        active_model.is_required = Set(is_required);
    }
    if let Some(is_searchable) = input.is_searchable {
        active_model.is_searchable = Set(is_searchable);
    }
    if let Some(sort_order) = input.sort_order {
        active_model.sort_order = Set(sort_order);
    }
    if let Some(options) = input.options {
        validate_field_options(active_model.field_type.as_ref(), &options)?;
        active_model.options_json = Set(options);
    }

    active_model.updated_at = Set(Utc::now());
    active_model
        .update(db)
        .await
        .context("failed to update profile field")
}

pub async fn delete_profile_field(
    db: &DatabaseConnection,
    user_pool_id: Uuid,
    field_id: Uuid,
) -> anyhow::Result<()> {
    let field = get_profile_field(db, user_pool_id, field_id).await?;
    user_pool_profile_field::Entity::delete(field.into_active_model())
        .exec(db)
        .await
        .context("failed to delete profile field")?;
    Ok(())
}

async fn get_profile_field(
    db: &DatabaseConnection,
    user_pool_id: Uuid,
    field_id: Uuid,
) -> anyhow::Result<user_pool_profile_field::Model> {
    user_pool_profile_field::Entity::find_by_id(field_id)
        .filter(user_pool_profile_field::Column::UserPoolId.eq(user_pool_id))
        .one(db)
        .await
        .context("failed to get profile field by id")?
        .ok_or_else(|| anyhow::anyhow!("profile field not found"))
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

fn normalize_field_key(input: &str) -> anyhow::Result<String> {
    let key = input.trim().to_ascii_lowercase();
    if key.is_empty() {
        bail!("profile field key is required");
    }
    if key.len() > 64 {
        bail!("profile field key must be at most 64 characters");
    }
    if !key
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        bail!("profile field key may only contain lowercase letters, numbers, and `_`");
    }
    Ok(key)
}

fn normalize_field_type(input: &str) -> anyhow::Result<String> {
    let field_type = input.trim().to_ascii_lowercase();
    match field_type.as_str() {
        "string" | "text" | "number" | "boolean" | "date" | "datetime" | "select"
        | "multi_select" => Ok(field_type),
        _ => bail!("unsupported profile field type"),
    }
}

fn validate_field_options(field_type: &str, options: &Option<Value>) -> anyhow::Result<()> {
    let requires_options = matches!(field_type, "select" | "multi_select");
    if requires_options {
        let Some(Value::Array(items)) = options else {
            bail!("select field options are required");
        };
        if items.is_empty() {
            bail!("select field options cannot be empty");
        }
    }
    Ok(())
}
