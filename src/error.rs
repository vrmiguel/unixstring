// use std::ffi::NulError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Internal zero byte found during CString construction")]
    InternalNulByte,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
