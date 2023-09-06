use std::{env, net::SocketAddr, str::FromStr};

use anyhow::Context;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use kountr_app::AppState;

use crate::handlers::*;

pub struct Web;

impl Web {
    pub async fn start() -> anyhow::Result<()> {
        dotenvy::dotenv().ok();

        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "kountr=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();


        let assets_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let host = env::var("HOST").expect("HOST is not set");
        let port = env::var("PORT").expect("PORT is not set");

        let server_url = format!("{host}:{port}");
        let addr = SocketAddr::from_str(&server_url).unwrap();

        let state = AppState::new(db_url).await;

        info!("Initializing router...");
        let router = Router::new()
            .route("/", get(home))
            .route("/dashboard", get(dashboard))
            .route("/counters", get(list_counters).post(add_counter_handler))
            .route("/counters/new", get(new_counter))
            .nest_service("/assets", ServeDir::new(assets_path))
            .with_state(state);

        info!("Router initialized, now listening on {}", server_url);
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .context("Error while starting server")?;

        Ok(())
    }
}
