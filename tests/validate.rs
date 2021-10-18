use std::{ffi::CString, path::PathBuf};

use unixstring::UnixString;

#[test]
fn valid() {
    let logs = PathBuf::from("/var/log");
    let mut logs = UnixString::from_pathbuf(logs).unwrap();

    assert!(logs.validate().is_ok());

    logs.push("bustd.out").unwrap();

    assert!(logs.validate().is_ok());

    logs.push("\0").unwrap();

    assert!(logs.validate().is_ok());
}

#[test]
fn invalid_interior_nul_byte() {
    let with_interior_nul_byte = b"hello\0world".to_vec();

    // Constructing an invalid CString from an unsafe method
    let c = unsafe { CString::from_vec_unchecked(with_interior_nul_byte) };

    let invalid_unix_strings = UnixString::from_cstring(c);

    assert!(matches!(
        invalid_unix_strings.validate(),
        Err(unixstring::Error::InteriorNulByte)
    ))
}

// TODO: finish this test
// #[test]
// fn invalid_missing_nul_terminator() {
//     let valid = b"hello world".to_vec();

//     // Constructing an invalid CString from an unsafe method
//     let mut invalid_unix_strings = UnixString::from_bytes(valid).unwrap();

//     let cap = invalid_unix_strings.capacity();

//     let mut_ptr = unsafe { invalid_unix_strings.as_mut_ptr() };

//     unsafe { mut_ptr.add(count) }

//     let mut vec = unsafe { Vec::from_raw_parts(mut_ptr, cap, cap) };

//     vec.remove(vec.len() - 1);

//     dbg!(invalid_unix_strings.validate());
// }