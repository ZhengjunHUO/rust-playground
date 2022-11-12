mod frontend;
use crate::frontend::recv::Session;

fn decode() {}

pub fn call_service() {
    //let mut s = crate::frontend::recv::Session::authentication("fufu");
    let mut s = Session::authentication("fufu");
    println!("Login as {} ...", s.username);

    s.username = String::from("huo");
    println!("Not working, try login as {} ...", s.username);
}
