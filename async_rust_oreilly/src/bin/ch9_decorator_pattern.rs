trait Echo {
    fn echo(&self) -> String;
}

struct Hello;
impl Echo for Hello {
    fn echo(&self) -> String {
        String::from("Hello Rust !")
    }
}

struct EchoWrapper<T> {
    echoer: T,
}

impl<T> EchoWrapper<T>
where
    T: Echo,
{
    fn echo(&self) -> String {
        let mut rslt = String::from("Inside wrapper: ");
        let temp = self.echoer.echo();
        rslt.push_str(&temp);
        rslt
    }
}

fn main() {
    let e = Hello;
    let w = EchoWrapper { echoer: Hello };
    println!("{}", e.echo());
    println!("{}", w.echo());
}
