use crossbeam_channel::{after, bounded, select, tick, Receiver};
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();

    // receive a signal every 0.5s
    let ticker = tick(Duration::from_millis(500));
    // receive a signal at the end of 5s
    let timeout = after(Duration::from_secs(5));
    // receive a signal when pressing ctrl + c
    let cancel = ctrlc_chan().unwrap();

    println!("Press Ctrl + c in 5 seconds !");
    loop {
        select! {
            recv(ticker) -> _ => println!("Elapsed: {:?}", start.elapsed()),
            recv(timeout) -> _ => {
                println!("Time out !");
                break;
            }
            recv(cancel) -> _ => {
                println!("Ctrl-C signal caught, quit !");
                break;
            }
        }
    }
}

fn ctrlc_chan() -> Result<Receiver<()>, ctrlc::Error> {
    // Creates a channel of bounded capacity
    let (sender, receiver) = bounded(10);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}
