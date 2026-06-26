#[derive(Debug, thiserror::Error)]
pub enum FlockError {
    #[error("Internal error: {0}")]
    Internal(String),
}
pub type FlockResult<T> = Result<T, FlockError>;
