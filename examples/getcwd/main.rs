use libc::{c_char, getcwd};
use unixstring::UnixString;

fn main() {
    const PATH_SIZ: usize = 1024;
    let mut buf: [c_char; 1024] = [0; 1024];

    let ptr = &mut buf as *mut c_char;

    unsafe { getcwd(ptr, PATH_SIZ) };

    if ptr.is_null() {
        panic!("getcwd failed");
    }

    let unix_string = unsafe { UnixString::from_ptr(ptr as *const c_char) };

    assert_eq!(unix_string.as_path(), std::env::current_dir().unwrap())
}
