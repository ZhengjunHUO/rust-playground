pub mod raw;
pub mod utils;

use libc;
use std::{
    error,
    ffi::{CString, NulError},
    fmt::{self, Display, Formatter},
    marker::PhantomData,
    mem,
    os::raw::c_char,
    path::Path,
    process, ptr, result,
    sync::Once,
};

#[derive(Debug)]
pub struct Error {
    code: i32,
    klass: i32,
    message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        self.message.fmt(f)
    }
}

impl From<String> for Error {
    fn from(message: String) -> Error {
        Error {
            code: -1,
            message,
            klass: 0,
        }
    }
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Error {
        Error {
            code: -1,
            message: err.to_string(),
            klass: 0,
        }
    }
}

impl error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

pub struct Oid {
    pub raw: raw::git_oid,
}

pub struct Commit<'r> {
    raw_ptr: *mut raw::git_commit,
    _lifetime_holder: PhantomData<&'r Repo>,
}

impl<'r> Drop for Commit<'r> {
    fn drop(&mut self) {
        unsafe {
            raw::git_commit_free(self.raw_ptr);
        }
    }
}

// wrapper the raw git_repository, providing a safe interface
pub struct Repo {
    raw_ptr: *mut raw::git_repository,
}

impl Repo {
    pub fn open<P>(path: P) -> Result<Repo>
    where
        P: AsRef<Path>,
    {
        init_lib_once();

        let path = utils::path_to_cstring(path.as_ref())?;
        let mut repo = ptr::null_mut();
        unsafe {
            utils::check(
                "open repo",
                raw::git_repository_open(&mut repo, path.as_ptr()),
            )?;
        }

        Ok(Repo { raw_ptr: repo })
    }

    pub fn reference_name_to_id(&self, ref_name: &str) -> Result<Oid> {
        let name = CString::new(ref_name)?;
        unsafe {
            let oid = {
                let mut oid = mem::MaybeUninit::uninit();
                utils::check(
                    "checkout HEAD",
                    raw::git_reference_name_to_id(
                        oid.as_mut_ptr(),
                        self.raw_ptr,
                        name.as_ptr() as *const c_char,
                    ),
                )?;
                oid.assume_init()
            };
            Ok(Oid { raw: oid })
        }
    }

    pub fn fetch_commit<'r>(&'r self, oid: &Oid) -> Result<Commit<'r>> {
        let mut commit = ptr::null_mut();
        unsafe {
            utils::check(
                "checkout commit",
                raw::git_commit_lookup(&mut commit, self.raw_ptr, &oid.raw),
            )?;
        }
        Ok(Commit {
            raw_ptr: commit,
            _lifetime_holder: PhantomData,
        })
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        unsafe { raw::git_repository_free(self.raw_ptr) }
    }
}

fn init_lib_once() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        unsafe {
            utils::check("init libgit", raw::git_libgit2_init()).expect("Error initing libgit2");
            // register a call back func, to be invoked before process exist
            assert_eq!(libc::atexit(shutdown_lib), 0);
        }
    });
}

// extern for using C calling conventions
extern "C" fn shutdown_lib() {
    unsafe {
        if let Err(e) = utils::check("fin libgit", raw::git_libgit2_shutdown()) {
            eprintln!("{}", e);
            process::abort();
        }
    }
}
