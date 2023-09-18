use async_trait::async_trait;
use mini_cqrs::*;
use serde::{Deserialize, Serialize};

use super::{CounterEvent, CounterCommand};

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
