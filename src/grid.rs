use image::{DynamicImage, GenericImageView, Pixel, Rgb, RgbImage};

use crate::{
    cli::Args,
    color::{should_replace_pixel, weight_alpha},
};

#[derive(Debug, Clone)]
pub struct GridColor {
    pub color: Rgb<u8>,
    pub alpha: f32,
}

#[derive(Debug, Clone)]
pub struct Grid(pub Vec<Vec<bool>>);

impl Grid {
    pub fn from_color(img: &DynamicImage, layer_img: &RgbImage, grid_color: &GridColor) -> Self {
        Self(
            layer_img
                .enumerate_rows()
                .map(|(_, row)| {
                    row.map(|(x, y, &current_color)| {
                        let actual_color = img.get_pixel(x, y).to_rgb();
                        should_replace_pixel(grid_color, current_color, actual_color)
                    })
                    .collect()
                })
                .collect(),
        )
    }
    pub fn width(&self) -> usize {
        self.0[0].len()
    }
    pub fn height(&self) -> usize {
        self.0.len()
    }
}

impl GridColor {
    pub fn random_from_img(img: &DynamicImage, args: &Args) -> Self {
        let color = img
            .get_pixel(
                rand::random::<u32>() % img.width(),
                rand::random::<u32>() % img.height(),
            )
            .to_rgb();
        let alpha = args.min_alpha + rand::random::<f32>() * (args.max_alpha - args.min_alpha);
        Self { color, alpha }
    }

    pub fn random_from_imgs(imgs: &[DynamicImage], args: &Args) -> Self {
        let img = &imgs[rand::random::<usize>() % imgs.len()];
        Self::random_from_img(img, args)
    }

    pub fn mutate(&mut self, args: &Args) {
        fn map(val: u8) -> u8 {
            (val as f32 + (rand::random::<f32>() - 0.5) * 10.0)
                .round()
                .clamp(0.0, 256.0) as u8
        }
        if rand::random::<f32>() > 0.5 {
            self.color = Rgb([map(self.color[0]), map(self.color[1]), map(self.color[2])]);
        } else {
            self.alpha = self.alpha + (rand::random::<f32>() - 0.5) * 0.1;
            self.alpha = self.alpha.clamp(args.min_alpha, args.max_alpha);
        }
    }
    pub fn empty() -> Self {
        Self {
            color: Rgb([0, 0, 0]),
            alpha: 0.0,
        }
    }
}

pub fn apply_grid(layer_img: &mut RgbImage, grid_color: &GridColor, grid: &Grid) {
    for (x, y, pixel) in layer_img.enumerate_pixels_mut() {
        let cell = grid.0[y as usize][x as usize];
        if cell {
            let new_color = weight_alpha(grid_color.alpha, *pixel, grid_color.color);
            *pixel = new_color;
        }
    }
}
