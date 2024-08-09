use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, Deserialize, Serialize, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "status")]
pub enum Status {
    #[sea_orm(string_value = "InProgress")]
    InProgress,
    #[sea_orm(string_value = "Completed")]
    Completed,
    #[sea_orm(string_value = "WontDo")]
    WontDo,
}

impl std::default::Default for Status {
    fn default() -> Self {
        Self::InProgress
    }
}
