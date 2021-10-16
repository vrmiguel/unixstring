use unixstring::{Result, UnixString};

#[test]
pub fn starts_with() -> Result<()> {
    let mut unix_string = UnixString::new();
    unix_string.push("/home/")?;
    unix_string.push("user")?;

    assert!(unix_string.starts_with("/home"));
    assert!(unix_string.starts_with("/home/user"));
    assert!(!unix_string.starts_with("/home/user/"));
    assert!(!unix_string.starts_with("/home/other-user"));

    Ok(())
}

#[test]
pub fn starts_with_boundaries() -> Result<()> {
    let mut unix_string = UnixString::new();
    unix_string.push("lorem ipsum")?;

    assert!(unix_string.starts_with("lorem"));
    assert!(unix_string.starts_with("lorem ipsum"));

    assert!(!unix_string.starts_with("lorem ipsun"));
    assert!(!unix_string.starts_with("lorem ipsum "));
    assert!(!unix_string.starts_with("lorem ipsumm"));

    Ok(())
}

#[test]
pub fn starts_with_empty() -> Result<()> {
    let mut unix_string = UnixString::new();
    unix_string.push("/home/")?;

    assert!(unix_string.starts_with(""));

    Ok(())
}
