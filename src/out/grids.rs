use serde::Serialize;

use crate::{
    cli::Args,
    grid::{Grid, GridColor},
};

#[derive(Debug, Clone, Serialize)]
pub struct OutGridColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl OutGridColor {
    pub fn from_grid_color(color: &GridColor) -> Self {
        Self {
            r: color.color.0[0],
            g: color.color.0[1],
            b: color.color.0[2],
            a: color.alpha,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OutGrid(pub String);

impl OutGrid {
    pub fn from_grid(grid: &Grid) -> Self {
        Self(
            grid.0
                .iter()
                .flatten()
                .map(|&x| if x { '1' } else { '0' })
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Layer {
    pub color: OutGridColor,
    pub grid: OutGrid,
}

#[derive(Debug, Clone, Serialize)]
pub struct OutputImage {
    pub width: u32,
    pub height: u32,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VideoLayer {
    pub color: OutGridColor,
    pub grid_frames: Vec<OutGrid>,
}
#[derive(Debug, Clone, Serialize)]
pub struct OutputVideo {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub layers: Vec<VideoLayer>,
}

pub fn save_image(grids: &[(Grid, GridColor)], path: &str) {
    let width = grids[0].0.width() as u32;
    let height = grids[0].0.height() as u32;
    let layers: Vec<Layer> = grids
        .into_iter()
        .map(|(grid, color)| Layer {
            color: OutGridColor::from_grid_color(color),
            grid: OutGrid::from_grid(grid),
        })
        .collect();
    let output = OutputImage {
        width,
        height,
        layers,
    };
    save_json(&output, path);
}

pub fn save_video(grids: &[(Vec<Grid>, GridColor)], path: &str, args: &Args) {
    let width = grids[0].0[0].width() as u32;
    let height = grids[0].0[0].height() as u32;
    let fps = args.fps;
    let layers: Vec<VideoLayer> = grids
        .into_iter()
        .map(|(grids, color)| VideoLayer {
            color: OutGridColor::from_grid_color(color),
            grid_frames: grids
                .into_iter()
                .map(|grid| OutGrid::from_grid(grid))
                .collect(),
        })
        .collect();
    let output = OutputVideo {
        width,
        height,
        fps,
        layers,
    };
    save_json(&output, path);
}

pub fn save_json<T: Serialize>(data: &T, path: &str) {
    let json = serde_json::to_string_pretty(data).expect("Unable to serialize");
    std::fs::write(path, json).expect("Unable to write file");
}
