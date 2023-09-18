use async_trait::async_trait;
use mini_cqrs::*;
use serde::{Deserialize, Serialize};

use crate::domain::models;
use kountr_db::repository::Repository;

// COMMANDS
#[derive(PartialEq, Clone)]
pub enum CounterCommand {
    Create {
        name: String,
        value: i32,
    },
    Increment {
        id: String,
        amount: i32,
    },
    Decrement {
        id: String,
        amount: i32,
    },
    Update {
        id: String,
        name: String,
        value: i32,
    },
    Delete {
        id: String,
    },
}

// EVENTS
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum CounterEvent {
    CounterCreated {
        aggregate_id: String,
        name: String,
        value: i32,
    },
    CounterIncremented {
        aggregate_id: String,
        amount: i32,
    },
    CounterDecremented {
        aggregate_id: String,
        amount: i32,
    },
    CounterUpdated {
        aggregate_id: String,
        name: String,
        value: i32,
    },
    CounterDeleted {
        aggregate_id: String,
    },
}

impl ToString for CounterEvent {
    fn to_string(&self) -> String {
        match self {
            CounterEvent::CounterCreated { .. } => {
                format!("CounterCreated")
            }
            CounterEvent::CounterIncremented { .. } => {
                format!("CounterIncremented")
            }
            CounterEvent::CounterDecremented { .. } => {
                format!("CounterDecremented")
            }
            CounterEvent::CounterUpdated { .. } => {
                format!("CounterUpdated")
            }
            CounterEvent::CounterDeleted { .. } => {
                format!("CounterDeleted")
            }
        }
    }
}

impl EventPayload for CounterEvent {
    fn aggregate_id(&self) -> String {
        match self {
            CounterEvent::CounterCreated { aggregate_id, .. } => aggregate_id.clone(),
            CounterEvent::CounterIncremented { aggregate_id, .. } => aggregate_id.clone(),
            CounterEvent::CounterDecremented { aggregate_id, .. } => aggregate_id.clone(),
            CounterEvent::CounterUpdated { aggregate_id, .. } => aggregate_id.clone(),
            CounterEvent::CounterDeleted { aggregate_id } => aggregate_id.clone(),
        }
    }
}

wrap_event!(CounterEvent);

// AGGREGATE

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CounterState {
    id: String,
    name: String,
    value: i32,
}

impl Default for CounterState {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            value: 0,
            name: "Default counter".to_string(),
        }
    }
}

#[async_trait]
impl Aggregate for CounterState {
    type Event = CounterEvent;
    type Command = CounterCommand;
    type Id = String;

    async fn handle(&self, command: CounterCommand) -> Result<Vec<Event>, CqrsError> {
        match command {
            CounterCommand::Create { name, value } => Ok(vec![CounterEvent::CounterCreated {
                aggregate_id: self.id.clone(),
                name,
                value,
            }
            .into()]),
            CounterCommand::Increment { id, amount } => {
                Ok(vec![CounterEvent::CounterIncremented {
                    aggregate_id: id,
                    amount,
                }
                .into()])
            }
            CounterCommand::Decrement { id, amount } => {
                Ok(vec![CounterEvent::CounterDecremented {
                    aggregate_id: id,
                    amount,
                }
                .into()])
            }
            CounterCommand::Update { id, name, value } => Ok(vec![CounterEvent::CounterUpdated {
                aggregate_id: id,
                name,
                value,
            }
            .into()]),
            CounterCommand::Delete { id } => {
                Ok(vec![
                    CounterEvent::CounterDeleted { aggregate_id: id }.into()
                ])
            }
        }
    }

    fn apply(&mut self, event: &Self::Event) {
        match event {
            CounterEvent::CounterCreated {
                aggregate_id, name, ..
            } => {
                self.id = aggregate_id.clone();
                self.value = 0;
                self.name = name.clone();
            }
            CounterEvent::CounterIncremented { amount, .. } => {
                self.value += amount;
            }
            CounterEvent::CounterDecremented { amount, .. } => {
                self.value -= amount;
            }
            CounterEvent::CounterUpdated { name, value, .. } => {
                self.name = name.clone();
                self.value = value.clone();
            }
            CounterEvent::CounterDeleted { .. } => {}
        }
    }

    fn aggregate_id(&self) -> Self::Id {
        self.id.clone()
    }

    fn set_aggregate_id(&mut self, id: Self::Id) {
        self.id = id.clone();
    }
}

// QUERIES

#[derive(Clone)]
pub struct AppQueries {}

#[async_trait]
impl QueriesRunner for AppQueries {}

#[derive(Clone)]
pub struct GetCounterQuery {
    pub id: String,
    repo: Repository,
}

impl GetCounterQuery {
    pub fn new(id: String, repo: &Repository) -> Self {
        Self {
            id,
            repo: repo.clone(),
        }
    }
}

#[async_trait]
impl Query for GetCounterQuery {
    type Output = Result<Option<models::Counter>, CqrsError>;

    async fn apply(&self) -> Self::Output {
        let model = self.repo.find_counter_by_id(self.id.clone()).await;

        Ok(Some(model.unwrap().into()))
    }
}

#[derive(Clone)]
pub struct ListCountersQuery {
    repo: Repository,
}

impl ListCountersQuery {
    pub fn new(repo: &Repository) -> Self {
        Self { repo: repo.clone() }
    }
}

#[async_trait]
impl Query for ListCountersQuery {
    type Output = Result<Vec<models::Counter>, CqrsError>;

    async fn apply(&self) -> Self::Output {
        if let Ok(list) = self.repo.list_counters().await {
            let result = list.into_iter().map(|c| c.into());
            return Ok(result.collect());
        }
        // TODO: handle errors
        Ok(vec![])
    }
}

// READ MODELS

#[derive(Clone)]
pub struct CounterView {
    repo: Repository,
}

impl CounterView {
    pub fn new(repo: &Repository) -> Self {
        Self { repo: repo.clone() }
    }
}

#[async_trait]
impl ModelReader for CounterView {
    type Repo = Repository;
    type Model = models::Counter;

    async fn update(&mut self, data: Self::Model) -> Result<(), CqrsError> {
        let _ = self
            .repo
            .insert_or_update_counter(data.clone().into())
            .await;
        Ok(())
    }
}

// CONSUMER

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
