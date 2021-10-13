use std::{convert::TryFrom, path::PathBuf};

use unixstring::UnixString;

#[test]
fn pathbuf() {
    let home_dir = PathBuf::from("/home/user");
    let unix_string = UnixString::try_from(home_dir.clone()).unwrap();

    assert_eq!(
        &home_dir,
        unix_string.as_path()
    )
}

#[test]
fn empty_pathbuf() {
    let empty = PathBuf::from("");
    let unix_string = UnixString::try_from(empty.clone()).unwrap();

    assert_eq!(
        &empty,
        unix_string.as_path()
    )   
}