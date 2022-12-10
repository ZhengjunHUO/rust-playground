use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::fs;

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
    let req: Vec<_> = rdr.lines().map(|rslt| rslt.unwrap()).take_while(|line| !line.is_empty()).collect();

    println!("[DEBUG] Recv conn req: {:#?}", req);

    let status = "HTTP/1.1 200 OK";
    let payload = fs::read_to_string("index.html").unwrap();
    let len = payload.len();

    let resp = format!("{status}\r\nContent-Length: {len}\r\n\r\n{payload}");
    conn.write_all(resp.as_bytes()).unwrap();
}
