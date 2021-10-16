use std::fmt::Display;

/// An error enum that encapsulates all possible errors in this crate.
#[derive(Debug)]
pub enum Error {
    InteriorNulByte,
    IntoUtf8(std::str::Utf8Error),
    FromUtf8(std::string::FromUtf8Error),
    //#[error("IO error: {0}")]
    Io(std::io::Error),
}

/// A [`Result`](std::result::Result) type alias for this crate’s [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InteriorNulByte => {
                write!(f, "Interior zero byte found during CString construction")
            }
            Error::IntoUtf8(err) => write!(
                f,
                "Failed to interpret a sequence of bytes as a string,: {0}",
                err
            ),
            Error::FromUtf8(err) => write!(
                f,
                "Failed to create a String from a sequence of bytes: {0}",
                err
            ),
            Error::Io(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::IntoUtf8(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8(err)
    }
}
