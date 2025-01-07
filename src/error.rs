use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum MediaError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid box size")]
    InvalidBoxSize,

    #[error("Invalid box type: {0}")]
    InvalidBoxType(String),

    #[error("Unexpected EOF")]
    UnexpectedEof,

    #[error("Parse error: {0}")]
    Parse(String),
}
