pub use sea_orm_migration::prelude::*;

mod m20230831_000001_create_events_table;
mod m20230901_155059_create_counters_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230831_000001_create_events_table::Migration),
            Box::new(m20230901_155059_create_counters_table::Migration),
        ]
    }
}
