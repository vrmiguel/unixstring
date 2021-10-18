use std::{
    borrow::Cow,
    convert::TryInto,
    ffi::{CStr, CString, OsStr, OsString},
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
};

use crate::error::{Error, Result};
use crate::memchr::find_nul_byte;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// An FFI-friendly null-terminated byte string.
#[non_exhaustive]
pub struct UnixString {
    inner: Vec<u8>,
}

impl Default for UnixString {
    fn default() -> Self {
        Self { inner: vec![0] }
    }
}

impl UnixString {
    /// Constructs a new, "empty" `UnixString`.
    ///
    /// This function allocates a byte in order to store its nul terminator.
    ///
    /// If you are creating a `UnixString` with the intention of writing stuff to it, you may be interested in [`UnixString::with_capacity`](UnixString::with_capacity).
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates if this `UnixString` has a correct internal representation: with a zero byte only at the end.
    ///
    /// All of the safe functions in this crate *must* maintain the `UnixString` in a valid state.
    ///
    /// This method is particularly useful to guarantee that a `UnixString` remains valid after being possibly modified through [`UnixString::as_mut_ptr`](UnixString::as_mut_ptr),
    /// or making sure that a `UnixString` created from [`UnixString::from_ptr`](UnixString::from_ptr) is correct.
    ///
    /// ```rust
    /// use unixstring::UnixString;
    /// # use unixstring::Result;
    /// # fn main() -> Result<()> {
    ///
    /// let mut unix_string = UnixString::new();
    /// unix_string.push("hello")?;
    /// unix_string.push("world")?;
    ///
    /// assert!(unix_string.validate().is_ok());
    ///
    /// # Ok(()) }
    /// ```
    pub fn validate(&self) -> Result<()> {
        let bytes = &*self.inner;
        match find_nul_byte(bytes) {
            Some(nul_pos) if nul_pos + 1 == bytes.len() => Ok(()),
            Some(_nul_pos) => Err(Error::InteriorNulByte),
            None => Err(Error::MissingNulTerminator),
        }
    }

    // Removes the existing nul terminator and then extends `self` with the given bytes.
    // Assumes that the given bytes have a nul terminator
    fn extend_slice(&mut self, slice: &[u8]) {
        let removed = self.inner.remove(self.inner.len() - 1);
        debug_assert!(removed == 0);
        self.inner.extend_from_slice(slice);
    }

    /// Extends the `UnixString` with anything that implements [`AsRef`](std::convert::AsRef)<[`OsStr`](std::ffi::OsStr)>.
    ///
    /// This method fails if the given data has a zero byte anywhere but at its end.
    ///
    /// ```rust
    /// # use unixstring::Result;
    /// use std::path::Path;
    ///
    /// use unixstring::UnixString;
    /// # fn main() -> Result<()> {
    /// let mut unix_string = UnixString::new();
    /// unix_string.push("/home/")?;
    /// let username = Path::new("user");
    /// unix_string.push(username)?;
    ///
    /// assert_eq!(unix_string.to_str()?, "/home/user");
    /// # Ok(()) }
    ///
    pub fn push(&mut self, value: impl AsRef<OsStr>) -> Result<()> {
        self.push_bytes(value.as_ref().as_bytes())
    }

