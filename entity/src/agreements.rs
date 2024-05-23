//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use crate::Id;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(schema_name = "refactor_platform", table_name = "agreements")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Id,
    #[sea_orm(unique)]
    pub coaching_session_id: Id,
    pub details: Option<String>,
    pub user_id: Id,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::coaching_sessions::Entity",
        from = "Column::CoachingSessionId",
        to = "super::coaching_sessions::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    CoachingSessions,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
}

impl Related<super::coaching_sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CoachingSessions.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
