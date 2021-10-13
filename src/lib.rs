//! [`UnixString`](UnixString) is an FFI-friendly null-terminated byte string that may be constructed from a [`String`], a [`CString`](std::ffi::CString), a [`PathBuf`](std::path::PathBuf), an [`OsString`](std::ffi::OsString) or a collection of bytes.
//! 
//! 
//! An [`UnixString`](UnixString) can then be converted into a slice of [`CStr`](std::ffi::CStr), [`Path`](std::path::Path) or [`OsStr`](std::ffi::OsStr) in infallible and zero-cost operations.

mod error;
mod memchr;
mod unix_string;

pub use error::{Error, Result};
pub use unix_string::UnixString;