    /// Extends the `UnixString` with the given bytes.
    ///
    /// This method fails if the bytes contain an interior zero byte (a zero byte not at the buffer's final position)
    ///
    /// ```rust
    /// # use unixstring::Result;
    /// use std::path::Path;
    ///
    /// use unixstring::UnixString;
    /// # fn main() -> Result<()> {
    /// let mut unix_string = UnixString::new();
    ///
    /// let abc = b"abc".to_vec();
    /// unix_string.push_bytes(&abc)?;
    ///
    /// assert_eq!(unix_string.into_bytes(), abc);
    /// # Ok(()) }
    pub fn push_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        match find_nul_byte(bytes) {
            Some(nul_pos) if nul_pos + 1 == bytes.len() => {
                // The given bytes already have a nul terminator
                self.extend_slice(bytes);
                Ok(())
            }
            Some(_nul_pos) => Err(Error::InteriorNulByte),
            None => {
                // There was no zero byte at all on the given bytes so we'll
                // have to manually append the null terminator after appending.
                self.extend_slice(bytes);
                self.inner.extend(Some(b'\0'));
                Ok(())
            }
        }
    }

    /// Creates a [`UnixString`](UnixString) given a `Vec` of bytes.
    ///
    /// This method will return an error if the given bytes have a zero byte, *except* if the zero byte is the last element of the `Vec`.
    ///  
    /// ```rust
    /// use unixstring::UnixString;
    ///
    /// let bytes_without_zero = b"abc".to_vec();
    /// let bytes_with_nul_terminator = b"abc\0".to_vec();
    /// let bytes_with_interior_nul = b"a\0bc".to_vec();
    ///
    /// // Valid: no zero bytes were given
    /// assert!(UnixString::from_bytes(bytes_without_zero).is_ok());
    ///
    /// // Still valid: the zero byte is being used as the terminator
    /// assert!(UnixString::from_bytes(bytes_with_nul_terminator).is_ok());
    ///
    /// // Invalid: an interior nul byte was found
    /// assert!(UnixString::from_bytes(bytes_with_interior_nul).is_err());
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        match find_nul_byte(&bytes) {
            Some(nul_pos) if nul_pos + 1 == bytes.len() => Ok(Self { inner: bytes }),
            Some(_nul_pos) => Err(Error::InteriorNulByte),
            None => {
                let mut bytes = bytes;
                bytes.extend(Some(b'\0'));
                Ok(Self { inner: bytes })
            }
        }
    }

    /// Constructs a new, empty `UnixString` with the specified capacity.
    ///
    /// The `UnixString`'s inner vector will be able to hold exactly `capacity` elements without
    /// reallocating.
    ///
    /// This function will always allocate enough to fit the null terminator byte, even if the given capacity is 0.
    ///
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX - 1` bytes.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut inner = Vec::with_capacity(capacity + 1);
        inner.push(0);

        Self { inner }
    }

    /// Clones a raw C string into an `UnixString`.
    ///
    /// The total size of the raw C string must be smaller than `isize::MAX` **bytes**
    /// in memory due to calling the `slice::from_raw_parts` function.
    ///
    /// # Safety
    ///
    /// This method is unsafe for a number of reasons:
    ///
    /// * There is no guarantee to the validity of `ptr`.
    /// * There is no guarantee that the memory pointed to by `ptr` contains a
    ///   valid nul terminator byte at the end of the string.
    /// * It is not guaranteed that the memory pointed by `ptr` won't change
    ///   before the `UnixString` has been constructed.
    ///
    /// See [`CStr::from_ptr`](std::ffi::CStr::from_ptr) for more info.
    pub unsafe fn from_ptr(ptr: *const libc::c_char) -> Self {
        CStr::from_ptr(ptr).to_owned().into()
    }

    /// Returns an inner pointer to the data this `UnixString` contains.
    ///
    /// The returned pointer will be valid for as long as the given `UnixString` is, and points
    /// to a null-terminated contiguous region of memory.
    ///
    /// *Note*: The returned pointer is read-only and writing to it in any way causes undefined behavior.
    ///
    /// You must ensure that the underlying memory is not
    /// freed too early. If the `UnixString` is deallocated then the pointer becomes dangling.
    ///
    /// See [`CStr::as_ptr`](std::ffi::CStr::as_ptr) for more info.
    ///
    pub fn as_ptr(&self) -> *const libc::c_char {
        self.as_c_str().as_ptr()
    }

    fn inner_without_nul_terminator(&self) -> &[u8] {
        &self.inner[0..self.inner.len() - 1]
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
    /// If instead you wish for a lossy conversion to &str, then use [`to_str_lossy`](UnixString::to_string_lossy).
    pub fn to_str(&self) -> Result<&str> {
        Ok(std::str::from_utf8(self.inner_without_nul_terminator())?)
    }

    /// Extends a `UnixString` by copying from a raw C string
    ///
    /// # Safety
    ///
    /// This method is unsafe for a number of reasons:
    /// * The total size of the raw C string must be smaller than `isize::MAX` bytes in memory.
    /// * There is no guarantee to the validity of `ptr`.
    /// * There is no guarantee that the memory pointed to by `ptr` contains a
    ///   valid nul terminator byte at the end of the string.
    /// * It is not guaranteed that the memory pointed by `ptr` won't change
    ///   while this operation occurs.
    ///
    /// This function uses [`CStr::from_ptr`](std::ffi::CStr::from_ptr) internally, so check it out for more information.
    ///
    pub unsafe fn extend_from_ptr(&mut self, ptr: *const libc::c_char) -> Result<()> {
        let cstr = CStr::from_ptr(ptr);
        let bytes = cstr.to_bytes();
        self.push_bytes(bytes)
    }

    /// This function will check if the inner bytes of this `UnixString` (without its null terminator) consists of valid UTF-8, and if so, returns a `String` reusing the `UnixString`'s buffer.
    ///
    /// If the inner bytes contain invalid UTF-8, then a new `String` will be allocated with the invalid bytes replaced with the Unicode replacement codepoint.
    pub fn into_string_lossy(self) -> String {
        let bytes = self.into_bytes();
        match String::from_utf8_lossy(&bytes) {
            Cow::Owned(new_string) => new_string,
            Cow::Borrowed(_) => {
                // Safety: String::from_utf8_lossy returns a reference when the supplied input
                // is valid UTF-8. Constructing a String unchecked, therefore, is safe.
                unsafe { String::from_utf8_unchecked(bytes) }
            }
        }
    }

    /// Converts a `UnixString` into a String if the bytes of the `UnixString` are valid UTF-8.
    ///
    /// If you are sure that the byte slice is valid UTF-8 and you don’t want to incur the overhead of the validity check, there is an unsafe version of this function, [`UnixString::into_string_unchecked`](UnixString::into_string_unchecked), which has the same behavior but skips the check.
    ///
    /// If the validity check passes, the resulting `String` will reuse the allocation of the `UnixString`'s inner buffer and no copy will be done.
    ///
    /// If you need a `&str` instead of a `String`, consider [`UnixString::as_str`](UnixString::to_str).
    pub fn into_string(self) -> Result<String> {
        Ok(String::from_utf8(self.into_bytes())?)
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
        String::from_utf8_unchecked(self.into_bytes())
    }

    /// This function will check if the inner bytes of this `UnixString` (without its null terminator) consists of valid UTF-8, and if so, returns a string slice without copying.
    ///
    /// If the inner bytes contain invalid UTF-8, then a new `String` will be allocated with the invalid bytes replaced with the Unicode replacement codepoint.
    pub fn to_string_lossy(&self) -> Cow<str> {
        self.as_c_str().to_string_lossy()
    }

    /// Gets the underlying byte view of this `UnixString` *without* the nul terminator.
    /// ```rust
    /// use unixstring::UnixString;
    ///
    /// let bytes = b"abc\0".to_vec();
    /// let unix_string = UnixString::from_bytes(bytes).unwrap();
    ///
    /// assert_eq!(
    ///     unix_string.as_bytes(),
    ///     &[b'a', b'b', b'c']
    /// );
    pub fn as_bytes(&self) -> &[u8] {
        self.inner_without_nul_terminator()
    }

    /// Converts a `UnixString` into an [`OsString`].
    ///
    /// This operation is zero-cost.
    ///
    /// If you need a `&OsStr` instead of an `OsString`, consider [`UnixString::as_os_str`](UnixString::as_os_str).
    pub fn into_os_string(self) -> OsString {
        self.into()
    }

    /// Converts a `UnixString` into a [`PathBuf`].
    ///
    /// This operation is zero-cost.
    ///
    /// If you need a `&Path` instead of a `PathBuf`, consider [`UnixString::as_path`](UnixString::as_path).
    pub fn into_pathbuf(self) -> PathBuf {
        self.into()
    }

    /// Converts a `UnixString` into a [`PathBuf`].
    ///
    /// This operation is zero-cost.
    ///
    /// If you need a `&CStr` instead of a `CString`, consider [`UnixString::as_c_str`](UnixString::as_c_str).
    pub fn into_cstring(self) -> CString {
        self.into()
    }

    /// Gets the underlying byte view of this `UnixString` *including* the nul terminator.
    /// 
    /// ```rust
    /// use unixstring::UnixString;
    ///
    /// let bytes = b"abc\0".to_vec();
    /// let unix_string = UnixString::from_bytes(bytes).unwrap();
    ///
    /// assert_eq!(
    ///     unix_string.as_bytes_with_nul(),
    ///     &[b'a', b'b', b'c', 0]
    /// );
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        &self.inner
    }

    /// Returns the inner representation of a `UnixString`.
    ///
    /// The `UnixString`'s nul terminator byte will be included.
    pub fn into_bytes_with_nul(self) -> Vec<u8> {
        self.inner
    }

    /// Returns the inner representation of a `UnixString` with its nul-terminator removed.
    pub fn into_bytes(self) -> Vec<u8> {
        let mut bytes = self.inner;
        bytes.remove(bytes.len() - 1);
        bytes
    }

    /// Converts a `CString` into an `UnixString`.
    ///
    /// This operation is zero-cost and does not fail.
    ///
    /// This operation fails if the `CString` has any interior zero byte but a zero byte at the last position is acceptable.
    pub fn from_cstring(cstring: CString) -> Self {
        cstring.into()
    }

    /// Converts a `PathBuf` into an `UnixString`.
    ///
    /// A validation will be done to ensure that the `PathBuf` has no interior zero bytes.
    /// Other than that, this operation is zero-cost.
    ///
    /// This operation fails if the `PathBuf` has any interior zero byte but a zero byte at the last position is acceptable.
    pub fn from_pathbuf(pathbuf: PathBuf) -> Result<Self> {
        pathbuf.try_into()
    }

    /// Converts a `String` into an `UnixString`.
    ///
    /// A validation will be done to ensure that the `String` has no interior zero bytes.
    /// Other than that, this operation is zero-cost.
    ///
    /// This operation fails if the `String` has any interior zero byte but a zero byte at the last position is acceptable.
    pub fn from_string(string: String) -> Result<Self> {
        string.try_into()
    }

    /// Converts an `OsString` into an `UnixString`.
    ///
    /// A validation will be done to ensure that the `OsString` has no interior zero bytes.
    /// Other than that, this operation is zero-cost.
    ///
    /// This operation fails if the `OsString` has any interior zero byte but a zero byte at the last position is acceptable.
    pub fn from_os_string(os_string: OsString) -> Result<Self> {
        os_string.try_into()
    }

    /// Checks if the `UnixString` starts with the given slice.
    ///
    /// ```
    /// use unixstring::UnixString;
    /// # use unixstring::Result;
    /// # fn main() -> Result<()> {
    /// let mut unix_string = UnixString::new();
    /// unix_string.push("/home/")?;
    /// unix_string.push("user")?;
    ///
    /// assert!(unix_string.starts_with("/home"));
    /// assert!(unix_string.starts_with("/home/user"));
    /// assert!(!unix_string.starts_with("/home/user/"));
    /// assert!(!unix_string.starts_with("/home/other-user"));
    ///
    ///
    /// # Ok(()) }
    /// ```
    pub fn starts_with(&self, rhs: impl AsRef<OsStr>) -> bool {
        let rhs = rhs.as_ref().as_bytes();
        match self.as_bytes().get(0..rhs.len()) {
            Some(subslice) => subslice == rhs,
            None => false,
        }
    }

    /// Returns an unsafe mutable pointer to the `UnixString`'s buffer.
    ///
    /// # Safety
    ///
    /// * The caller must ensure that the `UnixString` remains valid after modification.
    /// * The caller must ensure that the `UnixString` outlives the pointer this
    /// function returns, or else it ends up pointing to garbage.
    /// * Modifying the vector may cause its buffer to be reallocated,
    /// which would also make any pointers to it invalid.
    ///
    /// If you want to ensure that your `UnixString` is still valid after modified through [`as_mut_ptr`](UnixString::as_mut_ptr),
    /// check out [`UnixString::validate`](UnixString::validate).
    /// 
    /// See also: [`Vec::as_mut_ptr`](std::vec::Vec::as_mut_ptr)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use unixstring::UnixString;
    /// 
    /// let mut unx = UnixString::with_capacity(12);
    ///
    /// let ptr = unx.as_mut_ptr();
    /// 
    /// // This loop mocks a potential FFI call that uses this raw pointer
    /// for (idx, &byte) in b"hello world\0".iter().enumerate() {
    ///     unsafe {
    ///         ptr.add(idx).write(byte as _);
    ///     }
    /// }
    ///
    /// // Once you've written your data, you must set the 
    /// # // TODO: investigate a potential `truncate` function
    /// unsafe {
    ///     unx.set_len(12);
    /// }
    /// 
    /// // The internal representation was not broken
    /// assert!(unx.validate().is_ok());
    /// 
    /// assert!(matches!(unx.to_str(), Ok("hello world")));
    /// 
    /// ```
    pub fn as_mut_ptr(&mut self) -> *mut libc::c_char {
        // self.inner.as_mut_ptr() as *mut libc::c_char
        self.inner.as_mut_ptr() as *mut _
    }

    /// Forces the length of the inner buffer of `self` to `new_len`.
    ///
    /// This method can be useful for situations in which the `UnixString` is serving as a buffer for other code, particularly over FFI.
    /// 
    /// See [`UnixString::as_mut_ptr`](UnixString::as_mut_ptr) for an example usage of this function.
    /// 
    /// 
    /// # Safety
    ///
    /// - `new_len` must be less than or equal to [`capacity()`].
    /// - The elements at `old_len..new_len` must be initialized.
    ///
    /// [`capacity()`]: UnixString::capacity
    /// 
    /// See also: [`Vec::set_len`](Vec::set_len).
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.inner.set_len(new_len)
    }

    /// Returns the number of bytes this `UnixString` can hold without
    /// reallocating.
    /// 
    /// Do note that the nul terminator byte *is* included in this count.
    /// 
    /// ```rust
    /// use unixstring::UnixString;
    /// 
    /// assert_eq!(
    ///     // Capacity to hold 49 bytes + one byte for the nul terminator
    ///     UnixString::with_capacity(49).capacity(),
    ///     50
    /// );
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Returns the length of the underlying byte string *without* considering the nul terminator.
    /// 
    /// ```rust
    /// use unixstring::UnixString;
    /// 
    /// let name = "John Doe";
    /// let unx = UnixString::from_string(name.to_string()).unwrap();
    /// 
    /// assert_eq!(
    ///     name.len(),
    ///     unx.len()
    /// );
    /// 
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len().wrapping_sub(1)
    }

    /// Returns the length of the underlying byte string *considering* the nul terminator.
    ///
    /// ```rust
    /// use unixstring::UnixString;
    /// 
    /// let name = b"John Doe\0";
    /// let unx = UnixString::from_bytes(name.to_vec()).unwrap();
    /// 
    /// assert_eq!(
    ///     name.len(),
    ///     unx.len_with_nul()
    /// );
    /// 
    /// assert_eq!(
    ///     name.len(),
    ///     unx.len() + 1
    /// );
    /// 
    /// ```
    pub fn len_with_nul(&self) -> usize {
        self.inner.len()
    }

    /// Checks if `self` represents an empty byte string.
    /// 
    /// Note that `self` will never really be empty since a `UnixString` always allocates at least one byte
    /// to hold its nul terminator.
    /// 
    /// ```rust
    /// use unixstring::UnixString;
    /// 
    /// # use unixstring::Result;
    /// # fn main() -> Result<()> {
    /// 
    /// let mut unx = UnixString::new();
    /// 
    /// assert!(unx.is_empty());
    /// 
    /// unx.push("123321")?; 
    /// 
    /// assert_eq!(unx.is_empty(), false);
    /// 
    /// # Ok(()) }
    /// 
    /// ```
    pub fn is_empty(&self) -> bool {
        matches!(&*self.inner, &[0])
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
