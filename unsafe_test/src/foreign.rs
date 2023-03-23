use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    pub fn strlen(s: *const c_char) -> usize;
    static mut environ: *mut *mut c_char;
}

// print out all environment variables
pub fn print_env_vars() {
    unsafe {
        if environ.is_null() {
            return;
        }

        while !(*environ).is_null() {
            let env = CStr::from_ptr(*environ);
            println!("[DEBUG] Env: {}", env.to_string_lossy());
            environ = environ.offset(1)
        }
    }
}
