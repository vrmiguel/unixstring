use std::convert::TryInto;
use std::ffi::OsString;
use std::{convert::TryFrom, path::PathBuf};

use crate::Result;
use crate::UnixString;

impl TryFrom<PathBuf> for UnixString {
    type Error = crate::error::Error;

    fn try_from(value: PathBuf) -> Result<Self> {
        value.into_os_string().try_into()
    }
}

impl TryFrom<OsString> for UnixString {
    type Error = crate::error::Error;

    fn try_from(value: OsString) -> Result<Self> {
        use std::os::unix::prelude::OsStringExt;

        Self::from_bytes(value.into_vec())
    }
}

impl TryFrom<String> for UnixString {
    type Error = crate::error::Error;

    fn try_from(value: String) -> Result<Self> {
        Self::from_bytes(value.into_bytes())
    }
}

impl TryFrom<Vec<u8>> for UnixString {
    type Error = crate::error::Error;

    fn try_from(bytes: Vec<u8>) -> Result<Self> {
        Self::from_bytes(bytes)
    }
}
