use error_chain::error_chain;
use reqwest;
use std::io::Read;

const HTTPS_SCHEME: &str = "https://";

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let target_url = "www.kernel.org";
    let mut rslt = reqwest::blocking::get(HTTPS_SCHEME.to_owned() + target_url)?;
    let mut body = String::new();
    rslt.read_to_string(&mut body)?;

    println!("Host: {}", target_url);
    println!("HTTP/2 {}", rslt.status());
    println!("Headers:\n{:#?}", rslt.headers());
    println!("Body:\n{}", body);

    Ok(())
}
