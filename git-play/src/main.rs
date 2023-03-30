pub mod raw;

use crate::raw::*;

fn main() {
    unsafe {
        git_libgit2_init();
        git_libgit2_shutdown();
    }
}
