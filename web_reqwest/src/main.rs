use error_chain::error_chain;
use reqwest;
use std::fs::File;
use std::io::{Read, Write};

const HTTPS_SCHEME: &str = "https://";
const RESULT_FOLDER: &str = "result/";
static TARGET_URL: &str = "httpbin.org";

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    probe("/get")
}

fn probe(path: &str) -> Result<()> {
    let result_filename = RESULT_FOLDER.to_owned() + path;
    let mut f = File::create(result_filename)?;

    let mut rslt = reqwest::blocking::get(HTTPS_SCHEME.to_owned() + TARGET_URL + path)?;
    let mut body = String::new();
    rslt.read_to_string(&mut body)?;

    println!("Host: {}", TARGET_URL);
    println!("HTTP/2 {}", rslt.status());
    println!("Headers:\n{:#?}", rslt.headers());
    println!("Body:\n{}", body);

    f.write_all(body.as_bytes())?;

    Ok(())
}
