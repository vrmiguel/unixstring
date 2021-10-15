use std::{ffi::CString, path::Path};

use unixstring::UnixString;

#[test]
fn empty_extend_from_ptr() {
    let mut unx = UnixString::new();

    let home = CString::new("/home/vrmiguel").unwrap();

    let ptr = home.as_ptr();

    unsafe { unx.extend_from_ptr(ptr) }.unwrap();

    assert_eq!(home.as_c_str(), unx.as_c_str());
    assert_eq!(home.as_bytes(), unx.as_bytes());
    assert_eq!(b"/home/vrmiguel\0".to_vec(), unx.as_bytes_with_nul());
}

#[test]
fn extend_from_ptr() {
    let mut unx = UnixString::new();

    unx.push("/home/").unwrap();

    assert_eq!(Path::new("/home/"), unx.as_path());
    assert_eq!(b"/home/\0".to_vec(), unx.as_bytes_with_nul());

    let username = CString::new("user").unwrap();

    let ptr = username.as_ptr();

    unsafe { unx.extend_from_ptr(ptr) }.unwrap();

    dbg!(unx.as_path());

    assert_eq!(Path::new("/home/user"), unx.as_path());
    assert_eq!(b"/home/user\0".to_vec(), unx.as_bytes_with_nul());
}
