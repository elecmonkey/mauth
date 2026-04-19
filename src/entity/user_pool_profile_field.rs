use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, Set};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_pool_profile_fields")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_pool_id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(64))")]
    pub field_key: String,
    #[sea_orm(column_type = "String(StringLen::N(64))")]
    pub field_name: String,
    #[sea_orm(column_type = "String(StringLen::N(32))")]
    pub field_type: String,
    #[sea_orm(default_value = false)]
    pub is_required: bool,
    #[sea_orm(default_value = false)]
    pub is_unique: bool,
    #[sea_orm(default_value = false)]
    pub is_searchable: bool,
    #[sea_orm(default_value = 0)]
    pub sort_order: i32,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub options_json: Option<Value>,
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
        on_delete = "Cascade"
    )]
    UserPool,
}

impl Related<super::user_pool::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserPool.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let now = Utc::now();

        Self {
            id: Set(Uuid::new_v4()),
            is_required: Set(false),
            is_unique: Set(false),
            is_searchable: Set(false),
            sort_order: Set(0),
            created_at: Set(now),
            updated_at: Set(now),
            ..ActiveModelTrait::default()
        }
    }
}
