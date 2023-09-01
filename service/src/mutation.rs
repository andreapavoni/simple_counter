use ::entity::{events, events::Entity as Event};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_event(
        db: &DbConn,
        form_data: events::Model,
    ) -> Result<events::ActiveModel, DbErr> {
        events::ActiveModel {
            name: Set(form_data.name.to_owned()),
            value: Set(form_data.value.to_owned()),
            counter_id: Set(form_data.counter_id.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    pub async fn delete_all_events(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Event::delete_many().exec(db).await
    }
}
