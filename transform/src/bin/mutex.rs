use std::sync::Mutex;

fn main() {
    let lists = Mutex::new(Vec::new());

    let mut guard = lists.lock().unwrap();
    for i in 0..3 {
        guard.push(vec![i]);
    }

    for i in 0..guard.len() {
        println!("{:?}", guard[i]);
    }
}
