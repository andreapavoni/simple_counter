use std::{env, net::SocketAddr, str::FromStr};

use anyhow::Context;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use migration::{Migrator, MigratorTrait};
use service::sea_orm::{Database, DatabaseConnection};

use crate::handlers::*;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub struct App;

impl App {
    pub async fn start() -> anyhow::Result<()> {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "kountr=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();

        dotenvy::dotenv().ok();

        let assets_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let host = env::var("HOST").expect("HOST is not set");
        let port = env::var("PORT").expect("PORT is not set");

        let server_url = format!("{host}:{port}");
        let addr = SocketAddr::from_str(&server_url).unwrap();

        let db = Database::connect(db_url)
            .await
            .expect("Database connection failed");
        Migrator::up(&db, None).await.unwrap();

        let state = AppState { db };

        info!("initializing router...");
        let router = Router::new()
            .route("/", get(home))
            .route("/dashboard", get(dashboard))
            .route("/counters", get(list_counters).post(add_counter))
            .route("/counters/new", get(new_counter))
            .nest_service("/assets", ServeDir::new(assets_path))
            .with_state(state);

        info!("router initialized, now listening on {}", server_url);
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .context("error while starting server")?;

        Ok(())
    }
}
