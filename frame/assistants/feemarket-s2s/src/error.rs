use thiserror::Error as ThisError;

pub type FeemarketResult<T> = Result<T, FeemarketError>;

#[derive(ThisError, Debug)]
pub enum FeemarketError {
    #[error("Wrong data convert: {0}")]
    WrongConvert(String),
    #[error(transparent)]
    Subscan(#[from] component_subscan::SubscanComponentError),
    #[error("Custom: {0}")]
    Custom(String),
}
