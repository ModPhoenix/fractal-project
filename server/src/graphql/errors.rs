// fractal-project/server/src/graphql/errors.rs

use async_graphql::ErrorExtensions;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphQLError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::data::DataError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal server error")]
    InternalServerError,
}

impl ErrorExtensions for GraphQLError {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self)).extend_with(|_, e| match self {
            GraphQLError::DatabaseError(_) => {
                e.set("code", "DATABASE_ERROR");
            }
            GraphQLError::NotFound(_) => {
                e.set("code", "NOT_FOUND");
            }
            GraphQLError::InvalidInput(_) => {
                e.set("code", "INVALID_INPUT");
            }
            GraphQLError::Unauthorized(_) => {
                e.set("code", "UNAUTHORIZED");
            }
            GraphQLError::InternalServerError => {
                e.set("code", "INTERNAL_SERVER_ERROR");
            }
        })
    }
}
