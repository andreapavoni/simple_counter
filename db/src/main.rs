use sea_orm_migration::prelude::*;

use kountr_db::migrations::Migrator;

#[async_std::main]
async fn main() {
    cli::run_cli(Migrator).await;
}
