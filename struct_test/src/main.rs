#[derive(Debug)]
struct Rectangle {
    name: String,
    width: u32,
    height: u32,
}

impl Rectangle {
    fn size(&self) -> u32 {
        self.width * self.height
    }

    // &self is an alias
    fn contains(self: &Self, rect: &Rectangle) -> bool {
	self.width >= rect.width && self.height >= rect.height
    }

    fn new(name: String, width: u32, height: u32) -> Self {
        Self {
                name,
                width,
                height,
        }
    }
}

fn main() {
    let unit = 2;
    let rc1 = Rectangle {
        name: String::from("foo"),
        width: dbg!(30 * unit),
        height: 20,
    };

    let rc2 = Rectangle {
        name: String::from("bar"),	// remove this line compilation will fail(String is "moved" from rc1 to rc2)
        width: 40,
        ..rc1
    };

    let rc3 = Rectangle::new(String::from("huo"), 30, 100);

    // use reference here
    dbg!(&rc1);
    println!("rectangle {}'s size is {}.", rc1.name, size(&rc1));
    // use pretty "Debug" output format
    println!("{:#?}", rc2);
    println!("rectangle {}'s size is {}.", rc2.name, rc2.size());

    println!("rectangle {} contains {} ? {}", rc1.name, rc2.name, rc1.contains(&rc2));
    println!("rectangle {} contains {} ? {}", rc1.name, rc3.name, rc1.contains(&rc3));
}

fn size(rct: &Rectangle) -> u32 {
    rct.width * rct.height
}
