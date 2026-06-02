use std::path::Path;

use image::DynamicImage;

use crate::cli::Args;

pub fn process_image(img: DynamicImage, args: &Args) -> DynamicImage {
    let size = args.size;

    let img = img.resize(size, size, image::imageops::FilterType::Lanczos3);

    img
}

use video_rs::decode::Decoder;

pub fn process_and_load_video(args: &Args) -> Vec<DynamicImage> {
    let mut decoder = Decoder::new(Path::new(&args.input_file)).unwrap();

    let frame_time = 1.0 / args.fps as f32;
    let mut last_frame_timestamp = 0.0;
    decoder
        .decode_iter()
        .skip_while(|frame| {
            frame
                .as_ref()
                .is_ok_and(|(time, _)| time.as_secs() <= args.start_time)
        })
        .take_while(|frame| {
            frame
                .as_ref()
                .is_ok_and(|(time, _)| time.as_secs() <= args.end_time)
        })
        .filter_map(|frame| {
            let (time, frame) = frame.unwrap();
            let time = time.as_secs();
            if time - last_frame_timestamp < frame_time {
                return None;
            }
            last_frame_timestamp = time;
            let img = image::RgbImage::from_raw(
                frame.shape()[1] as u32,
                frame.shape()[0] as u32,
                frame.as_slice().unwrap().to_vec(),
            )
            .unwrap();
            Some(process_image(image::DynamicImage::ImageRgb8(img), args))
        })
        .collect()
}
