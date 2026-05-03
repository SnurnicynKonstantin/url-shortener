#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Empty command")]
    Empty,

    #[error("Unknown command: {0}")]
    UnknownCommand(String),

    #[error("Invalid number of arguments")]
    InvalidArguments,

    #[error("Invalid TTL value: {0}")]
    InvalidTTL(#[from] std::num::ParseIntError),
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Lock poisoned")]
    LockPoisoned,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}