use std::{
    io,
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Instant,
};

use async_runtime::{executor::Executor, receiver::CustomTcpReceiver, sender::CustomTcpSender};
use data_layer::data::Payload;

async fn send_payload(slot1: u32, slot2: u16, slot3: String) -> io::Result<String> {
    let stream = Arc::new(Mutex::new(TcpStream::connect("127.0.0.1:8080")?));
    let payload = Payload {
        slot1,
        slot2,
        slot3,
    };
    CustomTcpSender {
        stream: stream.clone(),
        buf: payload.serialize()?,
    }
    .await?;
    let recv = CustomTcpReceiver {
        stream: stream.clone(),
        buf: Vec::new(),
    };
    String::from_utf8(recv.await?)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid utf8"))
}

fn main() -> io::Result<()> {
    let mut executor = Executor::new();
    let mut handles = Vec::new();
    let begin = Instant::now();

    println!("Sending requests ...");
    for i in 0..200 {
        let h = executor.spawn(send_payload(i, i as u16, format!("[Req {}] Ping", i)));
        handles.push(h);
    }
    std::thread::spawn(move || {
        loop {
            executor.poll();
        }
    });

    println!("Waiting responses ...");
    for h in handles {
        match h.recv().unwrap() {
            Ok(resp) => println!("Recv resp: {}", resp),
            Err(e) => println!("Error occurred: {}", e),
        }
    }
    println!("Done ! Time elapsed: {:?}", begin.elapsed());
    Ok(())
}
