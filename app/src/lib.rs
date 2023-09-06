pub mod domain;
mod shims;

use domain::models;
use kountr_db::{repository, Database, DbConn, error::DbError};

#[derive(Clone)]
pub struct AppState {
    pub db: DbConn,
}

impl AppState {
    pub async fn new(db_url: String) -> Self {
        let db = Database::connect(db_url)
            .await
            .expect("Database connection failed");

        repository::run_migrations(&db)
            .await
            .expect("Database migrations failed");

        AppState { db }
    }
}

pub async fn add_counter(db: &DbConn, data: models::Counter) -> Result<models::Counter, DbError> {
    let new_counter = repository::insert_counter(db, data.into()).await?;

    Ok(new_counter.into())
}

pub async fn list_all_counters(db: &DbConn) -> Result<Vec<models::Counter>, DbError> {
    let counters = repository::list_counters(db).await?;

    Ok(counters.into_iter().map(Into::into).collect())
}

pub async fn increment_counter(db: &DbConn, id: String) -> Result<models::Counter, DbError> {
    let counter = repository::update_counter_value(db, id, 1).await?;
    Ok(counter.into())
}

pub async fn decrement_counter(db: &DbConn, id: String) -> Result<models::Counter, DbError> {
    let counter = repository::update_counter_value(db, id, -1).await?;
    Ok(counter.into())
}
