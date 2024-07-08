use clap::Parser;
use gif::{self, Frame, Repeat};
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, Pixel, Rgb, RgbImage};
use rand;
use serde::Serialize;
use serde_json;
use std::fs;

/// Create layers of single-color pixel grids that approximate an image
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the image to approximate
    #[arg(short, long)]
    input_file: String,

    /// Output directory
    #[arg(short, long, default_value = "./layers_output")]
    output_dir: String,

    /// Size of the output image
    #[arg(short, long, default_value_t = 150)]
    output_size: u32,

    /// Number of layers to create
    #[arg(short, long, default_value_t = 8)]
    layers: u32,

    /// Minimum transparency of the layers
    #[arg(short, long, default_value_t = 0.2)]
    min_alpha: f32,

    /// Maximum transparency of the layers
    #[arg(short = 'M', long, default_value_t = 1.0)]
    max_alpha: f32,
}

#[derive(Debug, Clone, Serialize)]
struct GridColor {
    color: [u8; 3],
    alpha: f32,
}

impl GridColor {
    fn random_from_img(img: &DynamicImage, args: &Args) -> Self {
        let color = img
            .get_pixel(
                rand::random::<u32>() % img.width(),
                rand::random::<u32>() % img.height(),
            )
            .to_rgb();
        let alpha = args.min_alpha + rand::random::<f32>() * (args.max_alpha - args.min_alpha);
        Self {
            color: color.0,
            alpha,
        }
    }

