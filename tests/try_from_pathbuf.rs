use std::{convert::TryFrom, path::PathBuf};

use unixstring::UnixString;

#[test]
fn empty_pathbuf() {
    let empty = PathBuf::from("");
    let unix_string = UnixString::try_from(empty.clone()).unwrap();

    assert_eq!(empty.as_path(), unix_string.as_path())
}

#[test]
fn size_one_pathbuf() {
    let one = PathBuf::from("1");
    let unix_string = UnixString::try_from(one.clone()).unwrap();

    assert_eq!(one.as_path(), unix_string.as_path())
}

#[test]
fn pathbuf() {
    let logs = PathBuf::from("/var/log/journal");
    let unix_string = UnixString::try_from(logs.clone()).unwrap();

    assert_eq!(logs.as_path(), unix_string.as_path())
}

#[test]
fn pathbuf_with_interior_nul_byte_fails() {
    let home = PathBuf::from("/home\0/user");
    assert!(UnixString::try_from(home).is_err());
}
