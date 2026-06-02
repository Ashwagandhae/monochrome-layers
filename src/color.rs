use image::Rgb;

use crate::grid::GridColor;

pub fn weight_alpha(alpha: f32, below_color: Rgb<u8>, above_color: Rgb<u8>) -> Rgb<u8> {
    let alpha = alpha.clamp(0.0, 1.0);
    let r = (below_color[0] as f32 * (1.0 - alpha) + above_color[0] as f32 * alpha).round() as u8;
    let g = (below_color[1] as f32 * (1.0 - alpha) + above_color[1] as f32 * alpha).round() as u8;
    let b = (below_color[2] as f32 * (1.0 - alpha) + above_color[2] as f32 * alpha).round() as u8;
    Rgb([r, g, b])
}

pub fn color_distance(color_1: Rgb<u8>, color_2: Rgb<u8>) -> f32 {
    let r = color_1[0] as f32 - color_2[0] as f32;
    let g = color_1[1] as f32 - color_2[1] as f32;
    let b = color_1[2] as f32 - color_2[2] as f32;
    (r * r + g * g + b * b).sqrt()
}

pub fn should_replace_pixel(
    grid_color: &GridColor,
    current_color: Rgb<u8>,
    actual_color: Rgb<u8>,
) -> bool {
    let candidate_color = weight_alpha(grid_color.alpha, current_color, grid_color.color);
    color_distance(candidate_color, actual_color) < color_distance(current_color, actual_color)
}
