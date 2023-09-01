// use std::{env, net::SocketAddr, str::FromStr};
//
// use anyhow::Context;
// use askama::Template;
// use axum::{
//     extract::{Query, State},
//     http::StatusCode,
//     response::{Html, IntoResponse, Response},
//     routing::get,
//     Router,
// };
// // use serde::{Deserialize, Serialize};
// use tower_http::services::ServeDir;
// use tracing::info;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// // use uuid::Uuid;
//
// use entity::events;
// use migration::{Migrator, MigratorTrait};
// use service::{
//     sea_orm::{Database, DatabaseConnection},
//     Mutation as MutationCore, Query as QueryCore,
// };
//
// #[tokio::main]
// pub async fn start() -> anyhow::Result<()> {
//     tracing_subscriber::registry()
//         .with(
//             tracing_subscriber::EnvFilter::try_from_default_env()
//                 .unwrap_or_else(|_| "kountr=debug".into()),
//         )
//         .with(tracing_subscriber::fmt::layer())
//         .init();
//
//     dotenvy::dotenv().ok();
//
//     let binding = std::env::current_dir().unwrap();
//     let assets_path = binding.to_str().unwrap();
//
//     let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
//     let host = env::var("HOST").expect("HOST is not set");
//     let port = env::var("PORT").expect("PORT is not set");
//
//     let server_url = format!("{host}:{port}");
//     let addr = SocketAddr::from_str(&server_url).unwrap();
//
//     let conn = Database::connect(db_url)
//         .await
//         .expect("Database connection failed");
//     Migrator::up(&conn, None).await.unwrap();
//
//     let state = AppState { conn };
//
//     info!("initializing router...");
//     let router = Router::new()
//         .route("/", get(home))
//         .route("/dashboard", get(dashboard))
//         .nest_service(
//             "/assets",
//             ServeDir::new(format!("{}/web/assets", assets_path)),
//         )
//         .with_state(state);
//
//     info!("router initialized, now listening on port {}", port);
//     axum::Server::bind(&addr)
//         .serve(router.into_make_service())
//         .await
//         .context("error while starting server")?;
//
//     Ok(())
// }
//
// #[derive(Clone)]
// struct AppState {
//     conn: DatabaseConnection,
// }

// #[derive(Deserialize)]
// struct ParamsUuid {
//     id: Option<Uuid>,
// }

// async fn home(state: State<AppState>) -> impl IntoResponse {
//     let events = QueryCore::list_all_events(&state.conn)
//         .await
//         .expect("Cannot find posts in page");
//
//     let template = HomeTemplate { events };
//     HtmlTemplate(template)
// }
//
// async fn dashboard() -> impl IntoResponse {
//     let template = DashboardTemplate {};
//     HtmlTemplate(template)
// }
//
// #[derive(Template)]
// #[template(path = "pages/dashboard.html")]
// struct DashboardTemplate;
//
// #[derive(Template)]
// #[template(path = "pages/home.html")]
// struct HomeTemplate {
//     events: Vec<events::Model>,
// }
//
// /// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
// struct HtmlTemplate<T>(T);
//
// /// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
// impl<T> IntoResponse for HtmlTemplate<T>
// where
//     T: Template,
// {
//     fn into_response(self) -> Response {
//         // Attempt to render the template with askama
//         match self.0.render() {
//             // If we're able to successfully parse and aggregate the template, serve it
//             Ok(html) => Html(html).into_response(),
//             // If we're not, return an error or some bit of fallback HTML
//             Err(err) => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 format!("Failed to render template. Error: {}", err),
//             )
//                 .into_response(),
//         }
//     }
// }


mod app;
mod handlers;

pub use app::*;
