use std::fs;

use gif::{Frame, Repeat};
use image::RgbImage;

pub fn save_gif_from_frames(frames: &[RgbImage], path: &str, frame_delay: std::time::Duration) {
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
        frame.delay = frame_delay.as_millis() as u16 / 10;

        encoder.write_frame(&frame).unwrap();
    }
}
