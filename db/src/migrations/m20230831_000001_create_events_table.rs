use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Events::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Events::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Events::Name).string().not_null())
                    .col(
                        ColumnDef::new(Events::Value)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Events::CounterId).uuid().not_null())
                    .col(
                        ColumnDef::new(Events::Timestamp)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_events_counter_id")
                    .table(Events::Table)
                    .col(Events::CounterId)
                    .to_owned(),
            )
            .await
    }

    // CREATE TABLE IF NOT EXISTS "events" (
    //     "id" text(36) NOT NULL PRIMARY KEY,
    //     "name" text NOT NULL,
    //     "value" integer DEFAULT 0 NOT NULL,
    //     "counter_id" text(36) NOT NULL,
    //     "timestamp" text NOT NULL,
    //     CONSTRAINT "idx_events_counter_id" ("counter_id")
    // )

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Events::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Events {
    Table,
    Id,
    Name,
    Value,
    CounterId,
    Timestamp,
}
