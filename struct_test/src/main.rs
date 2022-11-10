#[derive(Debug)]
struct Rectangle {
    name: String,
    width: u32,
    height: u32,
}

fn main() {
    let unit = 2;
    let rc1 = Rectangle {
        name: String::from("foo"),
        width: dbg!(30 * unit),
        height: 20,
    };

    let rc2 = Rectangle {
        name: String::from("bar"),	// remove this line compilation will fail
        width: 40,
        ..rc1
    };

    // use reference here
    dbg!(&rc1);
    println!("rectangle {}'s size is {}.", rc1.name, size(&rc1));
    // use pretty "Debug" output format
    println!("{:#?}.", rc2);
    println!("rectangle {}'s size is {}.", rc2.name, size(&rc2));
}

fn size(rct: &Rectangle) -> u32 {
    rct.width * rct.height
}
