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

fn find_boundries() {
    let (mut a_min, mut a_max) = (f32::MAX, f32::MIN);
    let (mut b_min, mut b_max) = (f32::MAX, f32::MIN);

    for r in 0..255 {
        for g in 0..255 {
            for b in 0..255 {
                let color = srgb_to_oklab(RGB {r, g, b});

                a_min = a_min.min(color.a);
                a_max = a_max.max(color.a);
                b_min = b_min.min(color.b);
                b_max = b_max.max(color.b);

            }
        }
    }
    println!("a range:{a_min:.3} - {a_max:.3}, b range: {b_min:.3} - {b_max:.3}");
}

fn main() {

    find_boundries();

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
