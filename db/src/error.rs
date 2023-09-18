use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
    NotFound,
    #[error(transparent)]
    Cqrs(#[from] mini_cqrs::CqrsError),
}

impl Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
