use std::convert::TryFrom;
use std::ffi::CString;
use std::ffi::OsString;
use std::path::PathBuf;

use unixstring::UnixString;

#[test]
fn into_cstring() {
    let home = "/home/user";

    let cstring = CString::new(home).unwrap();
    let unx = UnixString::from(cstring.clone());

    assert_eq!(cstring.as_c_str(), unx.as_c_str());

    assert_eq!(cstring, unx.into_cstring());
}

#[test]
fn into_pathbuf() {
    let home = "/home/user";

    let pathbuf = PathBuf::from(home);
    let unx = UnixString::try_from(pathbuf.clone()).unwrap();

    assert_eq!(pathbuf.as_path(), unx.as_path());

    assert_eq!(pathbuf, unx.into_pathbuf());
}

#[test]
fn into_os_string() {
    let home = "/home/user";

    let os_string = OsString::from(home);
    let unx = UnixString::try_from(os_string.clone()).unwrap();

    assert_eq!(os_string.as_os_str(), unx.as_os_str());

    assert_eq!(os_string, unx.into_os_string());
}
