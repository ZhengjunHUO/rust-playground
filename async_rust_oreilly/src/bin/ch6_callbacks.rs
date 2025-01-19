fn func_take_callback<F>(callback: F)
where
    F: Fn(u32),
{
    println!("Inside wrapper func, will execute callback func.");
    callback(25);
}

fn main() {
    let cb = |num: u32| {
        println!("Inside callback func, the num is {num}");
    };

    func_take_callback(cb);
}
