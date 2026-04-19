use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_pools")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "String(StringLen::N(64))", unique)]
    pub code: String,
    #[sea_orm(column_type = "String(StringLen::N(64))")]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::N(255))", nullable)]
    pub description: Option<String>,
    #[sea_orm(default_value = true)]
    pub is_active: bool,
    #[sea_orm(default_value = false)]
    pub allow_self_signup: bool,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_pool_profile_field::Entity")]
    ProfileField,
    #[sea_orm(has_many = "super::application::Entity")]
    Application,
}

impl Related<super::user_pool_profile_field::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ProfileField.def()
    }
}

impl Related<super::application::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Application.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        let now = Utc::now();

        Self {
            id: Set(Uuid::new_v4()),
            is_active: Set(true),
            allow_self_signup: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
            ..ActiveModelTrait::default()
        }
    }
}
