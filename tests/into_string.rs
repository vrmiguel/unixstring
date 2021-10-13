use unixstring::UnixString;

#[test]
fn into_string_one_char() {
    let bytes = vec![b'c'];
    let unix_string = UnixString::from_bytes(bytes).unwrap();

    let string = unix_string.into_string().unwrap();

    assert_eq!(&string, "c")
}

#[test]
fn into_string_empty() {
    let bytes = vec![];
    let unix_string = UnixString::from_bytes(bytes).unwrap();

    let string = unix_string.into_string().unwrap();

    assert_eq!(&string, "")
}

#[test]
fn into_string() {
    let bytes = b"/usr/bin".to_vec();
    let unix_string = UnixString::from_bytes(bytes).unwrap();

    let string = unix_string.into_string().unwrap();

    assert_eq!(&string, "/usr/bin")
}
