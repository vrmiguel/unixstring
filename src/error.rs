/// An error enum that encapsulates all possible errors in this crate.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Interior zero byte found during CString construction")]
    InteriorNulByte,
    #[error("Failed to interpret a sequence of bytes as a string: {0}")]
    IntoUtf8(#[from] std::str::Utf8Error),
    #[error("Failed to create a String from a sequence of bytes: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// A [`Result`](std::result::Result) type alias for this crate’s [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;
