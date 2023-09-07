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
use tracing::info;

use kountr_app::{init_app, AppOptions, AppState};

use crate::handlers::*;

pub struct Server;

impl Server {
    pub async fn start(opts: &AppOptions) -> anyhow::Result<()> {
        let state = init_app(&opts).await;

        let router = init_router(&opts, &state);
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

fn init_router(opts: &AppOptions, state: &AppState) -> Router {
    let assets_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");

    let http_tracing_layer = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(opts.log_level))
        .on_response(trace::DefaultOnResponse::new().level(opts.log_level));

    Router::new()
        .route("/", get(home))
        .route("/dashboard", get(dashboard))
        .route("/counters", get(list_counters).post(add_counter))
        .route("/counters/new", get(new_counter))
        .route("/counters/:id/edit", get(edit_counter))
        .route("/counters/:id/up", put(increment_counter))
        .route("/counters/:id/down", put(decrement_counter))
        .route("/counters/:id", put(update_counter).delete(delete_counter))
        .nest_service("/assets", ServeDir::new(assets_path))
        .with_state(state.clone())
        .layer(http_tracing_layer)
}
