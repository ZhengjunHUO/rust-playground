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
        // new y in a new scope, match any value in Some, y get the same value as x
        Some(y) => println!("x: {:?}, inner y: {}", x, y),
        _ => println!("Catchall case"),
    }

    println!("x: {:?}, outer y: {}", x, y);
}
