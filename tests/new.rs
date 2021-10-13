use std::{ffi::OsStr, path::Path};

use unixstring::UnixString;

fn check_empty(empty: UnixString) {
    assert_eq!(empty.as_str().ok(), Some(""));
    assert_eq!(empty.as_os_str(), OsStr::new(""));
    assert_eq!(empty.as_path(), Path::new(""));
    assert_eq!(empty.as_bytes(), &[]);
    assert_eq!(empty.as_bytes_with_nul(), &[0]);
}

#[test]
fn new() {
    // Sanity-check that empty UnixStrings work as intended
    let empty = UnixString::new();

    check_empty(empty);
}

#[test]
fn with_capacity() {
    // Sanity-check that empty UnixStrings work as intended
    check_empty(UnixString::with_capacity(5));
    check_empty(UnixString::with_capacity(0));
}
