use axum::{
    extract::{Form, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};

use kountr_app::domain::models::Counter;
use kountr_app::{AppState, add_counter, list_all_counters};
use serde::{Deserialize, Serialize};

use crate::views::*;

// Home
pub async fn home(_state: State<AppState>) -> impl IntoResponse {
    HtmlView(HomeView {})
}

// Dashboard
pub async fn dashboard() -> impl IntoResponse {
    HtmlView(DashboardView {})
}

// Counters
pub async fn list_counters(state: State<AppState>) -> impl IntoResponse {
    let counters = list_all_counters(&state.db).await.unwrap();

    HtmlView(ListCountersView { counters })
}

pub async fn new_counter() -> impl IntoResponse {
    HtmlView(NewCounterView {})
}

// Add Counter

#[derive(Serialize, Deserialize)]
pub struct NewCounterParams {
    name: String,
    value: i32,
}

impl Into<Counter> for NewCounterParams {
    fn into(self) -> Counter {
        Counter::new(self.name, self.value)
    }
}

pub async fn add_counter_handler(
    state: State<AppState>,
    Form(form): Form<NewCounterParams>,
) -> impl IntoResponse {
    let mut header = HeaderMap::new();
    header.insert(header::LOCATION, HeaderValue::from_static("/counters"));

    add_counter(&state.db, form.into())
        .await
        .expect("Cannot create counter");

    (StatusCode::SEE_OTHER, header)
    // HtmlView(ListCountersView {})
}
