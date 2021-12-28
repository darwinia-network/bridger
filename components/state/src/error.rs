use thiserror::Error as ThisError;

/// Ethereum component error
#[derive(ThisError, Debug)]
pub enum StateComponentError {
    #[error("Microkv error: {0}")]
    Microkv(String),
}
