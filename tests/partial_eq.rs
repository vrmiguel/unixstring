use std::{
    ffi::{CStr, CString, OsStr},
    path::Path,
};

use unixstring::UnixString;

#[test]
fn partial_eq_str() {
    fn assert_equal(string: &str, unix: UnixString) {
        assert!(string == unix);
        assert!(unix == string);
        assert!(unix.eq(&string));
        assert!((&string).eq(&unix));
    }

    let lipsum = "lorem ipsum";
    let lipsum_unx = UnixString::from_string(lipsum.into()).unwrap();

    assert_equal(lipsum, lipsum_unx);

    let empty_unx = UnixString::new();
    assert_equal("", empty_unx);

    let hello_world = "hello_world";
    let hello_worl_unx = UnixString::from_string("hello_worl".into()).unwrap();
    assert!(hello_world != hello_worl_unx);

    let hello_worldd_unx = UnixString::from_string("hello_worldd".into()).unwrap();
    assert!(hello_world != hello_worldd_unx);
}

#[test]
fn partial_eq_os_str() {
    fn assert_equal(string: &OsStr, unix: UnixString) {
        assert!(string == unix);
        assert!(unix == string);
        assert!(unix.eq(&string));
        assert!((&string).eq(&unix));
    }

    let lipsum = OsStr::new("lorem ipsum");
    let lipsum_unx = UnixString::from_os_string(lipsum.into()).unwrap();

    assert_equal(lipsum, lipsum_unx);

    let empty_unx = UnixString::new();
    assert_equal(OsStr::new(""), empty_unx);

    let hello_world = OsStr::new("hello_world");
    let hello_worl_unx = UnixString::from_string("hello_worl".into()).unwrap();
    assert!(hello_world != hello_worl_unx);

    let hello_worldd_unx = UnixString::from_string("hello_worldd".into()).unwrap();
    assert!(hello_world != hello_worldd_unx);
}

#[test]
fn partial_eq_c_str() {
    fn assert_equal(string: &CStr, unix: UnixString) {
        assert!(string == unix);
        assert!(unix == string);
        assert!(unix.eq(&string));
        assert!((&string).eq(&unix));
    }

    let lipsum = CString::new("lorem ipsum").unwrap();
    let lipsum_unx = UnixString::from_cstring(lipsum.clone());

    assert_equal(&lipsum, lipsum_unx);

    let empty_unx = UnixString::new();
    assert_equal(&CString::new("").unwrap(), empty_unx);

    let hello_world = CString::new("hello_world").unwrap();
    let hello_worl_unx = UnixString::from_string("hello_worl".into()).unwrap();
    assert!(&*hello_world != hello_worl_unx);

    let hello_worldd_unx = UnixString::from_string("hello_worldd".into()).unwrap();
    assert!(&*hello_world != hello_worldd_unx);
}

#[test]
fn partial_eq_path() {
    fn assert_equal(string: &Path, unix: UnixString) {
        assert!(string == unix);
        assert!(unix == string);
        assert!(unix.eq(&string));
        assert!((&string).eq(&unix));
    }

    let lipsum = Path::new("lorem ipsum");
    let lipsum_unx = UnixString::from_pathbuf(lipsum.into()).unwrap();

    assert_equal(&lipsum, lipsum_unx);

    let empty_unx = UnixString::new();
    assert_equal(Path::new(""), empty_unx);

    let hello_world = Path::new("hello_world");
    let hello_worl_unx = UnixString::from_string("hello_worl".into()).unwrap();
    assert!(&*hello_world != hello_worl_unx);

    let hello_worldd_unx = UnixString::from_string("hello_worldd".into()).unwrap();
    assert!(&*hello_world != hello_worldd_unx);
}
