use std::convert::TryFrom;

use unixstring::UnixString;

#[test]
fn string_with_interior_nul_byte_fails() {
    let string = String::from("/home\0/user");
    assert!(UnixString::try_from(string).is_err());
}

#[test]
fn string_with_terminator_nul_byte() {
    let string = String::from("/home/user\0");
    assert!(UnixString::try_from(string).is_ok());
}

#[test]
fn empty_string() {
    let empty = String::from("");
    let unix_string = UnixString::try_from(empty.clone()).unwrap();

    assert_eq!(empty.as_str(), unix_string.to_str().unwrap());
    assert!(unix_string.is_empty())
}

#[test]
fn size_one_string() {
    let one = String::from("1");
    let unix_string = UnixString::try_from(one.clone()).unwrap();

    assert_eq!(one.as_str(), unix_string.to_str().unwrap())
}

#[test]
fn string() {
    let logs = String::from("/var/log/journal");
    let unix_string = UnixString::try_from(logs.clone()).unwrap();

    assert_eq!(logs.as_str(), unix_string.to_str().unwrap())
}
