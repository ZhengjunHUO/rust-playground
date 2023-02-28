//use crossbeam;
use mandelbrot::{get_complex, get_pair, pixel2complex, render, save_to_img};
use rayon::prelude::*;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!(
            "Usage: {} FILENAME PIXEL_COORD UPPERLEFT LOWERRIGHT",
            args[0]
        );
        eprintln!(
            "   eg. {} result.png 5000x5000 -1.55#1.1 0.65#-1.1",
            args[0]
        );
        exit(1);
    }

    let canvas = get_pair(&args[2], 'x').expect("Failed parsing image's dimensions !");
    let upper_left = get_complex(&args[3]).expect("Failed parsing upper left corner point !");
    let lower_right = get_complex(&args[4]).expect("Failed parsing lower right corner point !");

    let mut pixels = vec![0; canvas.0 * canvas.1];

    // #1 Single thread rendering
    // render(canvas, (upper_left, lower_right), &mut pixels);

    // #2 Multi thread rendering using crossbeam (manually divided)
    /*
    let workers = 10;
    let rows_per_worker = canvas.1 / workers + 1;

    let sections: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_worker * canvas.0).collect();
    crossbeam::scope(|spawner| {
        for (i, sec) in sections.into_iter().enumerate() {
            let cpx_upper_left =
                pixel2complex(canvas, (upper_left, lower_right), (0, rows_per_worker * i));
            let cpx_lower_right = pixel2complex(
                canvas,
                (upper_left, lower_right),
                (canvas.0, rows_per_worker * i + sec.len() / canvas.0),
            );
            spawner.spawn(move |_| {
                render(
                    (canvas.0, sec.len() / canvas.0),
                    (cpx_upper_left, cpx_lower_right),
                    sec,
                );
            });
        }
    })
    .unwrap();
    */

    // #3 Multi thread rendering using rayon
    let sections: Vec<(usize, &mut [u8])> = pixels.chunks_mut(canvas.0).enumerate().collect();
    sections.into_par_iter().for_each(|(i, sec)| {
        let top = i;
        let canvas_range = (canvas.0, 1);

        let cpx_upper_left = pixel2complex(canvas, (upper_left, lower_right), (0, top));
        let cpx_lower_right = pixel2complex(canvas, (upper_left, lower_right), (canvas.0, top + 1));
        render(canvas_range, (cpx_upper_left, cpx_lower_right), sec);
    });

    save_to_img(&args[1], canvas, &pixels).expect("Failed writing to PNG !");
}
