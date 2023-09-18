use async_trait::async_trait;
use mini_cqrs::*;

use crate::domain::models;
use kountr_db::repository::Repository;

#[derive(Clone)]
pub struct CounterView {
    pub repo: Repository,
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
