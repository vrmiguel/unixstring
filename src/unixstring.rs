use std::{convert::{TryFrom, TryInto}, ffi::{CStr, CString, OsStr, OsString}, path::{Path, PathBuf}};

use bstr::ByteSlice;

use crate::error::{Error, Result};
use crate::memchr;

pub struct UnixString {
    inner: Vec<u8>,
}

fn find_nul_byte(bytes: &[u8]) -> Option<usize> {
    memchr::memchr(0, &bytes)
}

impl UnixString {
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn as_os_str(&self) -> &OsStr {
        // Safety: bstr::ByteSlice::to_os_str never fails on Unix and is zero-cost
        match self.inner.get(0..self.inner.len()-1) {
            Some(bytes) => bytes.to_os_str().unwrap(),
            None => {
                self.inner.to_os_str().unwrap()
            },
        }
    }

    pub fn as_path(&self) -> &Path {
        Path::new(self.as_os_str())
    }

    pub fn as_c_str(&self) -> &CStr {
        // Safety: unless this crate is a total failure, this cannot fail.
        // If you ever see this function fail, please notify this at our GitHub repo.
        CStr::from_bytes_with_nul(&self.inner).unwrap()
    }

    pub fn as_str(&self) -> std::result::Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.inner)
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        match find_nul_byte(&bytes) {
            Some(nul_pos) if nul_pos + 1 == bytes.len() => Ok(Self { inner: bytes }),
            Some(_nul_pos)=> Err(Error::InternalNulByte),
            None => {
                let mut bytes = bytes;
                bytes.extend(Some(b'\0'));
                Ok(Self { inner: bytes })
            }
        }
    }
}

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

impl From<CString> for UnixString {
    fn from(value: CString) -> Self {
        let bytes_with_nul_terminator = value.into_bytes_with_nul();

        Self {
            inner: bytes_with_nul_terminator
        }
    }
}