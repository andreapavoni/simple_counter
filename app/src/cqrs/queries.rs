use async_trait::async_trait;
use mini_cqrs::*;

use crate::domain::models;
use kountr_db::repository::Repository;

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

