use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
    string::FromUtf8Error,
};

use crate::error::{Error, Result};
use crate::memchr::find_nul_byte;

#[non_exhaustive]
pub struct UnixString {
    inner: Vec<u8>,
}

impl UnixString {
    /// Constructs a new, empty `UnixString`.
    /// The `UnixString`'s inner vector will not allocate until elements are pushed onto it.
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    fn inner_without_nul_terminator(&self) -> &[u8] {
        match self.inner.get(0..self.inner.len() - 1) {
            Some(bytes) => bytes,
            None => &self.inner,
        }
    }

    /// Converts the `UnixString` to an [`OsStr`] slice. This always succeeds and is zero cost. The terminating nul byte will not be included in the `OsStr` slice.
    /// ```rust
    /// use std::{convert::TryFrom, path::PathBuf};

    /// use unixstring::UnixString;
    ///
    /// let logs = PathBuf::from("/var/log/journal");
    /// let unix_string = UnixString::try_from(logs.clone()).unwrap();
    ///
    /// assert_eq!(
    ///     logs.as_os_str(),
    ///     unix_string.as_os_str()
    /// )
    ///
    /// ```
    pub fn as_os_str(&self) -> &OsStr {
        use std::os::unix::ffi::OsStrExt;

        OsStr::from_bytes(self.inner_without_nul_terminator())
    }

    /// Converts the `UnixString` to a [`Path`] slice. This always succeeds and is zero cost.
    /// The terminating nul byte will not be included in the `Path` slice.
    /// ```rust
    /// use std::{convert::TryFrom, path::PathBuf};
    ///
    /// use unixstring::UnixString;
    ///
    /// let home_dir = PathBuf::from("/home/user");
    /// let unix_string = UnixString::try_from(home_dir.clone()).unwrap();
    ///
    /// assert_eq!(&home_dir, unix_string.as_path())
    /// ```
    pub fn as_path(&self) -> &Path {
        Path::new(self.as_os_str())
    }

    /// Converts the `UnixString` to a [`CStr`] slice. This always succeeds and is zero cost.
    pub fn as_c_str(&self) -> &CStr {
        // Safety: we do not allow a UnixString to be built without a nul terminator, therefore this cannot fail.
        //
        // If you ever do see this function fail, please notify this at github.com/vrmiguel/unixstring
        CStr::from_bytes_with_nul(&self.inner).unwrap()
    }

    /// Tries to convert this `UnixString` into a [`&str`](str).
    ///
    /// The terminating nul byte will not be included in the `&str`.
    ///
    /// If this byte string is not valid UTF-8, then an error is returned indicating the first invalid byte found and the length of the error.
    /// If instead you wish for a lossy conversion to &str, then use one of the [`to_str_lossy`](unixstring::UnixString::to_string_lossy) or into_str_lossy methods.
    pub fn as_str(&self) -> std::result::Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.inner_without_nul_terminator())
    }

    /// Converts a `UnixString` into a String if the bytes of the `UnixString` are valid UTF-8.
    ///
    /// If you are sure that the byte slice is valid UTF-8 and you don’t want to incur the overhead of the validity check, there is an unsafe version of this function, [`UnixString::into_string_unchecked`](UnixString::into_string_unchecked), which has the same behavior but skips the check.
    ///
    /// If the validity check passes, the resulting `String` will reuse the allocation of the `UnixString`'s inner buffer and no copy will be done.
    ///
    /// If you need a `&str` instead of a `String`, consider [`UnixString::as_str`](UnixString::as_str).
    ///
    /// The inverse of this method is into_bytes.
    pub fn into_string(self) -> std::result::Result<String, FromUtf8Error> {
        let mut bytes = self.inner;
        bytes.remove(bytes.len() - 1);
        String::from_utf8(bytes)
    }

    /// Converts a `UnixString` into a `String` without checking that the
    /// string contains valid UTF-8.
    ///
    /// See the safe version, [`UnixString::into_string`](UnixString::into_string), for more details.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the bytes passed
    /// to it are valid UTF-8. If this constraint is violated, it may cause
    /// memory unsafety issues with future users of the `String`, as the rest of
    /// the standard library assumes that `String`s are valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// use unixstring::UnixString;
    ///
    /// let baby = UnixString::from_bytes(vec![0xF0, 0x9F, 0x91, 0xB6]).unwrap();
    ///
    /// let baby = unsafe {
    ///     baby.into_string_unchecked()
    /// };
    ///
    /// assert_eq!("👶", baby);
    /// ```
    pub unsafe fn into_string_unchecked(self) -> String {
        let mut bytes = self.inner;
        bytes.remove(bytes.len() - 1);
        String::from_utf8_unchecked(bytes)
    }

    /// Convert this byte string into a &str if it’s valid UTF-8.
    /// If this byte string is not valid UTF-8, then an error is returned. The error returned indicates the first invalid byte found and the length of the error.
    /// In cases where a lossy conversion to &str is acceptable, then use one of the to_str_lossy or to_str_lossy_into methods.
    pub fn to_string_lossy(&self) -> Cow<str> {
        self.as_os_str().to_string_lossy()
    }

    /// Creates a [`UnixString`](UnixString) given a
    ///
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        match find_nul_byte(&bytes) {
            Some(nul_pos) if nul_pos + 1 == bytes.len() => Ok(Self { inner: bytes }),
            Some(_nul_pos) => Err(Error::InternalNulByte),
            None => {
                let mut bytes = bytes;
                bytes.extend(Some(b'\0'));
                Ok(Self { inner: bytes })
            }
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.inner_without_nul_terminator()
    }

    pub fn as_bytes_with_nul(&self) -> &[u8] {
        &self.inner
    }

    pub fn into_bytes_with_nul(self) -> Vec<u8> {
        self.inner
    }
}

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
            inner: bytes_with_nul_terminator,
        }
    }
}
