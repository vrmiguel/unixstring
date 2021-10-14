use unixstring::UnixString;

#[test]
fn from_bytes() {
    let bytes_without_zero = vec![b'a', b'b', b'c'];
    let bytes_with_nul_terminator = vec![b'a', b'b', b'c', 0];
    let bytes_with_interior_nul = vec![b'a', 0, b'b', b'c'];

    // Valid: no zero bytes were given
    let unx = UnixString::from_bytes(bytes_without_zero).unwrap();
    assert_eq!(unx.as_str().unwrap(), "abc");
    assert_eq!(unx.as_bytes(), b"abc".to_vec());
    assert_eq!(unx.as_bytes_with_nul(), b"abc\0".to_vec());

    // Still valid: the zero byte is being used as the terminator
    let unx = UnixString::from_bytes(bytes_with_nul_terminator).unwrap();
    assert_eq!(unx.as_str().unwrap(), "abc");
    assert_eq!(unx.as_bytes(), b"abc".to_vec());
    assert_eq!(unx.as_bytes_with_nul(), b"abc\0".to_vec());

    // Invalid: an interior nul byte was found
    UnixString::from_bytes(bytes_with_interior_nul).unwrap_err();
}
