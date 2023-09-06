use ::entity::{counters, events};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn emit_event(
        db: &DbConn,
        model: events::Model,
    ) -> Result<events::ActiveModel, DbErr> {
        events::ActiveModel {
            name: Set(model.name.to_owned()),
            value: Set(model.value.to_owned()),
            counter_id: Set(model.counter_id.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn add_counter(
        db: &DbConn,
        model: counters::Model,
    ) -> Result<counters::Model, DbErr> {
        counters::ActiveModel {
            id: Set(model.id.to_owned()),
            name: Set(model.name.to_owned()),
            value: Set(model.value.to_owned()),
            ..Default::default()
        }
        .insert(db)
        // .save(db) // for updates
        .await
    }
}
