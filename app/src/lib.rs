pub mod domain;
mod shims;

use std::env;
use tracing::metadata::LevelFilter;
use tracing::Level as LogLevel;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use domain::models;
use kountr_db::{error::DbError, repository, Database, DbConn};

#[derive(Clone)]
pub struct AppState {
    pub db: DbConn,
}

impl AppState {
    pub fn new(db: DbConn) -> AppState {
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

pub async fn init_app(opts: &AppOptions) -> AppState {
    init_app_tracing(&opts);

    let db = Database::connect(&opts.db_url)
        .await
        .expect("Database connection failed");

    repository::run_migrations(&db)
        .await
        .expect("Database migrations failed");

    AppState::new(db.clone())
}

fn init_app_tracing(opts: &AppOptions) {
    let debug_filter = match opts.env {
        AppEnv::Prod => LevelFilter::OFF,
        _ => LevelFilter::DEBUG,
    };

    let tracing_filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", opts.log_level)
        .with_target("sea_orm_migration::migrator", debug_filter)
        .with_target("sqlx::query", debug_filter)
        .with_default(opts.log_level);

    let tracing_layer = tracing_subscriber::fmt::layer().compact();

    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(tracing_filter)
        .init();
}

pub async fn add_counter(db: &DbConn, data: models::Counter) -> Result<models::Counter, DbError> {
    let new_counter = repository::insert_counter(db, data.into()).await?;

    Ok(new_counter.into())
}

pub async fn list_all_counters(db: &DbConn) -> Result<Vec<models::Counter>, DbError> {
    let counters = repository::list_counters(db).await?;

    Ok(counters.into_iter().map(Into::into).collect())
}


pub async fn find_counter(db: &DbConn, id: String) -> Result<models::Counter, DbError> {
    let counter = repository::find_counter_by_id(db, id).await?;
    Ok(counter.into())
}


pub async fn update_counter(db: &DbConn, counter: models::Counter) -> Result<models::Counter, DbError> {
    let counter = repository::update_counter(db, counter.into()).await?;
    Ok(counter.into())
}


pub async fn delete_counter(db: &DbConn, id: String) -> Result<(), DbError> {
    let _ = repository::delete_counter(db, id).await?;
    Ok(())
}

pub async fn increment_counter(db: &DbConn, id: String) -> Result<models::Counter, DbError> {
    let counter = repository::update_counter_value(db, id, 1).await?;
    Ok(counter.into())
}

pub async fn decrement_counter(db: &DbConn, id: String) -> Result<models::Counter, DbError> {
    let counter = repository::update_counter_value(db, id, -1).await?;
    Ok(counter.into())
}
