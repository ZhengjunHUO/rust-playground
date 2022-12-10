use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::fs;

const RESP_OK_STATUS: &str = "HTTP/1.1 200 OK";
const RESP_NOT_FOUND_STATUS: &str = "HTTP/1.1 404 NOT FOUND";
const REQ_FORMAT: &str = "GET / HTTP/1.1";

fn main() {
    let l = TcpListener::bind("127.0.0.1:8080").unwrap();

    // retrieve incoming TcpStream
    for conn in l.incoming() {
        let conn = conn.unwrap();

        println!("[DEBUG] Recv conn attempt!");
	handle_conn(conn);
    }
}

fn handle_conn(mut conn: TcpStream) {
    let rdr = BufReader::new(&mut conn);

    // read the http request until a new empty line ("\n\n")
    //let req: Vec<_> = rdr.lines().map(|rslt| rslt.unwrap()).take_while(|line| !line.is_empty()).collect();

    let req_format = rdr.lines().next().unwrap().unwrap();
    println!("[DEBUG] Recv conn req: {:#?}", req_format);

    let (filename, resp_status) = if req_format == REQ_FORMAT {
        ("index.html", RESP_OK_STATUS)
    } else {
        ("404.html", RESP_NOT_FOUND_STATUS)
    };

    let payload = fs::read_to_string(filename).unwrap();
    let len = payload.len();
    let resp = format!("{resp_status}\r\nContent-Length: {len}\r\n\r\n{payload}");

    conn.write_all(resp.as_bytes()).unwrap();
}
