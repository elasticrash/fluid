// migration/src/lib.rs

pub use sea_orm_migration::prelude::*;

// Add each migration file as a module
mod m20230103_000001_create_schedule_table;
mod m20230103_000002_create_execution_table;
mod m20230104_000003_create_callback_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Define the order of migrations.
            Box::new(m20230103_000001_create_schedule_table::Migration),
            Box::new(m20230103_000002_create_execution_table::Migration),
            Box::new(m20230104_000003_create_callback_table::Migration),
        ]
    }
}
