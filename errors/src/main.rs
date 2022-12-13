use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, ErrorKind, Read, Seek, Write};

fn main() -> Result<(), Box<dyn Error>> {
    // file name typed string slice
    let filename = "foo.txt";
    let content = read_file_with_creation(filename)?;
    println!("file's content: {:?}", content);

    let content2 = read_file(filename)?;
    println!("file's content: {:?}", content2);

    let content3 = read_file_2(filename)?;
    println!("file's content: {:?}", content3);

    Ok(())
}

fn read_file_with_creation(filename: &str) -> Result<String, io::Error> {
    // try to open a file (read-only mode)
    // (1) use closure and the unwrap_or_else method here
    let mut f = File::open(filename).unwrap_or_else(|error| {
        // error while opening
        if error.kind() == ErrorKind::NotFound {
            // if file not exist, create it (File::create: write-only mode, need to use OpenOptions here)
            // (2) use match
            match OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(filename)
            {
                Ok(mut created_file) => {
                    // Write to the created file
                    match created_file.write_all(b"Hello Rust!") {
                        // Rewind cursor to the begining of the file
                        Ok(_) => match created_file.rewind() {
                            Ok(_) => {
                                println!("[DEBUG] {} not exist, create it", filename);
                                created_file
                            }
                            Err(err) => panic!("Failed to rewind: {:?}", err),
                        },
                        Err(err) => panic!("Failed to write to created file: {:?}", err),
                    }
                }
                Err(err) => panic!("Failed to create: {:?}", err),
            }
        } else {
            // all other unknown errors
            panic!("Failed to open: {:?}", error);
        }
    });

    println!("[DEBUG] file's info: {:?}", f);
    let mut buf = String::new();

    // read the file's content
    match f.read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(err) => Err(err),
    }
    // (3) use expect
    //f.read_to_string(&mut buf).expect("Failed to dump file");
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    //let mut f = match File::open(filename) {
    //    Ok(file) => file,
    //    Err(e) => return Err(e),
    //};
    let mut f = File::open(filename)?;

    let mut buf = String::new();

    //match f.read_to_string(&mut buf) {
    //    Ok(_) => Ok(buf),
    //    Err(e) => Err(e),
    //}
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

fn read_file_2(filename: &str) -> Result<String, io::Error> {
    let mut buf = String::new();
    File::open(filename)?.read_to_string(&mut buf)?;
    Ok(buf)
}
