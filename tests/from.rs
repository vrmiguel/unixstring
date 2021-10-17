use std::{
    ffi::{CString, OsString},
    path::PathBuf,
};

use unixstring::UnixString;

#[test]
fn empty() {
    let empty = OsString::from("");
    let unix_string = UnixString::from_os_string(empty.clone()).unwrap();
    assert_eq!(empty.as_os_str(), unix_string.as_os_str());

    let empty = PathBuf::from("");
    let unix_string = UnixString::from_pathbuf(empty.clone()).unwrap();
    assert_eq!(empty.as_path(), unix_string.as_path());

    let empty = CString::new("").unwrap();
    let unix_string = UnixString::from_cstring(empty.clone());
    assert_eq!(empty.as_c_str(), unix_string.as_c_str());

    let empty = String::from("");
    let unix_string = UnixString::from_string(empty.clone()).unwrap();
    assert_eq!(empty.as_str(), unix_string.to_str().unwrap());
}

#[test]
fn size_one() {
    let one = OsString::from("1");
    let unix_string = UnixString::from_os_string(one.clone()).unwrap();
    assert_eq!(one.as_os_str(), unix_string.as_os_str());

    let one = PathBuf::from("1");
    let unix_string = UnixString::from_pathbuf(one.clone()).unwrap();
    assert_eq!(one.as_path(), unix_string.as_path());

    let one = CString::new("1").unwrap();
    let unix_string = UnixString::from_cstring(one.clone());
    assert_eq!(one.as_c_str(), unix_string.as_c_str());

    let one = String::from("1");
    let unix_string = UnixString::from_string(one.clone()).unwrap();
    assert_eq!(one.as_str(), unix_string.to_str().unwrap());
}

#[test]
fn valid_bytes() {
    let abc = OsString::from("abc");
    let unix_string = UnixString::from_os_string(abc.clone()).unwrap();
    assert_eq!(abc.as_os_str(), unix_string.as_os_str());

    let abc = PathBuf::from("abc");
    let unix_string = UnixString::from_pathbuf(abc.clone()).unwrap();
    assert_eq!(abc.as_path(), unix_string.as_path());

    let abc = CString::new("abc").unwrap();
    let unix_string = UnixString::from_cstring(abc.clone());
    assert_eq!(abc.as_c_str(), unix_string.as_c_str());

    let abc = String::from("abc");
    let unix_string = UnixString::from_string(abc.clone()).unwrap();
    assert_eq!(abc.as_str(), unix_string.to_str().unwrap());
}

#[test]
fn invalid_bytes() {
    let abc = OsString::from("a\0bc");
    UnixString::from_os_string(abc.clone()).unwrap_err();

    let abc = PathBuf::from("a\0bc");
    UnixString::from_pathbuf(abc.clone()).unwrap_err();

    let abc = String::from("a\0bc");
    UnixString::from_string(abc.clone()).unwrap_err();
}
