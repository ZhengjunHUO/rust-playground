use std::iter::successors;
use std::sync::mpsc;
use std::thread;

pub trait OffThread: Iterator {
    // turn self iterator to a iterator in a thread
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}

impl<T> OffThread for T
where
    T: Iterator + Send + 'static,
    T::Item: Send + 'static,
{
    fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
        let (tx, rx) = mpsc::sync_channel(4096);

        thread::spawn(move || {
            for i in self {
                if tx.send(i).is_err() {
                    break;
                }
            }
        });

        // Receiver implements IntoIterator trait
        rx.into_iter()
    }
}

fn main() {
    let l = successors(Some(0), |n| Some(n + 1))
        .take(20)
        .map(|n| n * n)
        .off_thread()
        .map(|n| n * 100)
        .off_thread()
        .collect::<Vec<u32>>();
    println!("{:?}", l);
}
