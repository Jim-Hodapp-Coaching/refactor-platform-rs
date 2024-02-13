use sea_orm_migration::prelude::*;
use std::fs::File;
use std::io::Read;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let stringify_err = |err| -> DbErr {
            let string = format!("Migration error: {:?}", err);
            DbErr::Migration(string)
        };
        let mut file =
            File::open("migration/src/refactor_platform_rs.sql").map_err(stringify_err)?;

        let mut sql = String::new();

        file.read_to_string(&mut sql).map_err(stringify_err)?;

        // format sql as valid sql statements
        sql = sql.replace(";", ";\n");

        db.execute_unprepared(&sql).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }
}
