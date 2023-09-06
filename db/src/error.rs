use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error(transparent)]
    Db(#[from] sea_orm::DbErr),
    NotFound,
}

impl Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
