// m20230103_000001_create_schedule_table.rs

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230103_000001_create_schedule_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Bakery table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Schedule::Table)
                    .col(
                        ColumnDef::new(Schedule::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Schedule::Name).string().not_null())
                    .col(ColumnDef::new(Schedule::Expression).string().not_null())
                    .col(ColumnDef::new(Schedule::Start).timestamp().not_null())
                    .col(ColumnDef::new(Schedule::Finish).timestamp().null())
                    .col(ColumnDef::new(Schedule::Process).timestamp().null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Bakery table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Schedule::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Schedule {
    Table,
    Id,
    Name,
    Expression,
    Start,
    Finish,
    Process,
}
