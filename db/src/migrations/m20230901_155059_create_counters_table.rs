use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Counters::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Counters::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Counters::Name).string().not_null())
                    .col(
                        ColumnDef::new(Counters::Value)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Counters::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Counters {
    Table,
    Id,
    Name,
    Value,
}
