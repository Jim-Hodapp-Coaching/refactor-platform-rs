//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(
    schema_name = "refactor_platform_rs",
    table_name = "coaching_relationships"
)]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub coachee_id: String,
    pub coach_id: String,
    pub organization_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Organization,
    Coach,
    Coachee,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Organization => Entity::belongs_to(super::organization::Entity)
                .from(Column::OrganizationId)
                .to(super::organization::Column::Id)
                .into(),
            Self::Coach => Entity::belongs_to(super::user::Entity)
                .from(Column::CoachId)
                .to(super::user::Column::Id)
                .into(),
            Self::Coachee => Entity::belongs_to(super::user::Entity)
                .from(Column::CoacheeId)
                .to(super::user::Column::Id)
                .into(),
        }
    }
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
