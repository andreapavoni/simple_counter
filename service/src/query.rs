use ::entity::{events, events::Entity as Event, counters, counters::Entity as Counter};
use sea_orm::*;

pub struct Query;

impl Query {
    // pub async fn find_post_by_id(db: &DbConn, id: String) -> Result<Option<events::Model>, DbErr> {
    //     Event::find_by_id(id).one(db).await
    // }

    pub async fn list_events_by_counter_id(
        db: &DbConn,
        counter_id: String,
    ) -> Result<Vec<events::Model>, DbErr> {
        Event::find()
            .filter(events::Column::CounterId.eq(counter_id))
            .order_by_desc(events::Column::Timestamp)
            .all(db)
            .await
    }

    pub async fn list_all_events(db: &DbConn) -> Result<Vec<events::Model>, DbErr> {
        Event::find()
            .order_by_desc(events::Column::Timestamp)
            .all(db)
            .await
    }


    pub async fn list_all_counters(db: &DbConn) -> Result<Vec<counters::Model>, DbErr> {
        Counter::find()
            .all(db)
            .await
    }
}
