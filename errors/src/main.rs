use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Write;
use std::io::Read;

fn main() {
    // name typed string slice
    let filename = "foo.txt";

    // try to open a file (read-only mode)
    let rslt = File::open(filename);
    let mut f = match rslt {
        // file exist
        Ok(file) => file,
        // other open error
        Err(error) => match error.kind() {
            // if file not exist, create it (write-only mode)
            ErrorKind::NotFound => match OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(filename) {
                     Ok(mut created_file) => {
                         match created_file.write_all(b"Hello Rust!") {
                             Ok(_) => created_file,
                             Err(err) => panic!("Failed to write to created file: {:?}", err),
                         }
                     },
                     Err(err) => panic!("Failed to create: {:?}", err),
            },
            // other unknown errors
            _ => panic!("Failed to open: {:?}", error),
        },
    };

    println!("file's info: {:?}", f);
    let mut buf = String::new();
    match f.read_to_string(&mut buf) {
        Ok(_) => println!("file's content: {:?}", buf),
        Err(err) => panic!("Failed to dump file: {:?}", err),
    }
}
