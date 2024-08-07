mod raw;
mod utils;

use chrono::{DateTime, Local};
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
    time::{Duration, UNIX_EPOCH},
};

#[allow(dead_code)]
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

// wrap the raw git_repository, providing a safe interface
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

    //pub fn fetch_commit<'r>(&'r self, oid: &Oid) -> Result<Commit<'r>> {
    pub fn fetch_commit(&self, oid: &Oid) -> Result<Commit> {
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

// wrapper for raw git_oid
pub struct Oid {
    pub raw: raw::git_oid,
}

impl Oid {
    pub fn sha(&self) -> Option<String> {
        unsafe {
            let mut buf: [mem::MaybeUninit<u8>; 40] = mem::MaybeUninit::uninit_array();
            if let Err(e) = utils::check(
                "get commit's sha",
                raw::git_oid_fmt(buf.as_mut_ptr() as *mut c_char, &self.raw),
            ) {
                eprintln!("{}", e);
                return None;
            }
            let commit_sha = mem::MaybeUninit::slice_assume_init_ref(&buf[..40]);
            Some(std::string::String::from_utf8_lossy(commit_sha).to_string())
        }
    }
}

// wrapper for raw git_commit, should not outlive the repo it comes from
pub struct Commit<'r> {
    raw_ptr: *mut raw::git_commit,
    _lifetime_holder: PhantomData<&'r Repo>,
}

impl<'r> Commit<'r> {
    pub fn author(&self) -> Signature {
        unsafe {
            Signature {
                raw: raw::git_commit_author(self.raw_ptr),
                _lifetime_holder: PhantomData,
            }
        }
    }

    pub fn message(&self) -> Option<&str> {
        unsafe {
            let msg = raw::git_commit_message(self.raw_ptr);
            utils::ptr_char_to_str(self, msg)
        }
    }
}

impl<'r> Drop for Commit<'r> {
    fn drop(&mut self) {
        unsafe {
            raw::git_commit_free(self.raw_ptr);
        }
    }
}

// wrapper for raw git_signature, should not outlive the commit it comes from
pub struct Signature<'c> {
    raw: *const raw::git_signature,
    _lifetime_holder: PhantomData<&'c str>,
}

impl<'c> Signature<'c> {
    pub fn name(&self) -> Option<&str> {
        unsafe { utils::ptr_char_to_str(self, (*self.raw).name) }
    }

    pub fn email(&self) -> Option<&str> {
        unsafe { utils::ptr_char_to_str(self, (*self.raw).email) }
    }

    pub fn datetime(&self) -> Option<String> {
        unsafe {
            let secs = (*self.raw).when.time as u64;
            if secs == 0 {
                return None;
            }

            let datetime = DateTime::<Local>::from(UNIX_EPOCH + Duration::from_secs(secs));
            Some(datetime.format("%a %b  %d %H:%M:%S %Y %z").to_string())
        }
    }
}
