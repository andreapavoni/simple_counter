mod cqrs;
pub mod domain;
mod shims;

use mini_cqrs::{Cqrs, SimpleDispatcher, QueriesRunner};
use std::env;
use tracing::metadata::LevelFilter;
use tracing::Level as LogLevel;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use cqrs::{
    AppQueries, CounterCommand, CounterEventConsumer, CounterState,
    GetCounterQuery, MainEventConsumers, ListCountersQuery,
};
use domain::models;
use kountr_db::{error::DbError, event_store::EventStore, repository::Repository, Database};


pub type AppCrqs = Cqrs<
    SimpleDispatcher<CounterState, EventStore, MainEventConsumers>,
    CounterState,
    EventStore,
    AppQueries,
>;

#[derive(Clone)]
pub struct AppState {
    pub repo: Repository,
    pub cqrs: AppCrqs,
}

impl AppState {
    pub fn new(
        repo: Repository,
        cqrs: AppCrqs,
    ) -> AppState {
        AppState { repo, cqrs }
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

    let repo = Repository::new(&db);

    let cqrs = init_cqrs(repo.clone());

    repo.run_migrations()
        .await
        .expect("Database migrations failed");

    AppState::new(repo.clone(), cqrs)
}

fn init_cqrs(repo: Repository) -> AppCrqs {
    let store = EventStore::new(repo.db.clone());

    let consumers = vec![MainEventConsumers::Counter(CounterEventConsumer::new(
        &repo.clone(),
    ))];

    let dispatcher: SimpleDispatcher<CounterState, EventStore, MainEventConsumers> =
        SimpleDispatcher::new(store, consumers);

    let queries = AppQueries {};

    Cqrs::new(dispatcher, queries)
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

pub async fn add_counter(
    app: &mut AppState,
    data: models::Counter,
) -> Result<models::Counter, DbError> {
    let aggregate_id = uuid::Uuid::new_v4().to_string();
    let cmd = CounterCommand::Create {
        name: data.name,
        value: data.value,
    };
    let id = app.cqrs.execute(aggregate_id.clone(), cmd).await?;
    let q = GetCounterQuery ::new(id.clone(), &app.repo.clone());
    let counter = app.cqrs.queries().run(q.clone()).await?.unwrap();

    Ok(counter)
}

pub async fn list_all_counters(app: &AppState) -> Result<Vec<models::Counter>, DbError> {
    let query = ListCountersQuery::new(&app.repo);
    let counters = app.cqrs.queries().run(query.clone()).await?;

    let counters = counters.into_iter().map(Into::into).collect();

    Ok(counters)
}

pub async fn find_counter(app: &AppState, id: String) -> Result<models::Counter, DbError> {
    let q = GetCounterQuery ::new(id.clone(), &app.repo.clone());
    let result = app.cqrs.queries().run(q.clone()).await?.unwrap();

    Ok(result.into())
}

pub async fn update_counter(
    app: &AppState,
    counter: models::Counter,
) -> Result<models::Counter, DbError> {
    let cmd = CounterCommand::Update {
        id: counter.id.clone(),
        name: counter.name,
        value: counter.value,
    };
    let id = app.clone().cqrs.execute(counter.id.clone(), cmd).await?;

    let counter = find_counter(app, id).await?;
    Ok(counter.into())
}

pub async fn delete_counter(app: &AppState, id: String) -> Result<(), DbError> {
    let cmd = CounterCommand::Delete { id: id.clone() };
    let _ = app.clone().cqrs.execute(id.clone(), cmd).await?;
    Ok(())
}

pub async fn increment_counter(app: &AppState, id: String) -> Result<models::Counter, DbError> {
    let cmd = CounterCommand::Increment {
        id: id.clone(),
        amount: 1,
    };
    let id = app.clone().cqrs.execute(id.clone(), cmd).await?;

    let counter = find_counter(app, id).await?;
    Ok(counter.into())
}

pub async fn decrement_counter(app: &AppState, id: String) -> Result<models::Counter, DbError> {
    let cmd = CounterCommand::Decrement {
        id: id.clone(),
        amount: -1,
    };
    let id = app.clone().cqrs.execute(id.clone(), cmd).await?;

    let counter = find_counter(app, id).await?;
    Ok(counter.into())
}
