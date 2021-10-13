use std::{
    convert::TryFrom,
    ffi::{CStr, OsStr},
    os::unix::prelude::OsStrExt,
    path::PathBuf,
};

use libc::{c_char, getcwd, PATH_MAX};
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
    const PATH_SIZ: usize = 1024;
    let mut buf: [c_char; 1024] = [0; 1024];

    let ptr = &mut buf as *mut c_char;

    unsafe { getcwd(ptr, PATH_SIZ) };

    if ptr.is_null() {
        panic!("getcwd failed");
    }

    let unix_string = unsafe { UnixString::from_ptr(ptr as *const c_char) };

    assert_eq!(unix_string.as_path(), std::env::current_dir().unwrap())
}
