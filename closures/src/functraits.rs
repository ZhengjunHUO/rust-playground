// take a "Fn" parameter
pub fn give_five_to<F>(func: F) -> usize
    where F: Fn(usize) -> usize
{
    func(5)
}

// take a "FnMut" parameter
pub fn repeat<F>(mut func: F)
    where F: FnMut()
{
    func();
    func();
}

// take a "FnOnce" parameter
pub fn huo_say<F>(func: F)
    where F: FnOnce() -> String
{
    println!("HUO: {}", func());
}
