use crate::{
    entity::counters,
    migrations::{Migrator, MigratorTrait},
};
use sea_orm::*;

pub async fn run_migrations(db: &DbConn) -> Result<(), DbErr> {
    Migrator::up(db, None).await
}

pub async fn insert_counter(db: &DbConn, model: counters::Model) -> Result<counters::Model, DbErr> {
    counters::ActiveModel {
        id: Set(model.id.to_owned()),
        name: Set(model.name.to_owned()),
        value: Set(model.value.to_owned()),
        ..Default::default()
    }
    .insert(db)
    // .save(db) // for updates
    .await
}

pub async fn list_counters(db: &DbConn) -> Result<Vec<counters::Model>, DbErr> {
    counters::Entity::find().all(db).await
}
