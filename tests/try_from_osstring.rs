use std::{convert::TryFrom, ffi::OsString, path::PathBuf};

use unixstring::UnixString;

#[test]
fn empty_os_string() {
    let empty = OsString::from("");
    let unix_string = UnixString::try_from(empty.clone()).unwrap();

    assert_eq!(empty.as_os_str(), unix_string.as_os_str())
}

#[test]
fn size_one_os_string() {
    let one = OsString::from("1");
    let unix_string = UnixString::try_from(one.clone()).unwrap();

    assert_eq!(one.as_os_str(), unix_string.as_os_str())
}

#[test]
fn os_string() {
    let logs = OsString::from("/var/log/journal");
    let unix_string = UnixString::try_from(logs.clone()).unwrap();

    assert_eq!(logs.as_os_str(), unix_string.as_os_str())
}
