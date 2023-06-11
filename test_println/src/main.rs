//use std::fs::File;
use std::io::Write;
use std::mem::MaybeUninit;
use std::sync::Mutex;

//static mut F: MaybeUninit<Mutex<File>> = MaybeUninit::uninit();
static mut BUF: MaybeUninit<Mutex<Vec<u8>>> = MaybeUninit::uninit();

/// shadow the println macro
macro_rules! println {
    ($($tt:tt)*) => {{
        //unsafe { writeln!(&mut F.assume_init_mut().lock().unwrap(), $($tt)*).unwrap(); }
        unsafe { writeln!(&mut BUF.assume_init_mut().lock().unwrap(), $($tt)*).unwrap(); }
    }}
}

/// if use test_println::say_hi; here will not work as expected since macro is not shadowed
pub fn say_hi() {
    println!("Hello Huo !");
}

fn main() {
    use std::ops::Deref;

    unsafe {
        //F.write(Mutex::new(File::create("rslt").unwrap()));
        BUF.write(Mutex::new(Vec::new()));
    }
    say_hi();
    let buf = unsafe { BUF.assume_init_ref() };
    assert_eq!(buf.lock().unwrap().deref(), b"Hello Huo !\n");
}
