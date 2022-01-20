use unixstring::{Error, UnixString};

#[test]
fn writing_to_mut_ptr() {
    let mut unx = UnixString::with_capacity(12);

    let ptr = unx.as_mut_ptr();

    for (idx, &byte) in b"hello world\0".iter().enumerate() {
        unsafe {
            ptr.add(idx).write(byte as _);
        }
    }
    unsafe {
        unx.set_len(12);
    }

    assert!(unx.validate().is_ok());
}

#[test]
fn as_mut_ptr_invalidating_nul_terminator() {
    let mut unx = UnixString::with_capacity(50);

    let ptr = unx.as_mut_ptr();

    unsafe { ptr.write_bytes(5, 51) };

    assert!(matches!(unx.validate(), Err(Error::MissingNulTerminator)));
}

#[test]
fn as_mut_ptr_invalidating_interior_nul_byte() {
    let aaa = std::iter::repeat(b'a');
    let mut unx = UnixString::from_bytes(aaa.take(50).collect()).unwrap();

    let ptr = unx.as_mut_ptr();

    // Invalidate the UnixString by adding an interior nul byte
    unsafe { ptr.add(5).write(0) }

    assert!(matches!(unx.validate(), Err(Error::InteriorNulByte)));
}
