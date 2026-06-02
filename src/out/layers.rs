use image::RgbImage;

use crate::{
    cli::Args,
    grid::{apply_grid, Grid, GridColor},
};

pub fn save_image(grids: &[(Grid, GridColor)], path: &str) {
    img_with_all_layers_applied(grids).save(path).unwrap();
}

pub fn save_video(grids: &[(Vec<Grid>, GridColor)], path: &str, args: &Args) {
    let frames_count = grids[0].0.len();
    let frames: Vec<_> = (0..frames_count)
        .map(|frame| {
            img_with_all_layers_applied(
                &grids
                    .iter()
                    .map(|(grids, color)| (grids[frame].clone(), color.clone()))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    super::gif::save_gif_from_frames(
        &frames,
        path,
        std::time::Duration::from_secs_f32(1.0 / args.fps as f32),
    );
}

pub fn img_with_all_layers_applied(grids: &[(Grid, GridColor)]) -> RgbImage {
    let mut layer_img = RgbImage::new(grids[0].0.width() as u32, grids[0].0.height() as u32);

    for (grid, color) in grids {
        apply_grid(&mut layer_img, color, grid);
    }

    layer_img
}
