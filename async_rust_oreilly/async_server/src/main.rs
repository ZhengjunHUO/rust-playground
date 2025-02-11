use std::{
    io::{self, Cursor, ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::channel,
    },
    time::Duration,
};

use async_runtime::sleep::Sleep;
use data_layer::data::Payload;

static THREAD_IS_PARKED: [AtomicBool; 3] = [
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false),
];

macro_rules! spawn_worker {
    ($worker_name:expr, $rx:expr, $flag:expr) => {
        std::thread::spawn(move || {
            let mut executor = async_runtime::executor::Executor::new();
            loop {
                if let Ok(stream) = $rx.try_recv() {
                    println!(
                        "[{}] Recv conn from {}",
                        $worker_name,
                        stream.peer_addr().unwrap()
                    );
                    executor.spawn(handle(stream));
                } else {
                    if executor.polling.is_empty() {
                        println!("[{}] Become idle.", $worker_name);
                        $flag.store(true, std::sync::atomic::Ordering::SeqCst);
                        std::thread::park();
                    }
                }
                executor.poll();
            }
        })
    };
}

async fn handle(mut stream: TcpStream) -> io::Result<()> {
    stream.set_nonblocking(true)?;
    let mut buffer = Vec::new();
    let mut temp_buffer = [0; 1024];

    loop {
        match stream.read(&mut temp_buffer) {
            Ok(0) => {
                break;
            }
            Ok(size) => {
                buffer.extend_from_slice(&temp_buffer[..size]);
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                if !buffer.is_empty() {
                    break;
                }
                Sleep::new(Duration::from_millis(200)).await;
                continue;
            }
            Err(e) => {
                println!("Error occurred reading from conn: {}", e);
            }
        }
    }

    match Payload::deserialize(&mut Cursor::new(buffer.as_slice())) {
        Ok(payload) => println!("Recv payload: {:?}", payload),
        Err(e) => println!("Error occured during deserializing: {}", e),
    }
    Sleep::new(Duration::from_secs(1)).await;
    stream.write_all(b"Pong")?;
    Ok(())
}

fn main() -> io::Result<()> {
    let (tx0, rx0) = channel::<TcpStream>();
    let (tx1, rx1) = channel::<TcpStream>();
    let (tx2, rx2) = channel::<TcpStream>();

    let worker0 = spawn_worker!("Worker0", rx0, &THREAD_IS_PARKED[0]);
    let worker1 = spawn_worker!("Worker1", rx1, &THREAD_IS_PARKED[1]);
    let worker2 = spawn_worker!("Worker2", rx2, &THREAD_IS_PARKED[2]);

    let senders = [tx0, tx1, tx2];
    let workers = [worker0, worker1, worker2];
    let mut idx = 0;

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server up on localhost:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let _ = senders[idx].send(s);
                if THREAD_IS_PARKED[idx].load(Ordering::SeqCst) {
                    THREAD_IS_PARKED[idx].store(false, Ordering::SeqCst);
                    workers[idx].thread().unpark();
                }
                idx = (idx + 1) % 3;
            }
            Err(e) => println!("Error occurred on receiving conn: {}", e),
        }
    }
    Ok(())
}
