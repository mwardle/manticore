// Many things in here are taken from the following project:
// https://github.com/jeremyletang/rust-portaudio/

use libc::{c_char, c_double, c_ulong, c_void};
use std::ffi::{CStr, CString};

/// A function to convert C strings to Rust strings
fn c_str_to_string<'a>(c_str: &'a *const c_char) -> String {
    unsafe {
        String::from_utf8_lossy(CStr::from_ptr(*c_str).to_bytes()).into_owned()
    }
}

/// A function to convert Rust strings to C strings
fn string_to_c_str(rust_str: &String) -> *const c_char {
    match CString::new(rust_str.as_bytes()) {
        Ok(c_string) => c_string.as_ptr(),
        Err(err) => panic!(err),
    }
}