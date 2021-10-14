use std::borrow::Cow;

use unixstring::UnixString;

#[test]
fn to_string_lossy_owned() {
    let invalid_utf8 = vec![189, 178, 61, 188, 33, 226, 140, 153];

    // Making sure these bytes are not valid UTF-8
    assert!(std::str::from_utf8(&invalid_utf8).is_err());

    let unx = UnixString::from_bytes(invalid_utf8).unwrap();

    // to_string_lossy has to return a new string since the supplied bytes weren't valid UTF-8
    assert!(matches!(unx.to_string_lossy(), Cow::Owned(_)));
}

#[test]
fn to_string_lossy_borrowed() {
    let valid_utf8 = vec![0xF0, 0x9F, 0x91, 0xB6];

    // Making sure these bytes are valid UTF-8
    assert!(std::str::from_utf8(&valid_utf8).is_ok());

    let unx = UnixString::from_bytes(valid_utf8).unwrap();

    // to_string_lossy has to return a new string since the supplied bytes weren't valid UTF-8
    assert!(matches!(unx.to_string_lossy(), Cow::Borrowed("👶")));
}

#[test]
fn into_string_lossy() {
    let valid_utf8 = vec![0xF0, 0x9F, 0x91, 0xB6];

    // Making sure these bytes are valid UTF-8
    assert!(std::str::from_utf8(&valid_utf8).is_ok());

    let unx = UnixString::from_bytes(valid_utf8).unwrap();

    // to_string_lossy has to return a new string since the supplied bytes weren't valid UTF-8
    assert_eq!(unx.into_string_lossy(), "👶");
}
