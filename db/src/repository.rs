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

pub async fn find_counter_by_id(db: &DbConn, id: String) -> Result<counters::Model, DbError> {
    if let Some(counter) = counters::Entity::find_by_id(id).one(db).await? {
        return Ok(counter);
    }

    Err(DbError::NotFound)
}

pub async fn update_counter_value(
    db: &DbConn,
    id: String,
    value: i32,
) -> Result<counters::Model, DbError> {
    let txn = db.begin().await?;
    if let Some(mut counter) = counters::Entity::find_by_id(id).one(&txn).await? {
        let new_value = counter.clone().value + value;

        let mut model: counters::ActiveModel = counter.clone().into();
        model.value = sea_orm::Set(new_value);
        model.update(&txn).await?;

        counter.value = new_value;

        txn.commit().await?;

        return Ok(counter);
    }

    Err(DbError::NotFound)
}

pub async fn update_counter(
    db: &DbConn,
    counter: counters::Model,
) -> Result<counters::Model, DbError> {
    let txn = db.begin().await?;

    let db_counter = counters::Entity::find_by_id(counter.clone().id)
        .one(&txn)
        .await?;
    if db_counter.is_none() {
        return Err(DbError::NotFound);
    }
    let mut db_counter = db_counter.unwrap();

    let mut model: counters::ActiveModel = db_counter.clone().into();
    model.name = Set(counter.name.to_owned());
    model.value = Set(counter.value.to_owned());

    db_counter.name = counter.name;
    db_counter.value = counter.value;

    model.save(&txn).await?;

    txn.commit().await?;
    Ok(db_counter)
}

pub async fn delete_counter(db: &DbConn, id: String) -> Result<(), DbError> {
    let txn = db.begin().await?;

    if let Some(counter) = counters::Entity::find_by_id(id).one(&txn).await? {
        let model: counters::ActiveModel = counter.clone().into();
        model.delete(&txn).await?;

        txn.commit().await?;
        return Ok(());
    }

    txn.rollback().await?;
    Err(DbError::NotFound)
}
