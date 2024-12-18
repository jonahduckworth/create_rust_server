use crate::error::{ApiError, ErrorCode, ErrorContext};
use serde::Serialize;
use std::fmt;
use diesel::result::Error as DieselError;

#[derive(Debug, Serialize)]
pub enum DatabaseError {
    ConnectionFailed(String),
    QueryFailed(String),
    RecordNotFound(String),
    UniqueViolation(String),
    TransactionFailed(String),
    PoolError(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionFailed(msg) => write!(f, "Database connection failed: {}", msg),
            Self::QueryFailed(msg) => write!(f, "Database query failed: {}", msg),
            Self::RecordNotFound(msg) => write!(f, "Record not found: {}", msg),
            Self::UniqueViolation(msg) => write!(f, "Unique constraint violation: {}", msg),
            Self::TransactionFailed(msg) => write!(f, "Transaction failed: {}", msg),
            Self::PoolError(msg) => write!(f, "Connection pool error: {}", msg),
        }
    }
}

impl From<DatabaseError> for ApiError {
    fn from(error: DatabaseError) -> Self {
        let (code, message) = match &error {
            DatabaseError::ConnectionFailed(_) | DatabaseError::PoolError(_) => 
                (ErrorCode::ConnectionPoolError, error.to_string()),
            DatabaseError::QueryFailed(_) | DatabaseError::TransactionFailed(_) => 
                (ErrorCode::DatabaseError, error.to_string()),
            DatabaseError::RecordNotFound(_) => 
                (ErrorCode::NotFound, error.to_string()),
            DatabaseError::UniqueViolation(_) => 
                (ErrorCode::Conflict, error.to_string()),
        };

        ApiError::new(
            code,
            message,
            ErrorContext::new().with_details(serde_json::json!({
                "error_type": format!("{:?}", error)
            }))
        ).with_source(error)
    }
}

impl std::error::Error for DatabaseError {}

impl From<DieselError> for DatabaseError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => DatabaseError::RecordNotFound(error.to_string()),
            DieselError::DatabaseError(_, info) => DatabaseError::QueryFailed(info.message().to_string()),
            DieselError::RollbackTransaction => DatabaseError::TransactionFailed(error.to_string()),
            DieselError::AlreadyInTransaction => DatabaseError::TransactionFailed("Already in transaction".to_string()),
            DieselError::NotInTransaction => DatabaseError::TransactionFailed("Not in transaction".to_string()),
            _ => DatabaseError::QueryFailed(error.to_string()),
        }
    }
} 