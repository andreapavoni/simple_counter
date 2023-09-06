use axum::{
    extract::{Form, Path, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use kountr_app::domain::models::Counter;
use kountr_app::AppState;

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
    let counters = kountr_app::list_all_counters(&state.db).await.unwrap();

    HtmlView(ListCountersView {
        counters: counters
            .iter()
            .map(move |c| CounterView {
                counter: c.clone(),
                id: c.clone().id,
                name: c.clone().name,
                value: c.clone().value,
            })
            .collect::<Vec<CounterView>>(),
    })
}

// New Counter
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

pub async fn add_counter(
    state: State<AppState>,
    Form(form): Form<NewCounterParams>,
) -> impl IntoResponse {
    let mut header = HeaderMap::new();
    header.insert(header::LOCATION, HeaderValue::from_static("/counters"));

    kountr_app::add_counter(&state.db, form.into())
        .await
        .expect("Cannot create counter");

    (StatusCode::SEE_OTHER, header)
    // HtmlView(ListCountersView {})
}

// Increment/Decrement Counter
#[derive(Serialize, Deserialize)]
pub struct CounterIdParams {
    id: String,
}

pub async fn increment_counter(
    Path(id): Path<String>,
    state: State<AppState>,
) -> impl IntoResponse {
    let counter = kountr_app::increment_counter(&state.db, id)
        .await
        .expect("Cannot increment counter");

    HtmlView(CounterView {
        counter: counter.clone(),
        id: counter.id,
        name: counter.name,
        value: counter.value,
    })
}

pub async fn decrement_counter(
    state: State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let counter = kountr_app::decrement_counter(&state.db, id)
        .await
        .expect("Cannot increment counter");

    HtmlView(CounterView {
        counter: counter.clone(),
        id: counter.id,
        name: counter.name,
        value: counter.value,
    })
}
