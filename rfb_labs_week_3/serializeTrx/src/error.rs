use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerializeError {
    #[error("invalid hexadecimal value: {0}")]
    InvalidHex(#[from] hex::FromHexError),
}
