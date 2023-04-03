#![feature(maybe_uninit_uninit_array, maybe_uninit_slice)]

use crate::git::raw::*;
use crate::git::utils::*;
use crate::git::Result;
use std::ffi::CString;
use std::os::raw::c_char;
use std::{mem, ptr};

mod git;

fn main() -> Result<()> {
    let path_to_repo = std::env::args()
        .skip(1)
        .next()
        .expect("Wait for a path to git repo as arg");
    let path = CString::new(path_to_repo).expect("Invalid path");

    unsafe {
        check("init libgit", git_libgit2_init())?;

        let mut repo = ptr::null_mut();
        check("open repo", git_repository_open(&mut repo, path.as_ptr()))?;

        let ref_name = b"HEAD\0".as_ptr() as *const c_char;
        let oid = {
            let mut oid = mem::MaybeUninit::uninit();
            check(
                "checkout HEAD",
                git_reference_name_to_id(oid.as_mut_ptr(), repo, ref_name),
            )?;
            oid.assume_init()
        };

        let mut buf: [mem::MaybeUninit<u8>; 40] = mem::MaybeUninit::uninit_array();
        check(
            "get commit's sha",
            git_oid_fmt(buf.as_mut_ptr() as *mut c_char, &oid),
        )?;
        let commit_sha = mem::MaybeUninit::slice_assume_init_ref(&buf[..40]);
        println!(
            "commit {}",
            std::string::String::from_utf8_lossy(commit_sha)
        );

        let mut commit = ptr::null_mut();
        check(
            "checkout commit",
            git_commit_lookup(&mut commit, repo, &oid),
        )?;

        print_commit(commit);

        // clean up
        git_commit_free(commit);
        git_repository_free(repo);
        check("fin libgit", git_libgit2_shutdown())?;

        Ok(())
    }
}
