use oklab::*;
use std::env;
use rand::prelude::*;
use std::fs::write;


fn generate_random(n: u8) -> Vec<RGB<u8>> {
    let (a_min, a_max) = (-0.233, 0.275);
    let (b_min, b_max) = (-0.311, 0.198);

    let mut ret = Vec::new();

    let mut rng = rand::thread_rng();
    let l = rng.gen::<f32>();
    let mut a = rng.gen::<f32>() * (a_max - a_min) - a_min;
    let mut b = rng.gen::<f32>() * (b_max - b_min) - b_min;

    let mut da = rng.gen::<f32>() / n as f32;
    let mut db = rng.gen::<f32>() / n as f32;

    for _ in 0..n {
        ret.push(oklab_to_srgb(Oklab {l, a, b}));
        a += da;
        b += db;

        if a > a_max || a < a_min {
            da *= -1.0;
        }
        if b > b_max || b < b_min{
            db *= -1.0;
        }
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
    println!("a range: {a_min:.3} - {a_max:.3}, b range: {b_min:.3} - {b_max:.3}");
}

fn generate_palette_svg(fname: String, colors: Vec<RGB<u8>>) {
    let head = String::from("<svg version=\"1.1\"\nwidth=\"300\" height=\"200\"\nxmlns=\"http://www.w3.org/2000/svg\">");
    // let rect_template = String::from("<rect width=\"{:.2}%\" height=\"100%\" fill=\"#{}\" />");
    let end = String::from("</svg>");

    let fraction = 100.0 / colors.len() as f32;
    let mut content = vec![];
    content.push(head);
    
    for (i, color) in colors.iter().enumerate() {
        content.push(format!("<rect x=\"{}%\" width=\"{:.2}%\" height=\"100%\" fill=\"#{}\" />", fraction * i as f32, fraction, format!("{:0>2X}{:0>2X}{:0>2X}", color.r, color.g, color.b)));
    }
    content.push(end);
    
    write(fname, content.join("\n")).unwrap();
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

    for color in colors.iter() {
        print!("#{:0>2X}{:0>2X}{:0>2X} ", color.r, color.g, color.b);
    }
    println!("");

    generate_palette_svg("palette.svg".to_string(), colors);
}
