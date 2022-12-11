use std::{
    sync::{mpsc, Arc, Mutex},
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
    fs,
    thread,
    time::Duration,
};

const RESP_OK_STATUS: &str = "HTTP/1.1 200 OK";
const RESP_NOT_FOUND_STATUS: &str = "HTTP/1.1 404 NOT FOUND";
const REQ_ROOT_FORMAT: &str = "GET / HTTP/1.1";
const REQ_SLEEP_FORMAT: &str = "GET /expensive HTTP/1.1";

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct Server {
    l: TcpListener,
    wp: WorkerPool,
}

struct WorkerPool {
    workers: Vec<Worker>,
    tx: Option<mpsc::Sender<Task>>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Server {
    pub fn new(socket: &str, num_thread: usize) -> Server {
        let l = TcpListener::bind(socket).unwrap();
        let wp = WorkerPool::new(num_thread);

        Server { l , wp }
    }

    pub fn start(&self) {
        // retrieve incoming TcpStream
        // take only 5 req: l.incoming().take(5), will trigger the cleanup process
        for conn in self.l.incoming() {
            let conn = conn.unwrap();

            println!("[DEBUG] Recv conn attempt!");
            // send handler to workerpool
            self.wp.schedule(|| {
              handle_conn(conn);
            });
            println!("[DEBUG] Conn dispatched!");
        }
    }
}

impl WorkerPool {
    /// Create a new WorkerPool, whose size should be a positive number
    ///
    /// # Panics
    ///
    /// The `new` func will panic if s is zero
    fn new(s: usize) -> WorkerPool {
        assert!(s > 0);

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(s);

        for id in 0..s {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        WorkerPool { workers, tx: Some(tx) }
    }

    fn schedule<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(f);
        // send the handler(with conn) through the channel to the workerpool
        self.tx.as_ref().unwrap().send(task).unwrap();
    }
}

// hook called when program exit
impl Drop for WorkerPool {
    fn drop(&mut self) {
        // close the sender first to close the channel
        // if not the join() method below will not return
        // because of the workers loop forever
        drop(self.tx.take());

        for w in &mut self.workers {
            println!("[Worker {}] Stopped.", w.id);

            // move occurs
            // take() on "Option" to move value out of "Some" variant and leave a "None" variant
            // clean up done
            if let Some(thread) = w.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Task>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let msg = rx.lock().unwrap().recv();
            match msg {
                Ok(task) => {
                    println!("[Worker {}] Receive task.", id);
                    task();
                }
                Err(_) => {
                    println!("[Worker {}] Gracefully shutting down ...", id);
                    break;
                }
            }
        });

	Worker { id, thread: Some(thread) }
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
