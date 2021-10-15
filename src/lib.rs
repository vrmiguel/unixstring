//! # unixstring [![codecov](https://codecov.io/gh/vrmiguel/unixstring/branch/master/graph/badge.svg?token=6rvhsF5Eiq)](https://codecov.io/gh/vrmiguel/unixstring) ![Crates.io](https://img.shields.io/crates/v/unixstring) [![Docs](https://img.shields.io/badge/docs.rs-unixstring-green)](https://docs.rs/unixstring/)

//! [`UnixString`](UnixString) is an FFI-friendly null-terminated byte string that may be constructed from a [`String`], a [`CString`](std::ffi::CString), a [`PathBuf`](std::path::PathBuf), an [`OsString`](std::ffi::OsString) or a collection of bytes.
//!
//!
//! An [`UnixString`](UnixString) can then be converted into a slice of [`CStr`](std::ffi::CStr), [`Path`](std::path::Path) or [`OsStr`](std::ffi::OsStr) in infallible and zero-cost operations.

//! ## Why?
//!
//! `UnixString` aims to be useful in any scenario where you'd like to use FFI (specially with C) on Unix systems.
//! If you have a `PathBuf`, for example, you can send that data to a `libc` function, such as `stat`, but you'd have to first allocate a `CString` (or something analogous) to do so.
//!
//! The same is true with `OsString` and `String` because these three types are allowed to have internal zero bytes and are not null-terminated.
//!
//!
//! A `UnixString` is very close to what a `CString` is but with increased flexibility and usability. A `CString` cannot be changed or increased after instantited, while `UnixString` is growable through its `push` and `push_bytes` methods, somewhat similar to `OsString`.
//!
//! A `CString` also does not have direct reference conversions to anything but `&[u8]` or `&CStr`, while `UnixString` has those and more (described below).
//!
//! ## Obtaining references from an UnixString
//!
//! |   Into   |            Function             |                               Notes                               |
//! |:--------:|:-------------------------------:|:-----------------------------------------------------------------:|
//! | `&CStr`  |     `UnixString::as_c_str`      |                 Available through `AsRef` as well                 |
//! | `&Path`  |      `UnixString::as_path`      |                 Available through `AsRef` as well                 |
//! |  `&str`  |      `UnixString::as_str`       |     Fails if the bytes of the `UnixString` aren't valid UTF-8     |
//! | `&[u8]`  |     `UnixString::as_bytes`      | Returns the bytes of the `UnixString` without the null terminator |
//! | `&[u8]`  | `UnixString::as_bytes_with_nul` |  Returns the bytes of the `UnixString` with the null terminator   |
//! | `&OsStr` |     `UnixString::as_os_str`     |                 Available through `AsRef` as well                 |
//! | `* const c_char` | `UnixString::as_ptr`    |                                                                   |
//!
//! ## Creating an UnixString
//!
//! |    From    |            Potential failure            | Trait impl |           Function           |
//! |:----------:|:---------------------------------------:|:----------:|:----------------------------:|
//! | `CString`  |               Infallible                |    From    |  `UnixString::from_cstring`  |
//! | `PathBuf`  | Fails if contains an interior zero byte |  TryFrom   |  `UnixString::from_pathbuf`  |
//! |  `String`  | Fails if contains an interior zero byte |  TryFrom   |  `UnixString::from_string`   |
//! | `Vec<u8>`  | Fails if contains an interior zero byte |  TryFrom   |   `UnixString::from_bytes`   |
//! | `OsString` | Fails if contains an interior zero byte |  TryFrom   | `UnixString::from_os_string` |
//! | `* const c_char` | Unsafe, see the docs for more info|  None      | `UnixString::from_ptr`       |
//!
//! ## Converting from an UnixString
//!
//!
//! |    Into    |              Function               |                                 Notes                                  |
//! |:----------:|:-----------------------------------:|:----------------------------------------------------------------------:|
//! | `CString`  |     `UnixString::into_cstring`      |                                                                        |
//! | `PathBuf`  |     `UnixString::into_pathbuf`      |                                                                        |
//! | `OsString` |    `UnixString::into_os_string`     |                                                                        |
//! |  `String`  |      `UnixString::into_string`      |         Fails if the `UnixString`'s bytes are not valid UTF-8          |
//! |  `String`  |   `UnixString::into_string_lossy`   |                                                                        |
//! |  `String`  |    `UnixString::to_string_lossy`    |         Non-moving version of `UnixString::into_string_lossy`          |
//! |  `String`  | `UnixString::into_string_unchecked` | Unsafe: creates a String without checking if the bytes are valid UTF-8 |
//! | `Vec<u8>`  |      `UnixString::into_bytes`       |   Returns the bytes of the `UnixString` without the null terminator    |
//! | `Vec<u8>`  |  `UnixString::into_bytes_with_nul`  |     Returns the bytes of the `UnixString` with the null terminator     |
//!
//! All of the above are also available through `.into()`.

mod as_ref;
mod error;
mod from;
mod memchr;
mod partial_eq;
mod try_from;
mod unix_string;

pub use error::{Error, Result};
pub use unix_string::UnixString;
