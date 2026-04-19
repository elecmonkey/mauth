use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "applications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(64))", unique)]
    pub code: String,
    #[sea_orm(column_type = "String(StringLen::N(64))")]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub description: Option<String>,
    #[sea_orm(column_type = "String(StringLen::N(32))")]
    pub application_type: String,
    #[sea_orm(column_type = "String(StringLen::N(64))", unique)]
    pub client_id: String,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub client_secret_hash: Option<String>,
    pub user_pool_id: Uuid,
    #[sea_orm(column_type = "Text", nullable)]
    pub homepage_url: Option<String>,
    #[sea_orm(default_value = true)]
    pub is_active: bool,
    pub secret_rotated_at: Option<DateTimeUtc>,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user_pool::Entity",
        from = "Column::UserPoolId",
        to = "super::user_pool::Column::Id",
        on_update = "NoAction",
        on_delete = "Restrict"
    )]
    UserPool,
    #[sea_orm(has_many = "super::application_redirect_uri::Entity")]
    RedirectUri,
}

impl Related<super::user_pool::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserPool.def()
    }
}

impl Related<super::application_redirect_uri::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RedirectUri.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let now = Utc::now();

        Self {
            id: Set(Uuid::new_v4()),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..ActiveModelTrait::default()
        }
    }
}
