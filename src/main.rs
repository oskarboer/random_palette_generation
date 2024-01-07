use oklab::*;
use std::env;
use rand::prelude::*;

fn generate_random(n: u8) -> Vec<RGB<u8>> {
    let mut ret = Vec::new();

    let mut rng = rand::thread_rng();
    let l = rng.gen::<f32>();
    let mut a = rng.gen::<f32>() * 3.0 - 1.5;
    let mut b = rng.gen::<f32>() * 3.0 - 1.5;

    let da = rng.gen::<f32>() / n as f32;
    let db = rng.gen::<f32>() / n as f32;

    for _ in 0..n {
        ret.push(oklab_to_srgb(Oklab {l, a, b}));
        a += da;
        b += db;
    }

    ret
}

fn main() {

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let mut num_of_colors = 4; // default number;


    if let Some(number_str) = args.get(1) {
        if let Ok(num) = number_str.parse::<u8>() {
            num_of_colors = num;
        } else {
            eprintln!("parsing error, dropping to default")
        }
    }

    let colors = generate_random(num_of_colors);

    // println!("{:?}", colors);

    for color in colors {
        print!("#{:0>2X}{:0>2X}{:0>2X} ", color.r, color.g, color.b);
    }
    println!("");
}
