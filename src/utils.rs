use std::ffi::CString;

pub fn c_string_ptr(s: &str) -> *const i8 {
    let c_string = CString::new(s).unwrap();
    c_string.as_ptr()
}
