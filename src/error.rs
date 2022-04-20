use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
pub enum KvsError {
    #[error("IO failed")]
    IO(#[from] io::Error),

    #[error("Serde json failed")]
    Serde(#[from] serde_json::Error),

    #[error("Not found for key: {0}")]
    KeyNotFound(String),

    #[error("un expected command type")]
    UnExpectedCommandType,
}

pub type Result<T> = std::result::Result<T, KvsError>;
