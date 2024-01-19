use oklab::*;
use rand::prelude::*;
use std::fs::write;
use std::{env, f32::consts::PI};

fn oklch_to_oklab(l: f32, c: f32, h: f32) -> Oklab {
    Oklab {
        l: l,
        a: c * h.cos(),
        b: c * h.sin(),
    }
}

fn put_in_range(num: f32, min: f32, max: f32) -> f32 {
    num * (max - min) + min
}

struct Settings {
    chroma_base: f32,
    chroma_contrast: f32,

    lightness_base: f32,
    lightness_contrast: f32,

    hue_contrast: f32,
    hue_base: f32,

    color_count: u8,
}

fn generate_random(settings: Settings) -> Vec<RGB<u8>> {
    let mut ret = Vec::new();

    let mut delta;
    let mut hue_offset;

    for i in 0..settings.color_count {
        delta = i as f32 / (settings.color_count - 1) as f32;
        hue_offset = delta * settings.hue_contrast * 2.0 * PI + (PI / 4.0);
        hue_offset *= 0.33;

        let chroma = settings.chroma_base + delta * settings.chroma_contrast;
        let lightness = settings.lightness_base + delta * settings.lightness_contrast;

        ret.push(oklab_to_srgb(oklch_to_oklab(
            lightness,
            chroma,
            settings.hue_base + hue_offset,
        )));
    }

    ret
}

fn find_boundries() {
    let (mut a_min, mut a_max) = (f32::MAX, f32::MIN);
    let (mut b_min, mut b_max) = (f32::MAX, f32::MIN);

    for r in 0..255 {
        for g in 0..255 {
            for b in 0..255 {
                let color = srgb_to_oklab(RGB { r, g, b });

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
    let head = String::from(
        "<svg version=\"1.1\"\nwidth=\"300\" height=\"200\"\nxmlns=\"http://www.w3.org/2000/svg\">",
    );
    // let rect_template = String::from("<rect width=\"{:.2}%\" height=\"100%\" fill=\"#{}\" />");
    let end = String::from("</svg>");

    let fraction = 100.0 / colors.len() as f32;
    let mut content = vec![];
    content.push(head);

    for (i, color) in colors.iter().enumerate() {
        content.push(format!(
            "<rect x=\"{}%\" width=\"{:.2}%\" height=\"100%\" fill=\"#{}\" />",
            fraction * i as f32,
            fraction,
            format!("{:0>2X}{:0>2X}{:0>2X}", color.r, color.g, color.b)
        ));
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

    let mut rng = rand::thread_rng();

    let chroma_base = put_in_range(rng.gen::<f32>(), 0.01, 0.1);
    let chroma_contrast = put_in_range(rng.gen::<f32>(), 0.075, 0.125 - chroma_base);

    let lightness_base = put_in_range(rng.gen::<f32>(), 0.3, 0.6);
    let lightness_contrast = put_in_range(rng.gen::<f32>(), 0.3, 1.0 - lightness_base);

    let hue_contrast = put_in_range(rng.gen::<f32>(), 0.3, 1.0);
    let hue_base = rng.gen::<f32>() * 2.0 * PI;

    let settings = Settings {
        chroma_base: chroma_base,
        chroma_contrast: chroma_contrast,
        lightness_base: lightness_base,
        lightness_contrast: lightness_contrast,
        hue_contrast: hue_contrast,
        hue_base: hue_base,
        color_count: num_of_colors,
    };

    let colors = generate_random(settings);

    for color in colors.iter() {
        print!("#{:0>2X}{:0>2X}{:0>2X} ", color.r, color.g, color.b);
    }
    println!("");

    generate_palette_svg("palette.svg".to_string(), colors);
}
