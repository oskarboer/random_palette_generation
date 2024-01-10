use oklab::*;
use std::{env, f32::consts::PI};
use rand::prelude::*;
use std::fs::write;



fn oklch_to_oklab(l: f32, c: f32, h: f32) -> Oklab {
    Oklab {l: l, a: c * h.cos(), b: c * h.sin()}
}



fn generate_random(n: u8) -> Vec<RGB<u8>> {
    // let (a_min, a_max) = (-0.233, 0.275);
    // let (b_min, b_max) = (-0.311, 0.198);

    let mut ret = Vec::new();

    let mut rng = rand::thread_rng();

    let chroma_base = rng.gen::<f32>() * 0.1 + 0.01;
    let chroma_contrast = rng.gen::<f32>() * (0.125 - chroma_base - 0.075) + 0.075;

    let lightness_base = rng.gen::<f32>() * 0.3 + 0.3;
    let lightness_contrast = rng.gen::<f32>() * (1.0 - lightness_base - 0.3) + 0.3;

    let mut delta;
    let mut hue_offset;
    let hue_contrast = rng.gen::<f32>() * 0.7 + 0.3;
    let hue_base = rng.gen::<f32>() * 2.0 * PI;

    for i in 0..n {
        delta = i as f32 / (n - 1) as f32 ;
        hue_offset = delta * hue_contrast * 2.0 * PI + (PI / 4.0);
        hue_offset *= 0.33;

        let chroma = chroma_base + delta * chroma_contrast;
        let lightness = lightness_base + delta * lightness_contrast;
        
        
        ret.push(oklab_to_srgb(oklch_to_oklab(lightness, chroma, hue_base + hue_offset)));

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
