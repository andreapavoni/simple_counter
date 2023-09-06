use sea_orm::*;

use crate::{
    entity::counters,
    error::DbError,
    migrations::{Migrator, MigratorTrait},
};

pub async fn run_migrations(db: &DbConn) -> Result<(), DbError> {
    let _ = Migrator::up(db, None).await?;
    Ok(())
}

pub async fn insert_counter(
    db: &DbConn,
    model: counters::Model,
) -> Result<counters::Model, DbError> {
    let counter = counters::ActiveModel {
        id: Set(model.id.to_owned()),
        name: Set(model.name.to_owned()),
        value: Set(model.value.to_owned()),
        ..Default::default()
    }
    .insert(db)
    .await?;

    Ok(counter)
}

pub async fn list_counters(db: &DbConn) -> Result<Vec<counters::Model>, DbError> {
    let counters = counters::Entity::find().all(db).await?;

    Ok(counters)
}

pub async fn update_counter_value(
    db: &DbConn,
    id: String,
    value: i32,
) -> Result<counters::Model, DbError> {
    if let Some(mut counter) = counters::Entity::find_by_id(id).one(db).await? {
        let new_value = counter.clone().value + value;

        let mut model: counters::ActiveModel = counter.clone().into();
        model.value = sea_orm::Set(new_value);
        model.update(db).await?;

        counter.value = new_value;

        return Ok(counter);
    }

    Err(DbError::NotFound)
}
