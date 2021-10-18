use unixstring::UnixString;

#[test]
fn is_empty() {
    let mut unx = UnixString::new();
    
    assert!(unx.is_empty());

    unx.push("\0").unwrap(); 

    assert!(unx.is_empty());
    assert_eq!(unx.len(), 0);
    assert_eq!(unx.len_with_nul(), 1);
    assert_eq!(unx.capacity(), 1);
    
    unx.push("123321").unwrap(); 
    
    assert_eq!(unx.is_empty(), false);
    assert_eq!(unx.len(), 6);
    assert_eq!(unx.len_with_nul(), 7);
}

#[test]
fn len_and_cap() {
    let name = b"John Doe\0";
    let unx = UnixString::from_bytes(name.to_vec()).unwrap();
    
    assert_eq!(
        name.len(),
        unx.len_with_nul()
    );
    
    assert_eq!(
        name.len(),
        unx.len() + 1
    );

    assert_eq!(
        name.len(),
        unx.capacity()
    );
}