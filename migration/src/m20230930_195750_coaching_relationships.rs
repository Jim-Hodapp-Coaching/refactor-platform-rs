use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CoachingRelationships::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CoachingRelationships::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CoachingRelationships::CoacheeId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CoachingRelationships::CoachId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CoachingRelationships::OrganizationId)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CoachingRelationships::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CoachingRelationships {
    Table,
    Id,
    CoacheeId,
    CoachId,
    OrganizationId,
}