    fn mutate(&mut self, args: &Args) {
        fn map(val: u8) -> u8 {
            (val as f32 + (rand::random::<f32>() - 0.5) * 10.0)
                .round()
                .clamp(0.0, 256.0) as u8
        }
        if rand::random::<f32>() > 0.5 {
            self.color = [map(self.color[0]), map(self.color[1]), map(self.color[2])];
        } else {
            self.alpha = self.alpha + (rand::random::<f32>() - 0.5) * 0.1;
            self.alpha = self.alpha.clamp(args.min_alpha, args.max_alpha);
        }
    }
    fn empty() -> Self {
        Self {
            color: [0, 0, 0],
            alpha: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct Grid(Vec<Vec<bool>>);

fn main() {
    let args = Args::parse();
    let img = ImageReader::open(args.input_file.clone())
        .unwrap()
        .decode()
        .unwrap();
    let size = args.output_size;
    let layers = args.layers;

    let img = img.resize(size, size, image::imageops::FilterType::Lanczos3);
    dbg!(img.dimensions());

    let output_dir = args.output_dir.clone();

    fs::create_dir_all(&output_dir).unwrap();
    img.save(format!("{}/processed.jpg", output_dir)).unwrap();

    let mut layer_img = RgbImage::new(img.width(), img.height());
    let mut grids = Vec::new();
    let mut frames = vec![layer_img.clone()];

    let intial_color_distance_sum = fitness(&GridColor::empty(), &img, &layer_img);

    for layer in 0..layers {
        println!("layer {}", layer);

        let grid_color = evolve_grid(&img, &layer_img, &args);

        let total_percent_improvement = 100.0
            * (intial_color_distance_sum - fitness(&grid_color, &img, &layer_img))
            / intial_color_distance_sum;

        println!("\tnow {}% matching target image", total_percent_improvement);

        let grid = grid_from_color(&img, &layer_img, &grid_color);

        apply_grid(&mut layer_img, &grid_color, &grid);
        frames.push(layer_img.clone());

        grids.push((grid_color, grid));
    }

    layer_img
        .save(format!("{}/layers.jpg", output_dir))
        .unwrap();
    save_frames_gif(frames, &format!("{}/layers.gif", output_dir));
    save_grids_json(grids, &format!("{}/grids.json", output_dir));
}

fn save_grids_json(grids: Vec<(GridColor, Grid)>, path: &str) {
    let json = serde_json::to_string(&grids).unwrap();
    fs::write(path, json).unwrap();
}

fn save_frames_gif(frames: Vec<RgbImage>, path: &str) {
    let mut encoder = gif::Encoder::new(
        fs::File::create(path).unwrap(),
        frames[0].width() as u16,
        frames[0].height() as u16,
        &[],
    )
    .unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for frame in frames {
        let mut vec = Vec::new();
        for pixel in frame.pixels() {
            vec.push(pixel[0]);
            vec.push(pixel[1]);
            vec.push(pixel[2]);
        }
        let mut frame = Frame::from_rgb(frame.width() as u16, frame.height() as u16, &vec);
        frame.delay = 100;

        encoder.write_frame(&frame).unwrap();
    }
}

fn evolve_grid(img: &DynamicImage, layer_img: &RgbImage, args: &Args) -> GridColor {
    let with_fitness = |grid_color: GridColor| {
        let fitness = fitness(&grid_color, img, layer_img);
        (fitness, grid_color)
    };
    let mut population = (0..100)
        .map(|_| {
            let grid_color = GridColor::random_from_img(img, args);
            with_fitness(grid_color)
        })
        .collect::<Vec<_>>();
    population.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for generation in 0..20 {
        let mut new_population = population[0..20].to_vec();
        for _ in 0..60 {
            let (_, grid_color) = population[rand::random::<usize>() % 20].clone();
            let mut new_grid_color = grid_color.clone();
            new_grid_color.mutate(args);
            new_population.push(with_fitness(new_grid_color));
        }
        for _ in 0..20 {
            let (_, mut grid_color) = population[rand::random::<usize>() % 100].clone();
            grid_color.mutate(args);
            new_population.push(with_fitness(grid_color));
        }
        new_population.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let best = &new_population[0];
        let best_color_distance_sum = best.0;
        println!(
            "\tgeneration {}, best fitness: {}",
            generation, best_color_distance_sum
        );
        population = new_population;
    }
    population[0].1.clone()
}

fn fitness(grid_color: &GridColor, img: &DynamicImage, layer_img: &RgbImage) -> f32 {
    layer_img
        .enumerate_pixels()
        .map(|(x, y, &current_color)| {
            let actual_color = img.get_pixel(x, y).to_rgb();
            let candidate_color = weight_alpha(
                grid_color.alpha,
                current_color,
                image::Rgb(grid_color.color),
            );
            if should_replace_pixel(grid_color, current_color, actual_color) {
                color_distance(candidate_color, actual_color)
            } else {
                color_distance(current_color, actual_color)
            }
        })
        .sum()
}

fn grid_from_color(img: &DynamicImage, layer_img: &RgbImage, grid_color: &GridColor) -> Grid {
    let grid_vec: Vec<Vec<bool>> = layer_img
        .enumerate_rows()
        .map(|(_, row)| {
            row.map(|(x, y, &current_color)| {
                let actual_color = img.get_pixel(x, y).to_rgb();
                should_replace_pixel(grid_color, current_color, actual_color)
            })
            .collect()
        })
        .collect();
    Grid(grid_vec)
}

fn should_replace_pixel(
    grid_color: &GridColor,
    current_color: Rgb<u8>,
    actual_color: Rgb<u8>,
) -> bool {
    let candidate_color = weight_alpha(
        grid_color.alpha,
        current_color,
        image::Rgb(grid_color.color),
    );
    color_distance(candidate_color, actual_color) < color_distance(current_color, actual_color)
}

fn apply_grid(layer_img: &mut RgbImage, grid_color: &GridColor, grid: &Grid) {
    for (x, y, pixel) in layer_img.enumerate_pixels_mut() {
        let cell = grid.0[y as usize][x as usize];
        if cell {
            let new_color = weight_alpha(grid_color.alpha, *pixel, image::Rgb(grid_color.color));
            *pixel = new_color;
        }
    }
}

fn weight_alpha(alpha: f32, below_color: Rgb<u8>, above_color: Rgb<u8>) -> Rgb<u8> {
    let alpha = alpha.clamp(0.0, 1.0);
    let r = (below_color[0] as f32 * (1.0 - alpha) + above_color[0] as f32 * alpha).round() as u8;
    let g = (below_color[1] as f32 * (1.0 - alpha) + above_color[1] as f32 * alpha).round() as u8;
    let b = (below_color[2] as f32 * (1.0 - alpha) + above_color[2] as f32 * alpha).round() as u8;
    Rgb([r, g, b])
}

fn color_distance(color_1: Rgb<u8>, color_2: Rgb<u8>) -> f32 {
    let r = color_1[0] as f32 - color_2[0] as f32;
    let g = color_1[1] as f32 - color_2[1] as f32;
    let b = color_1[2] as f32 - color_2[2] as f32;
    (r * r + g * g + b * b).sqrt()
}
