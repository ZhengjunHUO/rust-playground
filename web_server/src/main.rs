use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
    fs,
    thread,
    time::Duration,
};
use web_server::WorkerPool;

const RESP_OK_STATUS: &str = "HTTP/1.1 200 OK";
const RESP_NOT_FOUND_STATUS: &str = "HTTP/1.1 404 NOT FOUND";
const REQ_ROOT_FORMAT: &str = "GET / HTTP/1.1";
const REQ_SLEEP_FORMAT: &str = "GET /expensive HTTP/1.1";

fn main() {
    let l = TcpListener::bind("127.0.0.1:8080").unwrap();
    let wp = WorkerPool::new(3);

    // retrieve incoming TcpStream
    for conn in l.incoming() {
        let conn = conn.unwrap();

        println!("[DEBUG] Recv conn attempt!");
        wp.schedule(|| {
	    handle_conn(conn);
        });
    }
}

fn handle_conn(mut conn: TcpStream) {
    let rdr = BufReader::new(&mut conn);

    // read the http request until a new empty line ("\n\n")
    //let req: Vec<_> = rdr.lines().map(|rslt| rslt.unwrap()).take_while(|line| !line.is_empty()).collect();

    let req_format = rdr.lines().next().unwrap().unwrap();
    println!("[DEBUG] Recv conn req: {:#?}", req_format);

    // under the hood: let (filename, resp_status) = if &req_format[..] == REQ_ROOT_FORMAT {
    //let (filename, resp_status) = if req_format == REQ_ROOT_FORMAT {
    //    ("index.html", RESP_OK_STATUS)
    //} else {
    //    ("404.html", RESP_NOT_FOUND_STATUS)
    //};

    let (filename, resp_status) = match &req_format[..] {
        REQ_ROOT_FORMAT => ("index.html", RESP_OK_STATUS),
        REQ_SLEEP_FORMAT => {
            thread::sleep(Duration::from_secs(10));
            ("index.html", RESP_OK_STATUS)
        }
        _ => ("404.html", RESP_NOT_FOUND_STATUS),
    };

    let payload = fs::read_to_string(filename).unwrap();
    let len = payload.len();
    let resp = format!("{resp_status}\r\nContent-Length: {len}\r\n\r\n{payload}");

    conn.write_all(resp.as_bytes()).unwrap();
}
