use std::ffi::CString;

pub struct StaticCString {
    _c_string: CString,
    pub ptr: *const i8,
}

impl StaticCString {
    pub fn new(s: &str) -> Self {
        let c_string = CString::new(s).unwrap();
        let ptr = c_string.as_ptr();
        Self { _c_string: c_string, ptr }
    }
}
