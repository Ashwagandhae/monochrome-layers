use clap::Parser;
use image::io::Reader as ImageReader;

pub mod cli;
pub mod color;
pub mod evolve;
pub mod grid;
pub mod out;
pub mod process;

fn main() {
    let args = cli::Args::parse();
    if args.video {
        run_video(args);
    } else {
        run_image(args);
    }
}

fn run_image(args: cli::Args) {
    let img = ImageReader::open(&args.input_file)
        .unwrap()
        .decode()
        .unwrap();

    let img = process::process_image(img, &args);

    let grids = evolve::evolve_image(&img, &args);

    out::save_outputs_for_image(args, grids);
}

fn run_video(args: cli::Args) {
    let frames = process::process_and_load_video(&args);

    let grids = evolve::evolve_image_frames(&frames, &args);

    out::save_outputs_for_video(args, grids);
}
