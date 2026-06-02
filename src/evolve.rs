use rayon::iter::{IntoParallelIterator, ParallelIterator};

use image::{DynamicImage, GenericImageView, Pixel, RgbImage};
use rand;

use crate::cli::Args;
use crate::color::{color_distance, should_replace_pixel, weight_alpha};
use crate::grid::{apply_grid, Grid, GridColor};

pub fn evolve_image(img: &DynamicImage, args: &Args) -> Vec<(Grid, GridColor)> {
    let frames = [img.clone()];
    evolve_image_key_frames(&frames, &frames.clone(), args)
        .into_iter()
        .map(|(grid, color)| (grid[0].clone(), color))
        .collect()
}

pub fn evolve_image_frames(
    img_frames: &[DynamicImage],
    args: &Args,
) -> Vec<(Vec<Grid>, GridColor)> {
    let key_frames = extract_key_frames(img_frames, args);

    evolve_image_key_frames(img_frames, &key_frames, args)
}

fn extract_key_frames(img_frames: &[DynamicImage], _args: &Args) -> Vec<DynamicImage> {
    const TOTAL_KEY_FRAMES: usize = 5;
    let interval = img_frames.len() / TOTAL_KEY_FRAMES;
    let mut key_frames = Vec::new();
    for i in 0..TOTAL_KEY_FRAMES {
        key_frames.push(img_frames[i * interval].clone());
    }
    key_frames
}

fn evolve_image_key_frames(
    img_frames: &[DynamicImage],
    key_frames: &[DynamicImage],
    args: &Args,
) -> Vec<(Vec<Grid>, GridColor)> {
    let width = img_frames[0].width();
    let height = img_frames[0].height();

    let mut layer_imgs: Vec<_> = std::iter::repeat(RgbImage::new(width, height))
        .take(key_frames.len())
        .collect();

    let mut colors = Vec::new();

    let intial_color_distance_sum = fitness(&GridColor::empty(), &key_frames, &layer_imgs);

    for layer in 0..args.layers {
        println!("layer {}", layer);

        let grid_color = evolve_grid_gen(&key_frames, &layer_imgs, &args);

        let total_percent_improvement = 100.0
            * (intial_color_distance_sum - fitness(&grid_color, &key_frames, &layer_imgs))
            / intial_color_distance_sum;

        println!("\tnow {}% matching target image", total_percent_improvement);

        let grids: Vec<_> = layer_imgs
            .iter()
            .zip(key_frames)
            .map(|(layer_img, img)| Grid::from_color(img, layer_img, &grid_color))
            .collect();

        for (grid, layer_img) in grids.iter().zip(layer_imgs.iter_mut()) {
            apply_grid(layer_img, &grid_color, &grid);
        }

        colors.push(grid_color);
    }

    // colors
    //     .into_iter()
    //     .map(|color| {
    //         let grids = img_frames
    //             .iter()
    //             .map(|img| Grid::from_color(img, &layer_imgs[0], &color))
    //             .collect();
    //         (grids, color)
    //     })
    //     .collect()

    let mut grids: Vec<_> = colors
        .iter()
        .map(|color| (Vec::new(), color.clone()))
        .collect();
    for img in img_frames {
        let mut layer_img = RgbImage::new(width, height);
        for (grid, color) in grids.iter_mut() {
            let new_grid = Grid::from_color(img, &layer_img, color);
            apply_grid(&mut layer_img, color, &new_grid);
            grid.push(new_grid);
        }
    }
    grids
}

fn evolve_grid_gen(img_frames: &[DynamicImage], layer_imgs: &[RgbImage], args: &Args) -> GridColor {
    let with_fitness = |grid_color: GridColor| {
        let fitness = fitness(&grid_color, img_frames, layer_imgs);
        (fitness, grid_color)
    };
    let mut population = (0..100)
        .into_par_iter()
        .map(|_| {
            let grid_color = GridColor::random_from_imgs(img_frames, args);
            with_fitness(grid_color)
        })
        .collect::<Vec<_>>();
    population.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for generation in 0..20 {
        let mut new_population = population[0..20].to_vec();

        new_population.append(
            &mut (0..60)
                .into_par_iter()
                .map(|_| {
                    let (_, grid_color) = population[rand::random::<usize>() % 20].clone();
                    let mut new_grid_color = grid_color.clone();
                    new_grid_color.mutate(args);
                    with_fitness(new_grid_color)
                })
                .collect(),
        );

        new_population.append(
            &mut (0..20)
                .into_par_iter()
                .map(|_| {
                    let (_, mut grid_color) = population[rand::random::<usize>() % 100].clone();
                    grid_color.mutate(args);
                    with_fitness(grid_color)
                })
                .collect(),
        );
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

fn fitness(grid_color: &GridColor, img_frames: &[DynamicImage], layer_imgs: &[RgbImage]) -> f32 {
    layer_imgs
        .iter()
        .zip(img_frames)
        .map(|(layer_img, img)| {
            layer_img
                .enumerate_pixels()
                .map(|(x, y, &current_color)| {
                    let actual_color = img.get_pixel(x, y).to_rgb();
                    let candidate_color =
                        weight_alpha(grid_color.alpha, current_color, grid_color.color);
                    if should_replace_pixel(grid_color, current_color, actual_color) {
                        color_distance(candidate_color, actual_color)
                    } else {
                        color_distance(current_color, actual_color)
                    }
                })
                .sum::<f32>()
        })
        .sum()
}
