use thiserror::Error as ThisError;

pub type FeemarketResult<T> = Result<T, FeemarketError>;

#[derive(ThisError, Debug)]
pub enum FeemarketError {
    #[error(transparent)]
    RelayClient(#[from] relay_substrate_client::Error),
    #[error("Wrong data convert: {0}")]
    WrongConvert(String),
}
