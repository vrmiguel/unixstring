use std::{
    ffi::{CString, OsString},
    path::PathBuf,
};

use crate::UnixString;

impl From<UnixString> for CString {
    fn from(unx: UnixString) -> Self {
        let bytes = unx.into_bytes();

        // Safety: a UnixString shall never contain a zero byte except for its null terminator.
        // UnixString::into_bytes strips away the null terminator, therefore using this function is safe
        unsafe { CString::from_vec_unchecked(bytes) }
    }
}

impl From<UnixString> for OsString {
    fn from(unx: UnixString) -> Self {
        use std::os::unix::prelude::OsStringExt;

        let bytes = unx.into_bytes();

        OsString::from_vec(bytes)
    }
}

impl From<UnixString> for PathBuf {
    fn from(unx: UnixString) -> Self {
        let os_string = unx.into_os_string();

        PathBuf::from(os_string)
    }
}
