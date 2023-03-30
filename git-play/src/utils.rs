use crate::raw;
use std::ffi::CStr;
use std::os::raw::c_int;
use std::process::exit;

pub fn check(msg: &'static str, exit_status: c_int) -> c_int {
    if exit_status < 0 {
        unsafe {
            let err = &*raw::giterr_last();
            println!(
                "Error occurred when {}: {} [{}]",
                msg,
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

    let msg = raw::git_commit_message(commit);
    let content = CStr::from_ptr(msg).to_string_lossy();

    println!("Author: {} <{}>\n\n{}\n", name, email, content);
}
