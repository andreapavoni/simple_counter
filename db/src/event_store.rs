use async_trait::async_trait;
pub use mini_cqrs::{CqrsError, Event, EventStore as CqrsEventStore};

use crate::{entity::events, DbConn};

use sea_orm::*;

// Event Store
#[derive(Clone)]
pub struct EventStore {
    db: DbConn,
}

impl EventStore {
    pub fn new(db: DbConn) -> Self {
        EventStore { db }
    }
}

#[async_trait]
impl CqrsEventStore for EventStore {
    type AggregateId = String;

    async fn save_events(
        &mut self,
        _aggregate_id: Self::AggregateId,
        events: &[Event],
    ) -> Result<(), CqrsError> {
        for evt in events.iter() {
            let model: events::ActiveModel =
                <Event as Into<events::Model>>::into(evt.clone()).into();
            let db = self.db.clone();
            let _ = model.insert(&db).await;
        }

        Ok(())
    }

    async fn load_events(&self, aggregate_id: Self::AggregateId) -> Result<Vec<Event>, CqrsError> {
        if let Ok(models) = events::Entity::find()
            .filter(events::Column::AggregateId.eq(aggregate_id.clone().to_string()))
            .all(&self.db.clone())
            .await
        {
            let events = models
                .into_iter()
                .map(|model| Event {
                    id: model.id,
                    aggregate_id: aggregate_id.clone(),
                    event_type: model.name,
                    payload: model.payload,
                    timestamp: model.timestamp,
                    version: 1,
                })
                .collect();

            return Ok(events);
        }

        Ok(vec![])
    }
}
