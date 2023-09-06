use axum::{
    extract::{Form, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};

use entity::counters;
use serde::{Deserialize, Serialize};
use service::{Mutation as MutationService, Query as QueryService};

use crate::views::*;
use crate::AppState;

// Home
pub async fn home(state: State<AppState>) -> impl IntoResponse {
    let events = QueryService::list_all_events(&state.db).await.unwrap();

    HtmlView(HomeView { events })
}

// Dashboard
pub async fn dashboard() -> impl IntoResponse {
    HtmlView(DashboardView {})
}

// Counters
pub async fn list_counters(state: State<AppState>) -> impl IntoResponse {
    let counters = QueryService::list_all_counters(&state.db).await.unwrap();

    HtmlView(ListCountersView { counters })
}

pub async fn new_counter() -> impl IntoResponse {
    HtmlView(NewCounterView {})
}

// Add Counter
//
#[derive(Serialize, Deserialize)]
pub struct NewCounterParams {
    name: String,
    value: i32,
}

impl Into<counters::Model> for NewCounterParams {
    fn into(self) -> counters::Model {
        counters::Model {
            name: self.name,
            value: self.value,
            ..Default::default()
        }
    }
}

pub async fn add_counter(
    state: State<AppState>,
    Form(form): Form<NewCounterParams>,
) -> impl IntoResponse {
    let mut header = HeaderMap::new();
    header.insert(header::LOCATION, HeaderValue::from_static("/counters"));

    MutationService::add_counter(&state.db, form.into())
        .await
        .expect("Cannot create post");

    (StatusCode::SEE_OTHER, header)
    // HtmlView(ListCountersView {})
}
