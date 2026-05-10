use thiserror::Error;

#[derive(Debug, Error)]
pub enum IoError {
    #[error("model has no objective. MPS requires one")]
    NoObjective,
    #[error("nonlinear nodes are not representable in MPS")]
    Nonlinear,
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}
