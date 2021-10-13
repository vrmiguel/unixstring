use std::{convert::TryFrom, path::PathBuf};

use unixstring::UnixString;

#[test]
fn empty_os_string() {
    let empty = PathBuf::from("");
    let unix_string = UnixString::try_from(empty.clone()).unwrap();

    assert_eq!(empty.as_os_str(), unix_string.as_os_str())
}

#[test]
fn size_one_os_string() {
    let one = PathBuf::from("1");
    let unix_string = UnixString::try_from(one.clone()).unwrap();

    assert_eq!(one.as_os_str(), unix_string.as_os_str())
}

#[test]
fn os_string() {
    let logs = PathBuf::from("/var/log/journal");
    let unix_string = UnixString::try_from(logs.clone()).unwrap();

    assert_eq!(logs.as_os_str(), unix_string.as_os_str())
}

#[test]
fn pathbuf() {
    let home_dir = PathBuf::from("/home/user");
    let unix_string = UnixString::try_from(home_dir.clone()).unwrap();

    assert_eq!(&home_dir, unix_string.as_path())
}

#[test]
fn empty_pathbuf() {
    let empty = PathBuf::from("");
    let unix_string = UnixString::try_from(empty.clone()).unwrap();

    assert_eq!(&empty, unix_string.as_path())
}
