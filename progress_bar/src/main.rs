use indicatif;
use std::{thread, time::Duration};

const NUM_JOB: u64 = 10;

fn work_on_sth() {
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    let ind = indicatif::ProgressBar::new(NUM_JOB);
    for i in 0..NUM_JOB {
        work_on_sth();
        ind.println(format!("[+] Task #{} finished !", i));
        ind.inc(1);
    }
    ind.finish_with_message("All done!");
}
