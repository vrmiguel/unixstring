pub fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    let p = unsafe {
        libc::memchr(
            haystack.as_ptr() as *const libc::c_void,
            needle as libc::c_int,
            haystack.len(),
        )
    };
    if p.is_null() {
        None
    } else {
        Some(p as usize - (haystack.as_ptr() as usize))
    }
}

pub fn find_nul_byte(bytes: &[u8]) -> Option<usize> {
    memchr(0, &bytes)
}

#[cfg(test)]
mod tests {

    use super::memchr;

    #[test]
    fn memchr_() {
        let text = "textwithoutnulbytes";
        assert!(memchr(0, text.as_bytes()).is_none());

        let text = "textwithout\0nulbytes";
        dbg!(memchr(0, text.as_bytes()));
        assert!(matches!(memchr(0, text.as_bytes()), Some(11)));
    }
}
