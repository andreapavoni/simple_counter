use mini_cqrs::Repository as CqrsRepository;
use sea_orm::*;

use crate::{
    entity::counters,
    error::DbError,
    migrations::{Migrator, MigratorTrait},
};

#[derive(Clone)]
pub struct Repository {
    pub db: DbConn,
}

impl Repository {
    pub fn new(db: &DbConn) -> Self {
        Repository { db: db.clone() }
    }

    pub async fn run_migrations(&self) -> Result<(), DbError> {
        let _ = Migrator::up(&self.db, None).await?;
        Ok(())
    }

    pub async fn insert_counter(&self, model: counters::Model) -> Result<counters::Model, DbError> {
        let counter = counters::ActiveModel {
            id: Set(model.id.to_owned()),
            name: Set(model.name.to_owned()),
            value: Set(model.value.to_owned()),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;

        Ok(counter)
    }

    pub async fn list_counters(&self) -> Result<Vec<counters::Model>, DbError> {
        let counters = counters::Entity::find().all(&self.db).await?;

        Ok(counters)
    }

    pub async fn find_counter_by_id(&self, id: String) -> Result<counters::Model, DbError> {
        if let Some(counter) = counters::Entity::find_by_id(id).one(&self.db).await? {
            return Ok(counter);
        }

        Err(DbError::NotFound)
    }

    pub async fn update_counter_value(
        &self,
        id: String,
        value: i32,
    ) -> Result<counters::Model, DbError> {
        let txn = self.db.begin().await?;
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

    pub async fn insert_or_update_counter(
        &self,
        model: counters::Model,
    ) -> Result<counters::Model, DbError> {
        if let Ok(model) = self.update_counter(model.clone()).await {
            return Ok(model);
        } else {
            return self.insert_counter(model).await;
        }
    }

    pub async fn update_counter(
        &self,
        counter: counters::Model,
    ) -> Result<counters::Model, DbError> {
        let txn = self.db.begin().await?;

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

    pub async fn delete_counter(&self, id: String) -> Result<(), DbError> {
        let txn = self.db.begin().await?;

        if let Some(counter) = counters::Entity::find_by_id(id).one(&txn).await? {
            let model: counters::ActiveModel = counter.clone().into();
            model.delete(&txn).await?;

            txn.commit().await?;
            return Ok(());
        }

        txn.rollback().await?;
        Err(DbError::NotFound)
    }
}

impl CqrsRepository for Repository {}
