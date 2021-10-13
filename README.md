# unixstring

`UnixString` is an FFI-friendly null-terminated byte string that may be constructed from a [`String`], a [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html), a [`PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html), an [`OsString`](https://doc.rust-lang.org/std/ffi/struct.OsString.html) or a collection of bytes.

An [`UnixString`](UnixString) can then be converted into a slice of [`CStr`](https://doc.rust-lang.org/std/ffi/struct.CStr.html), [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html) or [`OsStr`](https://doc.rust-lang.org/std/ffi/struct.OsStr.html) in infallible and zero-cost operations.

## Example

```rust
use std::{convert::TryFrom, env};

use unixstring::UnixString;

fn stat(path: &UnixString) -> std::io::Result<libc::stat> {
    // Safety: The all-zero byte-pattern is a valid `struct stat`
    let mut stat_buf = unsafe { std::mem::zeroed() };

    if -1 == unsafe { libc::lstat(path.as_ptr(), &mut stat_buf) } {
        let io_err = std::io::Error::last_os_error();
        Err(io_err)
    } else {
        Ok(stat_buf)
    }
}


fn main() -> std::io::Result<()>{
    for arg in env::args_os().map(UnixString::try_from).flatten() {
        let stat = stat(&arg)?;
        
        let size = stat.st_size;

        println!("{} occupies {} bytes.", arg.as_path().display(), size);
    }

    Ok(())
}
```