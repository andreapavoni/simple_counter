use std::{env, net::SocketAddr, str::FromStr};

use anyhow::Context;
use axum::{
    routing::{get, put},
    Router,
};
use tower_http::{
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::{info, metadata::LevelFilter};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::handlers::*;
use kountr_app::{AppEnv, AppOptions, AppState};

pub struct Server;

impl Server {
    pub async fn start(opts: &AppOptions) -> anyhow::Result<()> {
        init_tracing(&opts);

        let router = init_router(&opts).await;

        let server_url = format!("{}:{}", opts.host, opts.port);
        let addr = SocketAddr::from_str(&server_url).unwrap();

        info!("Starting server listening on http://{}", server_url);
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .context("Error while starting server")?;

        Ok(())
    }
}

fn init_tracing(opts: &AppOptions) {
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

async fn init_router(opts: &AppOptions) -> Router {
    let assets_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");

    let http_tracing_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(opts.log_level))
        .on_response(trace::DefaultOnResponse::new().level(opts.log_level));

    let state = AppState::new(&opts.db_url).await;

    Router::new()
        .route("/", get(home))
        .route("/dashboard", get(dashboard))
        .route("/counters", get(list_counters).post(add_counter))
        .route("/counters/new", get(new_counter))
        .route("/counters/:id/up", put(increment_counter))
        .route("/counters/:id/down", put(decrement_counter))
        .nest_service("/assets", ServeDir::new(assets_path))
        .with_state(state.clone())
        .layer(http_tracing_layer)
}
