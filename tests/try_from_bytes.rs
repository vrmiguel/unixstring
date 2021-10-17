use std::convert::TryFrom;

use unixstring::UnixString;

#[test]
fn empty_vec() {
    let empty = UnixString::try_from(vec![]).unwrap();

    assert_eq!(empty.to_str().unwrap(), "");

    assert_eq!(empty.as_bytes(), &[]);

    assert_eq!(empty.as_bytes_with_nul(), &[0]);
}

#[test]
fn size_one_vec() {
    let one = UnixString::try_from(vec![b'1']).unwrap();

    assert_eq!(one.to_str().unwrap(), "1");

    assert_eq!(one.as_bytes(), &[b'1']);

    assert_eq!(one.as_bytes_with_nul(), &[b'1', 0]);
}

#[test]
fn valid_bytes() {
    let abc = UnixString::try_from(b"abc".to_vec()).unwrap();

    assert_eq!(abc.to_str().unwrap(), "abc");

    assert_eq!(abc.as_bytes(), b"abc".to_vec().as_slice());

    assert_eq!(abc.as_bytes_with_nul(), b"abc\0".to_vec().as_slice());
}

#[test]
fn invalid_bytes_fails() {
    let abc = UnixString::try_from(b"a\0bc".to_vec());

    assert!(abc.is_err())
}
