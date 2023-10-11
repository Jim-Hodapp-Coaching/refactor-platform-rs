pub use sea_orm_migration::prelude::*;

mod m20230930_182906_create_organizations;
mod m20230930_195750_coaching_relationships;
mod m20230930_200501_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230930_182906_create_organizations::Migration),
            Box::new(m20230930_195750_coaching_relationships::Migration),
            Box::new(m20230930_200501_users::Migration),
        ]
    }
}
