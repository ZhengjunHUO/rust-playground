use crate::worker::WorkerPool;
use anyhow::Result;
use crossbeam_channel::{bounded, select, unbounded, Receiver};
use ctrlc;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

const RESP_OK_STATUS: &str = "HTTP/1.1 200 OK";
const RESP_NOT_FOUND_STATUS: &str = "HTTP/1.1 404 NOT FOUND";
const REQ_ROOT_FORMAT: &str = "GET / HTTP/1.1";
const REQ_SLEEP_FORMAT: &str = "GET /expensive HTTP/1.1";

pub struct Server {
    l: Option<TcpListener>,
    wp: Option<WorkerPool>,
}

impl Server {
    pub fn new(socket: &str, num_thread: usize) -> Server {
        let l = TcpListener::bind(socket).unwrap();
        let wp = WorkerPool::new(num_thread);

        Server {
            l: Some(l),
            wp: Some(wp),
        }
    }

    pub fn start(&self) {
        // handler for Ctrl-C
        let cancel = ctrlc_chan().unwrap();

        let (tx, rx) = unbounded();
        let listener_clone = self.l.as_ref().unwrap().try_clone().unwrap();
        // thread accepting incoming connection, sent to rx
        thread::spawn(move || {
            // retrieve incoming TcpStream
            // take only 5 req: l.as_ref().unwrap().incoming().take(5)
            for conn in listener_clone.incoming() {
                let conn = conn.unwrap();
                tx.send(conn).unwrap();
            }
        });

        loop {
            select! {
                recv(cancel) -> _ => {
                    println!("Catch Ctrl-C signal, quitting ...");
                    break;
                }

                recv(rx) -> rslt => {
                    // rslt is Result<TcpStream, crossbeam_channel::RecvError>
                    match rslt {
                        Ok(conn) => {
                            println!("[DEBUG] Recv conn attempt!");
                            // send handler to workerpool
                            self.wp.as_ref().unwrap().schedule(|| {
                                handle_conn(conn);
                            });
                            println!("[DEBUG] Conn dispatched!");
                        }
                        Err(e) => {
                            println!("[DEBUG] Got an error: {e}");
                            break;
                        }
                    }
                }
            }
        }
    }

    //pub fn stop(&mut self) {
    //    drop(self.l.take());
    //    drop(self.wp.take());
    //}
}

fn handle_conn(mut conn: TcpStream) {
    let rdr = BufReader::new(&mut conn);

    // read the http request until a new empty line ("\n\n")
    //let req: Vec<_> = rdr.lines().map(|rslt| rslt.unwrap()).take_while(|line| !line.is_empty()).collect();

    let req_format = rdr.lines().next().unwrap().unwrap();
    println!("[DEBUG] Recv conn req: {:#?}", req_format);

    // under the hood: let (filename, resp_status) = if &req_format[..] == REQ_ROOT_FORMAT {
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

fn ctrlc_chan() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}
