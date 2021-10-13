use unixstring::UnixString;

#[test]
fn into_string_unchecked_one_char() {
    let bytes = vec![b'c'];
    let unix_string = UnixString::from_bytes(bytes).unwrap();

    let string = unsafe { unix_string.into_string_unchecked() };

    assert_eq!(&string, "c")
}

#[test]
fn into_string_unchecked_empty() {
    let bytes = vec![];
    let unix_string = UnixString::from_bytes(bytes).unwrap();

    let string = unsafe { unix_string.into_string_unchecked() };

    assert_eq!(&string, "")
}

#[test]
fn into_string_unchecked() {
    let bytes = b"/usr/bin".to_vec();
    let unix_string = UnixString::from_bytes(bytes).unwrap();

    let string = unsafe { unix_string.into_string_unchecked() };

    assert_eq!(&string, "/usr/bin")
}
