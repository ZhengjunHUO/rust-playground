use super::{raw, Error, Result};
//use chrono::{DateTime, Local};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::Path;
//use std::process::exit;
//use std::time::{Duration, UNIX_EPOCH};

pub fn check(msg: &'static str, exit_status: c_int) -> Result<c_int> {
    if exit_status >= 0 {
        return Ok(exit_status);
    }

    unsafe {
        // retrieve git_err structure for the latest error details
        let err = &*raw::giterr_last();

        let content = format!(
            "Error occurred when {}: {} [{}]",
            msg,
            CStr::from_ptr(err.message).to_string_lossy(),
            err.klass
        );

        Err(Error {
            code: exit_status as i32,
            message: content,
            klass: err.klass as i32,
        })
    }
}

/*
pub fn check(msg: &'static str, exit_status: c_int) -> c_int {
    if exit_status < 0 {
        unsafe {
            // retrieve git_err structure for the latest error details
            let err = &*raw::giterr_last();
            println!(
                "Error occurred when {}: {} [{}]",
                msg,
                // wrap a raw "*const u8" to a "representation of a borrowed C string"
                // then convert to a Cow<str>
                CStr::from_ptr(err.message).to_string_lossy(),
                err.klass
            );
            exit(1);
        }
    }

    exit_status
}

pub unsafe fn print_commit(commit: *const raw::git_commit) {
    let author = raw::git_commit_author(commit);
    let name = CStr::from_ptr((*author).name).to_string_lossy();
    let email = CStr::from_ptr((*author).email).to_string_lossy();

    let date =
        DateTime::<Local>::from(UNIX_EPOCH + Duration::from_secs((*author).when.time as u64));

    let msg = raw::git_commit_message(commit);
    let content = CStr::from_ptr(msg).to_string_lossy();

    println!(
        "Author: {} <{}>\nDate:   {}\n\n{}\n",
        name,
        email,
        date.format("%a %b  %d %H:%M:%S %Y %z").to_string(),
        content
    );
}
*/

#[cfg(unix)]
pub fn path_to_cstring(path: &Path) -> Result<CString> {
    use std::os::unix::ffi::OsStrExt;
    Ok(CString::new(path.as_os_str().as_bytes())?)
}

//pub unsafe fn ptr_char_to_str<'a, T: 'a>(_owner: &'a T, ptr_char: *const c_char) -> Option<&'a str> {
pub unsafe fn ptr_char_to_str<T>(_owner: &T, ptr_char: *const c_char) -> Option<&str> {
    if ptr_char.is_null() {
        return None;
    }

    CStr::from_ptr(ptr_char).to_str().ok()
}
