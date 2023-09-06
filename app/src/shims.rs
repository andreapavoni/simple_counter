use crate::domain::models::*;
use kountr_db::entity::counters::Model as ModelCounter;

// COUNTER: domain::models:Counter <-> kountr_db::entity::counters::Model
impl From<ModelCounter> for Counter {
    fn from(model: ModelCounter) -> Self {
        Counter {
            id: model.id,
            name: model.name,
            value: model.value,
        }
    }
}

impl Into<ModelCounter> for Counter {
    fn into(self) -> ModelCounter {
        ModelCounter {
            id: self.id,
            name: self.name,
            value: self.value,
        }
    }
}
