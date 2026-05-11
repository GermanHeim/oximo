use thiserror::Error;

#[derive(Debug, Error)]
pub enum IoError {
    #[error("model has no objective")]
    NoObjective,
    #[error("nonlinear nodes are not representable in this format")]
    Nonlinear,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}
