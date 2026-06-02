use clap::Parser;

/// Create layers of single-color pixel grids that approximate an image
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the image to approximate
    #[arg(short, long)]
    pub input_file: String,

    /// Output directory
    #[arg(short, long, default_value = "./layers_output")]
    pub output_dir: String,

    /// Size of the output image
    #[arg(short, long, default_value_t = 150)]
    pub size: u32,

    /// Number of layers to create
    #[arg(short, long, default_value_t = 8)]
    pub layers: u32,

    /// Minimum transparency of the layers
    #[arg(short, long, default_value_t = 0.2)]
    pub min_alpha: f32,

    /// Maximum transparency of the layers
    #[arg(short = 'M', long, default_value_t = 1.0)]
    pub max_alpha: f32,

    /// Whether input is a video or not
    #[arg(short, long)]
    pub video: bool,

    /// Video start time (in seconds)
    #[arg(long, default_value_t = 0.0)]
    pub start_time: f32,

    /// Video end time (in seconds)
    #[arg(long, default_value_t = 10.0)]
    pub end_time: f32,

    /// Frames per second of output
    #[arg(long, default_value_t = 30)]
    pub fps: u32,
}
