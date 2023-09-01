use std::{env, net::SocketAddr, str::FromStr};

use anyhow::Context;
use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use migration::{Migrator, MigratorTrait};
use service::sea_orm::{Database, DatabaseConnection};

use crate::handlers::*;

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

        let assets_path_ref = std::env::current_dir().unwrap();
        let assets_path = assets_path_ref.to_str().unwrap();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let host = env::var("HOST").expect("HOST is not set");
        let port = env::var("PORT").expect("PORT is not set");

        let server_url = format!("{host}:{port}");
        let addr = SocketAddr::from_str(&server_url).unwrap();

        let conn = Database::connect(db_url)
            .await
            .expect("Database connection failed");
        Migrator::up(&conn, None).await.unwrap();

        let state = AppState { conn };

        info!("initializing router...");
        let router = Router::new()
            .route("/", get(home))
            .route("/dashboard", get(dashboard))
            .nest_service(
                "/assets",
                ServeDir::new(format!("{}/web/assets", assets_path)),
            )
            .with_state(state);

        info!("router initialized, now listening on port {}", port);
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .context("error while starting server")?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}
