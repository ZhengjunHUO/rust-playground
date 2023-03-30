use crate::raw::*;
use crate::utils::check;

pub mod raw;
mod utils;

fn main() {
    unsafe {
        check("init libgit", git_libgit2_init());
        git_libgit2_shutdown();
    }
}
