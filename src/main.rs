extern crate cgmath;
extern crate rand;
extern crate image;
extern crate clap;
extern crate num_cpus;
extern crate noise;

#[macro_use]
extern crate log;
extern crate stderrlog;

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

use clap::{
    Arg,
    App,
};

fn main() {
    let matches = App::new("Raytracer")
        .version("1.0")
        .arg(Arg::with_name("scene")
            .display_order(0)
            .short("s")
            .long("scene")
            .possible_values(&["default", "random", "test", "teapot", "motionblur", "2spheres", "2perlinspheres"])
            .default_value("default")
            .takes_value(true))
        .arg(Arg::with_name("out")
            .display_order(1)
            .short("o")
            .long("out")
            .value_name("FILE")
            .takes_value(true))
        .arg(Arg::with_name("nx")
            .display_order(2)
            .short("x")
            .long("nx")
            .default_value("64")
            .value_name("UINT")
            .takes_value(true))
        .arg(Arg::with_name("ny")
            .display_order(3)
            .short("y")
            .long("ny")
            .default_value("48")
            .value_name("UINT")
            .takes_value(true))
        .arg(Arg::with_name("samples-per-pixel")
            .display_order(4)
            .short("p")
            .long("samples-per-pixel")
            .default_value("100")
            .value_name("UINT")
            .takes_value(true))
        .arg(Arg::with_name("seed")
            .display_order(5)
            .short("e")
            .long("seed")
            .value_name("UINT")
            .takes_value(true))
        .arg(Arg::with_name("width")
            .display_order(5)
            .short("w")
            .long("width")
            .value_name("UINT")
            .takes_value(true))
        .arg(Arg::with_name("height")
            .display_order(6)
            .short("h")
            .long("height")
            .value_name("UINT")
            .takes_value(true))
        .arg(Arg::with_name("verbose")
            .display_order(7)
            .short("v")
            .long("verbose")
            .takes_value(false))
        .get_matches();

    let verbose = matches.is_present("verbose");
    let nx = matches.value_of("nx").unwrap().parse::<u32>().unwrap();
    let ny = matches.value_of("ny").unwrap().parse::<u32>().unwrap();
    let ns = matches.value_of("samples-per-pixel").unwrap().parse::<u32>().unwrap();
    let width = matches.value_of("width").map_or(nx, |w| w.parse::<u32>().unwrap());
    let height = matches.value_of("height").map_or(ny, |h| h.parse::<u32>().unwrap());
    let scene = matches.value_of("scene");
    let seed = matches.value_of("seed");
    let out = matches.value_of("out");

    stderrlog::new()
        .module(module_path!())
        .quiet(!verbose)
        .timestamp(stderrlog::Timestamp::Second)
        .verbosity(3)
        .init()
        .unwrap();

    raytracer::trace(nx, ny, width, height, ns, scene, out, seed, verbose);
}
