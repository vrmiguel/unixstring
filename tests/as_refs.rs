use std::{convert::TryFrom, ffi::{CStr, CString, OsStr}, path::{Path, PathBuf}};

use unixstring::UnixString;

#[test]
fn as_ref_path() {
    let home = "home/user/";
    let pathbuf = PathBuf::from(&home);

    let unix_string = UnixString::try_from(home.to_owned()).unwrap();
    let path: &Path = unix_string.as_ref();

    assert_eq!(pathbuf.as_path(), path);
}

#[test]
fn as_ref_os_str() {
    let home = "home/user/";
    let pathbuf = PathBuf::from(&home);

    let unix_string = UnixString::try_from(home.to_owned()).unwrap();
    let os_str: &OsStr = unix_string.as_ref();

    assert_eq!(pathbuf.as_os_str(), os_str);
}

#[test]
fn as_ref_cstr() {
    let home = CString::new("home/user/").unwrap();

    let unix_string = UnixString::try_from(home.clone()).unwrap();
    let unix_string_cstr: &CStr = unix_string.as_ref();

    assert_eq!(home.as_c_str(), unix_string_cstr);
}
