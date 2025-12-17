use thiserror::Error;

/// Generic error type used by service layer functions.
#[derive(Debug, Error)]
pub enum ServiceError {
    /// The user is not authorized to perform the operation.
    #[error("unauthorized")]
    Unauthorized,

    /// Requested resource was not found.
    #[error("not found")]
    NotFound,

    /// The resource already exists.
    #[error("conflict")]
    Conflict,

    /// Persistence layer failures.
    #[cfg(feature = "db")]
    #[error("repository error: {0}")]
    Repository(crate::repository::errors::RepositoryError),

    /// ZmqSenderError
    #[cfg(feature = "zeromq")]
    #[error("zmq send error: {0}")]
    ZmqSender(#[from] crate::zmq::ZmqSenderError),

    /// Form validation error.
    #[error("form error: {0}")]
    Form(String),

    /// Problems with environment or configuration.
    #[error("configuration error: {0}")]
    Config(String),

    /// An unexpected internal error occurred.
    #[error("internal error")]
    Internal,

    /// Type constraint violation.
    #[error("type constraint violation: {0}")]
    TypeConstraint(String),
}

/// Convenient alias for results returned from service functions.
pub type ServiceResult<T> = Result<T, ServiceError>;

// Manual From implementation for RepositoryError
#[cfg(feature = "db")]
impl From<crate::repository::errors::RepositoryError> for ServiceError {
    fn from(err: crate::repository::errors::RepositoryError) -> Self {
        match err {
            crate::repository::errors::RepositoryError::NotFound => ServiceError::NotFound,
            crate::repository::errors::RepositoryError::ConstraintViolation(_) => {
                ServiceError::Conflict
            }
            other => ServiceError::Repository(other),
        }
    }
}
