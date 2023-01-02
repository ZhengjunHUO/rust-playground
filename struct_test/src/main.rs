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

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn up(&mut self) {
        self.y += 1;
    }

    fn down(&mut self) {
        self.y -= 1;
    }

    fn left(&mut self) {
        self.x -= 1;
    }

    fn right(&mut self) {
        self.x += 1;
    }
}

struct Cluster {
    name: Option<String>,
    nodes: u32,
}

fn main() {
    let unit = 2;
    let rc1 = Rectangle {
        name: String::from("foo"),
        width: dbg!(30 * unit),
        height: 20,
    };

    let rc2 = Rectangle {
        name: String::from("bar"), // remove this line compilation will fail(String is "moved" from rc1 to rc2)
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

    println!(
        "rectangle {} contains {} ? {}",
        rc1.name,
        rc2.name,
        rc1.contains(&rc2)
    );
    println!(
        "rectangle {} contains {} ? {}",
        rc1.name,
        rc3.name,
        rc1.contains(&rc3)
    );

    let mut c = Coordinate { x: 3, y: 5 };
    println!("Initial coordiante: {:?}", c);
    c.up();
    c.left();
    println!("Moved coordiante: {:?}", c);


    // 如果需要从结构中取出个别值，可以考虑在结构定义使用Option<T>
    // 调用take可以取出Some<T>并在原地留下None, 不会触发ownership的约束
    let mut clusters = Vec::new();
    clusters.push(Cluster {
        name: Some("internal".to_string()),
        nodes: 8,
    });

    let cname = clusters[0].name.take();
    // 等同于take
    //let cname = std::mem::replace(&mut clusters[0].name, None);
    assert_eq!(cname, Some("internal".to_string()));
    assert_eq!(clusters[0].name, None);
}

fn size(rct: &Rectangle) -> u32 {
    rct.width * rct.height
}
