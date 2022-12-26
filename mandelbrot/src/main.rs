use mandelbrot::{get_complex, get_pair, render, save_to_img};
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!(
            "Usage: {} FILENAME PIXEL_COORD UPPERLEFT LOWERRIGHT",
            args[0]
        );
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20#0.35 -1#0.20",
            args[0]
        );
        exit(1);
    }

    let canvas = get_pair(&args[2], 'x').expect("Failed parsing image's dimensions !");
    let upper_left = get_complex(&args[3]).expect("Failed parsing upper left corner point !");
    let lower_right = get_complex(&args[4]).expect("Failed parsing lower right corner point !");

    let mut pixels = vec![0; canvas.0 * canvas.1];

    render(canvas, (upper_left, lower_right), &mut pixels);

    save_to_img(&args[1], canvas, &pixels).expect("Failed writing to PNG !");
}
