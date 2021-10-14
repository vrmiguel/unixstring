use unixstring::UnixString;



#[test]
fn push_empty_str() {
    let mut unx = UnixString::new();

    unx.push("").unwrap();

    assert_eq!(unx.as_bytes(), &[]);
    assert_eq!(unx.as_bytes_with_nul(), &[0]);
    assert_eq!(unx.as_str().unwrap(), "");

    unx.push("\0").unwrap();

    assert_eq!(unx.as_bytes(), &[]);
    assert_eq!(unx.as_bytes_with_nul(), &[0]);
    assert_eq!(unx.as_str().unwrap(), "");

    unx.push("abc").unwrap();

    assert_eq!(unx.as_bytes(), b"abc".to_vec());
    assert_eq!(unx.as_bytes_with_nul(), b"abc\0".to_vec());
    assert_eq!(unx.as_str().unwrap(), "abc");

    unx.push("\0").unwrap();

    assert_eq!(unx.as_bytes(), b"abc".to_vec());
    assert_eq!(unx.as_bytes_with_nul(), b"abc\0".to_vec());
    assert_eq!(unx.as_str().unwrap(), "abc");

    unx.push("").unwrap();

    assert_eq!(unx.as_bytes(), b"abc".to_vec());
    assert_eq!(unx.as_bytes_with_nul(), b"abc\0".to_vec());
    assert_eq!(unx.as_str().unwrap(), "abc");
}


#[test]
fn push_str() {
    let mut unx = UnixString::new();
    unx.push("abc").unwrap();

    assert_eq!(unx.as_bytes(), b"abc".to_vec());
    assert_eq!(unx.as_bytes_with_nul(), b"abc\0".to_vec());
    assert_eq!(unx.as_str().unwrap(), "abc");

    // Valid: has a zero byte but it's at the end
    unx.push("d\0").unwrap();

    assert_eq!(unx.as_bytes(), b"abcd".to_vec());
    assert_eq!(unx.as_bytes_with_nul(), b"abcd\0".to_vec());
    assert_eq!(unx.as_str().unwrap(), "abcd");

    // Should fail: value has an interior nul byte
    unx.push("\0d\0").unwrap_err();

    // We then reassert these things to make sure that a failed push did not alter our UnixString
    assert_eq!(unx.as_bytes(), b"abcd".to_vec());
    assert_eq!(unx.as_bytes_with_nul(), b"abcd\0".to_vec());
    assert_eq!(unx.as_str().unwrap(), "abcd");
}