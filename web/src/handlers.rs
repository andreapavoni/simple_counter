use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};

use kountr_app::domain::models::Counter;
use kountr_app::AppState;

use crate::views::*;

// ====================== PARAMS ==============================================
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

#[derive(Serialize, Deserialize)]
pub struct UpdateCounterParams {
    id: String,
    name: String,
    value: i32,
}

impl Into<Counter> for UpdateCounterParams {
    fn into(self) -> Counter {
        Counter::new_with_id(self.id, self.name, self.value)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CounterIdParams {
    id: String,
}

// ====================== HANDLERS ============================================

pub async fn home(_state: State<AppState>) -> impl IntoResponse {
    HtmlView(HomeView {})
}

pub async fn dashboard() -> impl IntoResponse {
    HtmlView(DashboardView {})
}

pub async fn list_counters(state: State<AppState>) -> impl IntoResponse {
    let counters = kountr_app::list_all_counters(&state).await.unwrap();

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

pub async fn new_counter() -> impl IntoResponse {
    HtmlView(NewCounterView {})
}

pub async fn add_counter(
    mut state: State<AppState>,
    Form(form): Form<NewCounterParams>,
) -> impl IntoResponse {
    kountr_app::add_counter(&mut state, form.into())
        .await
        .expect("Cannot create counter");

    Redirect::to("/counters")
}

pub async fn edit_counter(Path(id): Path<String>, state: State<AppState>) -> impl IntoResponse {
    let counter = kountr_app::find_counter(&state, id)
        .await
        .expect("Cannot find counter");

    HtmlView(EditCounterView {
        id: counter.id,
        name: counter.name,
        value: counter.value,
    })
}

pub async fn update_counter(
    state: State<AppState>,
    Form(form): Form<UpdateCounterParams>,
) -> impl IntoResponse {
    kountr_app::update_counter(&state, form.into())
        .await
        .expect("Cannot update counter");

    (
        StatusCode::SEE_OTHER,
        [("HX-Redirect", "/counters")],
        "updated",
    )
}

pub async fn delete_counter(Path(id): Path<String>, state: State<AppState>) -> impl IntoResponse {
    kountr_app::delete_counter(&state, id)
        .await
        .expect("Cannot delete counter");

    (
        StatusCode::SEE_OTHER,
        [("HX-Redirect", "/counters")],
        "deleted",
    )
}

pub async fn increment_counter(
    Path(id): Path<String>,
    state: State<AppState>,
) -> impl IntoResponse {
    let counter = kountr_app::increment_counter(&state, id)
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
    let counter = kountr_app::decrement_counter(&state, id)
        .await
        .expect("Cannot increment counter");

    HtmlView(CounterView {
        counter: counter.clone(),
        id: counter.id,
        name: counter.name,
        value: counter.value,
    })
}
