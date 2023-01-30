use error_chain::error_chain;
use reqwest;
use scraper::{node::Node, Html};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};

const HTTPS_SCHEME: &str = "https://";
const RESULT_FOLDER: &str = "result/";
static TARGET_URL: &str = "httpbin.org/";
static EXCLUDE_PATTERN: &str = "Unknown";
static INCLUDE_PATTERN: &str = "<span";

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let f = File::open("urls")?;
    let buf = BufReader::new(f);
    for line in buf.lines() {
        if let Ok(path) = line {
            match probe(&path) {
                Err(e) => {
                    println!("Failed to retrieve {}: {}", path, e)
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn probe(path: &str) -> Result<()> {
    println!("[DEBUG] URL: {}{}", TARGET_URL, path);
    let mut rslt = reqwest::blocking::get(HTTPS_SCHEME.to_owned() + TARGET_URL + path)?;
    println!("[DEBUG] HTTP/2 {}", rslt.status());
    let mut body = String::new();
    rslt.read_to_string(&mut body)?;
    println!("[DEBUG] Headers:\n{:#?}", rslt.headers());

    if !body.contains(EXCLUDE_PATTERN) {
        let mut filtered = String::new();
        for line in body.lines() {
            if line.starts_with(INCLUDE_PATTERN) {
                filtered.push_str(line);
                filtered.push_str("\n");
            }
        }

        let mut result = String::new();
        let fragment = Html::parse_fragment(&filtered);
        for node in fragment.tree {
            if let Node::Text(text) = node {
                result.push_str(&text.text);
            }
        }

        let result_filename = RESULT_FOLDER.to_owned() + path;
        let mut f = File::create(&result_filename)?;
        f.write_all(result.as_bytes())?;
        println!("[DEBUG] Write {} to disk.", result_filename);
    }
    //println!("Body:\n{}", body);

    Ok(())
}
