use unixstring::UnixString;

#[test]
fn from_bytes() {
    let bytes_without_zero = vec![b'a', b'b', b'c'];
    let bytes_with_nul_terminator = vec![b'a', b'b', b'c', 0];
    let bytes_with_interior_nul = vec![b'a', 0, b'b', b'c'];

    // Valid: no zero bytes were given
    assert!(UnixString::from_bytes(bytes_without_zero).is_ok());

    // Still valid: the zero byte is being used as the terminator
    assert!(UnixString::from_bytes(bytes_with_nul_terminator).is_ok());

    // Invalid: an interior nul byte was found
    assert!(UnixString::from_bytes(bytes_with_interior_nul).is_err());
}
