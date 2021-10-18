use unixstring::UnixString;

#[test]
fn empty_vec() {
    let empty = UnixString::from_bytes(vec![]).unwrap();

    assert_eq!(empty.to_str().unwrap(), "");

    assert_eq!(empty.as_bytes(), &[]);

    assert_eq!(empty.as_bytes_with_nul(), &[0]);
}

#[test]
fn size_one_vec() {
    let one = UnixString::from_bytes(vec![b'1']).unwrap();

    assert_eq!(one.to_str().unwrap(), "1");

    assert_eq!(one.as_bytes(), &[b'1']);

    assert_eq!(one.as_bytes_with_nul(), &[b'1', 0]);
}

#[test]
fn valid_bytes() {
    let abc = UnixString::from_bytes(b"abc".to_vec()).unwrap();

    assert_eq!(abc.to_str().unwrap(), "abc");

    assert_eq!(abc.as_bytes(), b"abc".to_vec().as_slice());

    assert_eq!(abc.as_bytes_with_nul(), b"abc\0".to_vec().as_slice());
}

#[test]
fn invalid_bytes_fails() {
    let abc = UnixString::from_bytes(b"a\0bc".to_vec());

    assert!(abc.is_err())
}

#[test]
fn from_bytes() {
    let bytes_without_zero = vec![b'a', b'b', b'c'];
    let bytes_with_nul_terminator = vec![b'a', b'b', b'c', 0];
    let bytes_with_interior_nul = vec![b'a', 0, b'b', b'c'];

    // Valid: no zero bytes were given
    let unx = UnixString::from_bytes(bytes_without_zero).unwrap();
    assert_eq!(unx.to_str().unwrap(), "abc");
    assert_eq!(unx.as_bytes(), b"abc".to_vec());
    assert_eq!(unx.as_bytes_with_nul(), b"abc\0".to_vec());

    // Still valid: the zero byte is being used as the terminator
    let unx = UnixString::from_bytes(bytes_with_nul_terminator).unwrap();
    assert_eq!(unx.to_str().unwrap(), "abc");
    assert_eq!(unx.as_bytes(), b"abc".to_vec());
    assert_eq!(unx.as_bytes_with_nul(), b"abc\0".to_vec());

    // Invalid: an interior nul byte was found
    UnixString::from_bytes(bytes_with_interior_nul).unwrap_err();
}
