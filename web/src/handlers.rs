use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use entity::events;
use service::{
    Mutation as MutationCore, Query as QueryCore,
};

use crate::AppState;


struct HtmlTemplate<T>(T);

/// Convert Askama HTML templates into valid HTML for Axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

pub async fn home(state: State<AppState>) -> impl IntoResponse {
    let events = QueryCore::list_all_events(&state.conn)
        .await
        .expect("Cannot find posts in page");

    let template = HomeTemplate { events };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomeTemplate {
    events: Vec<events::Model>,
}

pub async fn dashboard() -> impl IntoResponse {
    let template = DashboardTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "pages/dashboard.html")]
struct DashboardTemplate;


