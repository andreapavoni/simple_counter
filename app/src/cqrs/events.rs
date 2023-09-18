use mini_cqrs::*;
use serde::{Deserialize, Serialize};

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
            CounterEvent::CounterCreated { .. } => "CounterCreated".to_string(),
            CounterEvent::CounterIncremented { .. } => "CounterIncremented".to_string(),
            CounterEvent::CounterDecremented { .. } => "CounterDecremented".to_string(),
            CounterEvent::CounterUpdated { .. } => "CounterUpdated".to_string(),
            CounterEvent::CounterDeleted { .. } => "CounterDeleted".to_string(),
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
