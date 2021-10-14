use std::{
    convert::TryFrom,
    ffi::{CStr, CString},
    path::PathBuf,
};

use unixstring::UnixString;

#[test]
fn as_ptr() {
    const HOME: &str = "/home/user";
    let home = PathBuf::from(HOME);
    let home = UnixString::try_from(home).unwrap();

    let ptr = home.as_ptr();
    let cstr = unsafe { CStr::from_ptr(ptr) };

    assert_eq!(cstr.to_str().unwrap(), HOME);

    assert_eq!(cstr, home.as_c_str())
}

#[test]
fn from_ptr() {
    let home = CString::new("/home/vrmiguel").unwrap();

    let ptr = home.as_ptr();

    let unix_string = unsafe { UnixString::from_ptr(ptr) };

    assert_eq!(home.as_c_str(), unix_string.as_c_str())
}
