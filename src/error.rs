/// An error enum that encapsulates all possible errors in this crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Interior zero byte found during CString construction")]
    InteriorNulByte,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// A [`Result`](std::result::Result) type alias for this crate’s [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;
