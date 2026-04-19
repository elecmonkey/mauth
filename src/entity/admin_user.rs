use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "admin_users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(320))", unique)]
    pub email: String,
    #[sea_orm(column_type = "String(StringLen::N(64))")]
    pub nickname: String,
    #[sea_orm(column_type = "String(StringLen::N(255))")]
    pub password_hash: String,
    #[sea_orm(default_value = false)]
    pub totp_enabled: bool,
    #[sea_orm(column_type = "Text", nullable)]
    pub totp_secret_encrypted: Option<String>,
    #[sea_orm(default_value = true)]
    pub is_active: bool,
    pub last_login_at: Option<DateTimeUtc>,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub password_changed_at: DateTimeUtc,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let now = Utc::now();

        Self {
            id: Set(Uuid::new_v4()),
            totp_enabled: Set(false),
            is_active: Set(true),
            password_changed_at: Set(now),
            created_at: Set(now),
            updated_at: Set(now),
            ..ActiveModelTrait::default()
        }
    }
}
