use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "application_redirect_uris")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub application_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub redirect_uri: String,
    #[sea_orm(default_value = false)]
    pub is_primary: bool,
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::application::Entity",
        from = "Column::ApplicationId",
        to = "super::application::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Application,
}

impl Related<super::application::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Application.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            is_primary: Set(false),
            created_at: Set(Utc::now()),
            ..ActiveModelTrait::default()
        }
    }
}
