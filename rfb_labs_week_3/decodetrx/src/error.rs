use thiserror::Error;

/// Errors that can occur while decoding a raw Bitcoin transaction.
#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("invalid transaction hex: {0}")]
    InvalidHex(#[from] hex::FromHexError),

    #[error("failed to read transaction data: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid CompactSize value")]
    InvalidCompactSize,

    #[error("invalid SegWit marker or flag")]
    InvalidSegwitMarker,

    #[error("transaction contains {0} unparsed trailing byte(s)")]
    TrailingBytes(usize),

    #[error("failed to serialize the decoded transaction: {0}")]
    Serialization(#[from] serde_json::Error),
}
