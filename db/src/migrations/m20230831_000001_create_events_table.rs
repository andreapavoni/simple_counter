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
                        ColumnDef::new(Events::Payload)
                            .json()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Events::AggregateId).uuid().not_null())
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
                    .name("idx_events_aggregate_id")
                    .table(Events::Table)
                    .col(Events::AggregateId)
                    .to_owned(),
            )
            .await?;


        manager
            .create_index(
                Index::create()
                    .name("idx_events_name")
                    .table(Events::Table)
                    .col(Events::Name)
                    .to_owned(),
            )
            .await

    }

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
    Payload,
    AggregateId,
    Timestamp,
}
