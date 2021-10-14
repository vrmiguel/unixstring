use std::{
    ffi::{CStr, OsStr},
    path::Path,
};

use crate::UnixString;

impl AsRef<Path> for UnixString {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl AsRef<CStr> for UnixString {
    fn as_ref(&self) -> &CStr {
        self.as_c_str()
    }
}

impl AsRef<OsStr> for UnixString {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}
