use crate::grid::{apply_grid, Grid, GridColor};
use image::RgbImage;

pub fn save_image(grids: &[(Grid, GridColor)], path: &str) {
    let mut layer_img = RgbImage::new(grids[0].0.width() as u32, grids[0].0.height() as u32);

    let frames: Vec<_> = grids
        .iter()
        .map(|(grid, color)| {
            apply_grid(&mut layer_img, color, grid);
            layer_img.clone()
        })
        .collect();

    super::gif::save_gif_from_frames(&frames, path, std::time::Duration::from_millis(500));
}

pub fn save_video(grids: &[(Vec<Grid>, GridColor)], path: &str) {
    // just save the first frame
    save_image(
        &grids
            .iter()
            .map(|(grids, color)| (grids[0].clone(), color.clone()))
            .collect::<Vec<_>>(),
        path,
    );
}
