pub mod domain;
mod shims;

use std::env;
use tracing::Level as LogLevel;

use domain::models;
use kountr_db::{error::DbError, repository, Database, DbConn};

#[derive(Clone)]
pub struct AppState {
    pub db: DbConn,
}

impl AppState {
    pub async fn new(db_url: &str) -> Self {
        let db = Database::connect(db_url)
            .await
            .expect("Database connection failed");

        repository::run_migrations(&db)
            .await
            .expect("Database migrations failed");

        AppState { db }
    }
}

pub enum AppEnv {
    Dev,
    Prod,
}

pub struct AppOptions {
    pub db_url: String,
    pub host: String,
    pub port: u16,
    pub env: AppEnv,
    pub log_level: LogLevel,
}

impl AppOptions {
    pub fn new_from_envs() -> Self {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
        let port = env::var("PORT").unwrap_or("8000".to_string());
        let env_level = env::var("APP_ENV").unwrap_or("dev".to_string());

        let (app_env, log_level) = match env_level.as_str() {
            "prod" => (AppEnv::Prod, LogLevel::INFO),
            _ => (AppEnv::Dev, LogLevel::DEBUG),
        };

        Self {
            db_url,
            host,
            port: port.parse().unwrap(),
            env: app_env,
            log_level,
        }
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
