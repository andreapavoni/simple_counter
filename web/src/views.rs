use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use kountr_app::domain::models::Counter;

pub struct HtmlView<T>(pub T);

/// Convert Askama HTML templates into valid HTML for Axum to serve in the response.
impl<T> IntoResponse for HtmlView<T>
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

#[derive(Template)]
#[template(path = "views/pages/home.html")]
pub struct HomeView {
}

#[derive(Template)]
#[template(path = "views/pages/dashboard.html")]
pub struct DashboardView;

// Counters
#[derive(Template)]
#[template(path = "views/counters/list.html")]
pub struct ListCountersView {
    pub counters: Vec<Counter>,
}

#[derive(Template)]
#[template(path = "views/counters/new.html")]
pub struct NewCounterView;
