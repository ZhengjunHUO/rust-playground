fn main() {
    // (1) About "if let"
    //
    let game_preferred: Option<&str> = None;
    let has_ps5 = false;
    let budget: Result<u32, _> = "60".parse();

    if let Some(game) = game_preferred {
        println!("You can have the {game} as you wish");
    } else if has_ps5 {
        println!("Try Uncharted then.");
    } else if let Ok(budget) = budget {
        if budget > 50 {
            println!("Try a new 3A game then.");
        } else {
            println!("Got some old games in stock");
        }
    } else {
        println!("There are always some free-to-game who suit you.");
    }

    // (2) About "while let" conditional loops & for loop
    //
    let mut stack = Vec::new();
    for i in 0..10 {
        stack.push(i);
    }

    for (i, elem) in stack.iter().enumerate() {
        println!("Index [{i}]: {elem}");
    }

    while let Some(elem) = stack.pop() {
        println!("Count down: {}", elem);
    }

    // (3) About "named variable"
    //
    let x = Some(25);
    // named value y, will be shadowed in the match {}
    let y = 75;

    match x {
        Some(100) => println!("x == 100"),
        Some(n) if n*3 == y => println!("x == 3*y "),
        // new y in a new scope, match any value in Some, y get the same value as x
        Some(y) => println!("x: {:?}, inner y: {}", x, y),
        _ => println!("Catchall case"),
    }

    println!("x: {:?}, outer y: {}", x, y);

    // (4) Multi Patterns
    //
    let num = 18;
    match num {
        1 | 2 | 3 => println!("num is 1/2/3"),
        4..=20 => println!("num is between 4-20"),
        _ => println!("something else"),
    }

    let c = 'q';
    match c {
        'a'..='m' => println!("belong to the first half of lowercase alphabet"),
        'n'..='z' => println!("belong to the second half of lowercase alphabet"),
        _ => println!("some non lowercase alphabet char"),
    }

    // (5) pattern used for destructuring
    //
    let c = Cordinate{ x: 9, y: 0 };
    destructuring_struct(&c);
    destructuring_struct(&Cordinate{ x: 12, y: 15 });

    let s = Sketchbook::SetColor(ColorMode::Rgb(125, 100, 250));
    destructuring_enum(s);

    destructuring_enum(Sketchbook::SetColor(ColorMode::Hsv(25, 10, 200)));
    destructuring_enum(Sketchbook::CursorTo{ x: 20, y: 30 });
    destructuring_enum(Sketchbook::Draw(String::from("Hello Rust!")));
    destructuring_enum(Sketchbook::Close);

    // (6) ignoring
    //
    let dst = Some(26);
    let src = Some(12);
    copy_check_exist(src, dst);
    copy_check_exist(src, None);

    ignore_multi_places();
    ignore_consec_places();
}

struct Cordinate {
    x: i32,
    y: i32,
}

fn destructuring_struct(c: &Cordinate) {
    match c {
        Cordinate { x: x @ 10..=20, y: y @ 10..=20 } => println!("c({}, {}) is in a specific range !", x, y),
        // match c.x == 0
        Cordinate { x: 0, y } => println!("c is on the y axis, offset {}", y),
        // match c.y == 0
        Cordinate { x, y: 0 } => println!("c is on the x axis, offset {}", x),
        Cordinate { x, y } => println!("c({}, {}) is not on the axis", x, y),
    }

    // break apart c, store the value in x, y
    let Cordinate { x, y } = c;
    assert_eq!(*x, 9);
    assert_eq!(*y, 0);
}

enum ColorMode {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Sketchbook {
    CursorTo { x: i32, y: i32}, // struct-like enum variant
    SetColor(ColorMode), // nested enum
    Draw(String), // tuple-like enum variant (single element)
    Close, // enum variant without any data
}

fn destructuring_enum(s: Sketchbook) {
    match s {
        Sketchbook::CursorTo { x, y } => println!("Cursor moved to ({}, {})", x, y),
        Sketchbook::SetColor(ColorMode::Rgb(r, g, b)) => println!("Set brush color to RGB({}, {}, {})", r, g, b),
        Sketchbook::SetColor(ColorMode::Hsv(h, s, v)) => println!("Set brush color to HSV({}, {}, {})", h, s, v),
        Sketchbook::Draw(s) => println!("Drawing {s}"),
        Sketchbook::Close => println!("Quit."),
    }
}

fn copy_check_exist(src: Option<u32>, mut dst: Option<u32>) {
    match (src, dst) {
        (Some(_), Some(_)) => println!("dst already exist, skip copy"),
        _ => {
            dst = src;
        }
    }

    println!("dst is {:?}", dst);
}

fn ignore_multi_places () {
    let t = (1, 2, 3, 4, 5);

    match t {
        (first, _, third, _, fifth) => println!("partial tuple: ({first}, {third}, {fifth})"),
    }
}

fn ignore_consec_places () {
    let t = (1, 2, 3, 4, 5);

    match t {
        (first, .., last) => println!("partial tuple: ({first}, {last})"),
    }
}
