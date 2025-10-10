pub mod hitable;
pub mod sphere;
pub mod triangle;
pub mod quad;
pub mod pyramid;
pub mod ray;
pub mod camera;
pub mod scene;
pub mod material;
pub mod raytracer;
pub mod teapot;
pub mod plane;
pub mod aabb;
pub mod bvh;
pub mod texture;

pub mod randomutil;
pub mod timeutil;

use clap::Parser;

#[derive(Parser)]
#[command(name = "Raytracer")]
#[command(version = "1.0")]
struct Args {
    /// Scene to render
    #[arg(short, long, default_value = "default")]
    #[arg(value_parser = ["default", "random", "test", "teapot", "motionblur", "2spheres", "2perlinspheres"])]
    scene: String,

    /// Output file path
    #[arg(short, long)]
    out: Option<String>,

    /// Width in pixels
    #[arg(short = 'x', long, default_value = "64")]
    nx: u32,

    /// Height in pixels
    #[arg(short = 'y', long, default_value = "48")]
    ny: u32,

    /// Samples per pixel
    #[arg(short = 'p', long, default_value = "100")]
    samples_per_pixel: u32,

    /// Random seed
    #[arg(short = 'e', long)]
    seed: Option<String>,

    /// Output image width (defaults to nx if not specified)
    #[arg(short, long)]
    width: Option<u32>,

    /// Output image height (defaults to ny if not specified)
    #[arg(short = 'h', long)]
    height: Option<u32>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(if args.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Warn
        })
        .init();

    let width = args.width.unwrap_or(args.nx);
    let height = args.height.unwrap_or(args.ny);

    raytracer::trace(
        args.nx,
        args.ny,
        width,
        height,
        args.samples_per_pixel,
        Some(&args.scene),
        args.out.as_deref(),
        args.seed.as_deref(),
        args.verbose,
    );
}
