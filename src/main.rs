use image::{io::Reader as ImageReader, GenericImageView, RgbImage};
use serde::Serialize;
use serde_json;
use std::fs;

#[derive(Debug, Clone, Serialize)]
struct Text {
    text: String,
    color: String,
    alpha: f32,
    rows_above: u32,
}

const SIZE: u32 = 60;
const SPLIT: u32 = 4;
fn main() {
    let img = ImageReader::open("images/irving3.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let processed = img
        .resize(SIZE, SIZE, image::imageops::FilterType::Lanczos3)
        .adjust_contrast(40.0)
        .grayscale();
    dbg!(processed.dimensions());
    processed.save("images/shrek_processed.jpeg").unwrap();
    let (width, height) = processed.dimensions();

    let mut rows_per_text = height;
    while rows_per_text * (width + 1) >= 512 {
        rows_per_text -= 1;
    }
    if rows_per_text % 2 != 0 {
        rows_per_text -= 1;
    }

    let mut texts: Vec<(Text, Text)> = vec![];
    let mut rows_above = 0;
    while rows_above < height {
        texts.extend((0..SPLIT).map(|d| {
            let mut s_1 = String::new();
            let mut s_2 = String::new();
            for y in rows_above..(rows_above + rows_per_text).min(height) {
                let s = if y % 2 == 0 { &mut s_1 } else { &mut s_2 };
                for x in 0..width {
                    let pixel = processed.get_pixel(x, y);
                    let color = pixel.0[0] as f32;
                    let div = 255.0 / SPLIT as f32;
                    if (color > div * d as f32) && (color <= div * (d as f32 + 1.0)) {
                        s.push_str("◼");
                    } else {
                        s.push_str(" ");
                    }
                }
                s.push_str("\n");
            }
            let alpha = (d as f32 / SPLIT as f32) * 0.8 + 0.2;
            (
                Text {
                    text: s_1,
                    color: format!("#ffffff"),
                    alpha,
                    rows_above,
                },
                Text {
                    text: s_2,
                    color: format!("#ffffff"),
                    alpha,
                    rows_above,
                },
            )
        }));
        rows_above += rows_per_text;
    }

    // save texts to json

    let mut test_img = RgbImage::new(SIZE, SIZE);
    for (text_1, text_2) in texts.iter() {
        for (text, downshift) in &[(text_1, 0), (text_2, 1)] {
            for (y, line) in text.text.lines().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    if c == '◼' {
                        let color = image::Rgb([255, 255, 255]);
                        let (x, y) = (
                            x as u32,
                            (y * 2 + downshift + text.rows_above as usize) as u32,
                        );
                        if y >= height {
                            continue;
                        }
                        let pixel = test_img.get_pixel(x, y);
                        let new_color = weighted_color(color, *pixel, text.alpha);
                        test_img.put_pixel(x, y, new_color);
                    }
                }
            }
        }
    }
    test_img.save("images/shrek_test.jpeg").unwrap();
    let json = serde_json::to_string(&texts).unwrap();
    fs::write("out/input.json", json).unwrap();
}

fn weighted_color(color: image::Rgb<u8>, pixel: image::Rgb<u8>, alpha: f32) -> image::Rgb<u8> {
    let color = color.0;
    let pixel = pixel.0;
    let new_color = [
        (color[0] as f32 * alpha + pixel[0] as f32 * (1.0 - alpha)) as u8,
        (color[1] as f32 * alpha + pixel[1] as f32 * (1.0 - alpha)) as u8,
        (color[2] as f32 * alpha + pixel[2] as f32 * (1.0 - alpha)) as u8,
    ];
    image::Rgb(new_color)
}
