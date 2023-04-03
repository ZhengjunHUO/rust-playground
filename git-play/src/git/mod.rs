pub mod raw;
pub mod utils;

use std::{
    error,
    fmt::{self, Display, Formatter},
    result,
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

impl error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;
