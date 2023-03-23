use std::os::raw::c_char;

extern "C" {
    pub fn strlen(s: *const c_char) -> usize;
}
