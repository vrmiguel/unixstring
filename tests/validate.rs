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
// A similar test to this one can be found in `as_mut_ptr.rs`, as well as one checking for MissingNulTerminator
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
