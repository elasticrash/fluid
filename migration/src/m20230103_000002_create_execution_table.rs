// m20230103_000002_create_execution_table.rs

use sea_orm_migration::prelude::*;

use super::m20230103_000001_create_schedule_table::Schedule;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230103_000002_create_execution_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Chef table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Execution::Table)
                    .col(
                        ColumnDef::new(Execution::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Execution::Name).string().not_null())
                    .col(ColumnDef::new(Execution::When).json())
                    .col(ColumnDef::new(Execution::Pending).integer().not_null())
                    .col(ColumnDef::new(Execution::ScheduleId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-execution-schedule_id")
                            .from(Execution::Table, Execution::ScheduleId)
                            .to(Schedule::Table, Schedule::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Execution::Table).to_owned())
            .await
    }
}

// For ease of access
#[derive(Iden)]
pub enum Execution {
    Table,
    Id,
    Name,
    When,
    Pending,
    ScheduleId,
}