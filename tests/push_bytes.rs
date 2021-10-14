use unixstring::UnixString;

#[test]
// This test makes sure that pushing empty bytes doesn't mess up the location of the terminator zero byte.
fn push_empty_bytes() {
    let mut unxstr = UnixString::new();

    unxstr.push_bytes(&[]).unwrap();

    assert_eq!(unxstr.as_bytes_with_nul(), &[0]);

    unxstr.push_bytes(&[]).unwrap();
    unxstr.push_bytes(&[]).unwrap();

    assert_eq!(unxstr.as_bytes_with_nul(), &[0]);

    let abc = b"abc".to_vec();

    unxstr.push_bytes(&abc).unwrap();

    assert_eq!(unxstr.as_bytes_with_nul(), &[b'a', b'b', b'c', 0]);

    unxstr.push_bytes(&[]).unwrap();

    assert_eq!(unxstr.as_bytes_with_nul(), &[b'a', b'b', b'c', 0]);
}

#[test]
fn push_bytes() {
    let abc = b"abc".to_vec();
    let cde = b"cde".to_vec();

    let mut unxstr = UnixString::new();

    unxstr.push_bytes(&abc).unwrap();

    assert_eq!(unxstr.as_bytes_with_nul(), &b"abc\0".to_vec());

    unxstr.push_bytes(&cde).unwrap();

    assert_eq!(unxstr.as_bytes_with_nul(), &b"abccde\0".to_vec());
}

#[test]
fn push_byte_failure_does_not_alter_the_unix_string() {
    let abc = b"abc".to_vec();
    let a0bc = b"a\0bc".to_vec();

    let mut unxstr = UnixString::new();

    unxstr.push_bytes(&abc).unwrap();

    assert_eq!(unxstr.as_bytes_with_nul(), &b"abc\0".to_vec());

    // This one will fail, but what we want to see is if this alters the UnixString at all
    unxstr.push_bytes(&a0bc).unwrap_err();

    // This assertion must once again pass since a failed `push_bytes` must not leave us with a broken UnixString
    assert_eq!(unxstr.as_bytes_with_nul(), &b"abc\0".to_vec());
}

#[test]
fn push_byte_fails_with_interior_zero_bytes() {
    let a0bc = b"a\0bc".to_vec();

    let mut unxstr = UnixString::new();
    unxstr.push_bytes(&a0bc).unwrap_err();
}

#[test]
fn push_null_terminated_bytes() {
    let abc = b"abc".to_vec();
    let abc2 = b"abc\0".to_vec();
    let cde = b"cde\0".to_vec();

    let mut unxstr = UnixString::new();

    unxstr.push_bytes(&abc).unwrap();
    assert_eq!(unxstr.as_bytes_with_nul(), &[b'a', b'b', b'c', 0]);

    unxstr.push_bytes(&abc2).unwrap();
    assert_eq!(unxstr.as_bytes_with_nul(), &b"abcabc\0".to_vec());

    unxstr.push_bytes(&cde).unwrap();
    assert_eq!(unxstr.as_bytes_with_nul(), &b"abcabccde\0".to_vec());
}
