use std::fs;

use crate::{
    cli::Args,
    grid::{Grid, GridColor},
};

pub mod gif;
pub mod grids;
pub mod layers;
pub mod paint;

pub fn save_outputs_for_image(args: Args, grids: Vec<(Grid, GridColor)>) {
    let output_dir = args.output_dir.clone();
    fs::create_dir_all(&output_dir).unwrap();

    grids::save_image(&grids, &format!("{}/grids.json", output_dir));
    paint::save_image(&grids, &format!("{}/paint.gif", output_dir));
    layers::save_image(&grids, &format!("{}/layers.jpg", output_dir));
}

pub fn save_outputs_for_video(args: Args, grids: Vec<(Vec<Grid>, GridColor)>) {
    let output_dir = args.output_dir.clone();
    fs::create_dir_all(&output_dir).unwrap();

    grids::save_video(&grids, &format!("{}/grids.json", output_dir), &args);
    layers::save_video(&grids, &format!("{}/layers.gif", output_dir,), &args);
    paint::save_video(&grids, &format!("{}/paint.gif", output_dir));
}
