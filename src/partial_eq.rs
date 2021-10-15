use std::{
    ffi::{CStr, OsStr},
    path::Path,
};

use crate::UnixString;

impl PartialEq<&Path> for UnixString {
    fn eq(&self, other: &&Path) -> bool {
        self.as_path() == *other
    }
}

impl PartialEq<UnixString> for &Path {
    fn eq(&self, other: &UnixString) -> bool {
        other == self
    }
}

impl PartialEq<&str> for UnixString {
    /// Does a byte-level comparison to a string slice.
    /// This function does not at all suppose that `self` is valid UTF-8, but this does not make this comparison unsafe.
    fn eq(&self, other: &&str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<UnixString> for &str {
    /// Does a byte-level comparison to a string slice.
    /// This function does not at all suppose that `self` is valid UTF-8, but this does not make this comparison unsafe.
    fn eq(&self, other: &UnixString) -> bool {
        other == self
    }
}

impl PartialEq<&OsStr> for UnixString {
    fn eq(&self, other: &&OsStr) -> bool {
        self.as_os_str() == *other
    }
}

impl PartialEq<UnixString> for &OsStr {
    fn eq(&self, other: &UnixString) -> bool {
        other == self
    }
}

impl PartialEq<&CStr> for UnixString {
    fn eq(&self, other: &&CStr) -> bool {
        self.as_c_str() == *other
    }
}

impl PartialEq<UnixString> for &CStr {
    fn eq(&self, other: &UnixString) -> bool {
        other == self
    }
}
