use async_trait::async_trait;
use mini_cqrs::*;

use crate::domain::models;
use kountr_db::repository::Repository;

use super::{CounterView, CounterEvent};

#[derive(Clone)]
pub struct CounterEventConsumer {
    pub counter_model: CounterView,
}

impl CounterEventConsumer {
    pub fn new(repo: &Repository) -> Self {
        Self {
            counter_model: CounterView::new(repo),
        }
    }
}

#[async_trait]
impl EventConsumer for CounterEventConsumer {
    async fn process(&mut self, evt: Event) {
        let event = evt.get_payload::<CounterEvent>();
        match event {
            CounterEvent::CounterCreated {
                aggregate_id,
                name,
                value,
            } => {
                let counter = models::Counter::new_with_id(aggregate_id.clone(), name, value);
                _ = self.counter_model.update(counter).await;
            }
            CounterEvent::CounterIncremented {
                aggregate_id,
                amount,
            } => {
                let mut counter = self
                    .counter_model
                    .repo
                    .find_counter_by_id(aggregate_id.clone())
                    .await
                    .unwrap();
                counter.value += amount;
                _ = self.counter_model.update(counter.into()).await;
            }
            CounterEvent::CounterDecremented {
                aggregate_id,
                amount,
            } => {
                let mut counter = self
                    .counter_model
                    .repo
                    .find_counter_by_id(aggregate_id.clone())
                    .await
                    .unwrap();
                counter.value -= amount;
                _ = self.counter_model.update(counter.into()).await;
            }
            CounterEvent::CounterUpdated {
                aggregate_id,
                name,
                value,
            } => {
                let mut counter = self
                    .counter_model
                    .repo
                    .find_counter_by_id(aggregate_id.clone())
                    .await
                    .unwrap();
                counter.name = name;
                counter.value = value;
                _ = self.counter_model.update(counter.into()).await;
            }
            CounterEvent::CounterDeleted { aggregate_id } => {
                _ = self
                    .counter_model
                    .repo
                    .delete_counter(aggregate_id.clone())
                    .await;
            }
        }
    }
}

event_consumers_group! {
    MainEventConsumers {
        Counter => CounterEventConsumer,
    }
}
