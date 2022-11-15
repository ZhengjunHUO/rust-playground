use std::fs::{File,OpenOptions};
use std::io::{ErrorKind,Write,Read,Seek};

fn main() {
    // file name typed string slice
    let filename = "foo.txt";

    // try to open a file (read-only mode)
    let rslt = File::open(filename);
    let mut f = match rslt {
        // file exist
        Ok(file) => file,
        // error while opening
        Err(error) => match error.kind() {
            // if file not exist, create it (File::create: write-only mode, need to use OpenOptions here)
            ErrorKind::NotFound => match OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(filename) {
                     Ok(mut created_file) => {
                         // Write to the created file
                         match created_file.write_all(b"Hello Rust!") {
                             // Rewind cursor to the begining of the file
                             Ok(_) => match created_file.rewind() {
                                 Ok(_) => {
                                     println!("{} not exist, create it", filename);
                                     created_file
                                 }
                                 Err(err) => panic!("Failed to rewind: {:?}", err),
                             }
                             Err(err) => panic!("Failed to write to created file: {:?}", err),
                         }
                     },
                     Err(err) => panic!("Failed to create: {:?}", err),
            },
            // all other unknown errors
            _ => panic!("Failed to open: {:?}", error),
        },
    };

    println!("file's info: {:?}", f);
    let mut buf = String::new();
    // read the file's content
    match f.read_to_string(&mut buf) {
        Ok(_) => println!("file's content: {:?}", buf),
        Err(err) => panic!("Failed to dump file: {:?}", err),
    }
}
