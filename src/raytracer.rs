use ray::Ray3;
use scene;
use randomutil;
use material;
use hitable::Hitable;

use cgmath::{
    Vector3,
    InnerSpace,
    Zero,
    ElementWise,
};
use std::{
    io,
    path,
    fs,
    env,
    f32,
    thread,
};
use std::sync::{
    Arc,
    Mutex,
};
use std::io::Write;
use rand::{
    Rng,
    thread_rng,
};
use std::time::Instant;
use image;
use num_cpus;

pub fn trace(nx: u32,
             ny: u32,
             width: u32,
             height: u32,
             ns: u32,
             scene: Option<&str>,
             out: Option<&str>,
             seed: Option<&str>,
             verbose: bool) {
    let start = Instant::now();
    let aspect = nx as f32 / ny as f32;
    let seed_val = seed.map(|s| s.parse::<usize>().unwrap()).unwrap_or(thread_rng().next_u32() as usize);

    randomutil::thread_seedable_reseed(seed_val);

    let (world, camera) = match scene {
        Some("random") => scene::random_scene(aspect, false),
        Some("motionblur") => scene::random_scene(aspect, true),
        Some("default") => scene::default_scene(aspect),
        Some("teapot") => scene::teapot_scene(aspect),
        Some("test") => scene::test_scene(aspect),
        Some("2spheres") => scene::two_spheres(aspect),
        Some("2perlinspheres") => scene::two_perlin_spheres(aspect),
        _ => {
            if verbose {
                info!("Unrecognised scene type, using default");
            }
            scene::default_scene(aspect)
        }
    };

    let world_created = Instant::now();
    if verbose {
        let diff = world_created.duration_since(start);
        info!("Created world in {:.*} seconds", 2, diff.as_secs() as f32 + diff.subsec_nanos() as f32 / 1_000_000_000f32);
        info!("Starting ray trace with the following parameters");
        info!(" -    nx : {}", nx);
        info!(" -    ny : {}", ny);
        info!(" -    ns : {}", ns);
        info!(" -  seed : {}", seed_val);
        info!(" - scene : {}", scene.unwrap_or("default"));
    }

    let num = num_cpus::get();
    let mut tasks = Vec::new();

    let rows = (ny as f32 / num as f32) as u32;

    let arc_world = Arc::new(world);
    let arc_img = Arc::new(Mutex::new(image::ImageBuffer::new(nx, ny)));

    for cpu in 0..num {
        let inner_world = arc_world.clone();
        let inner_img = arc_img.clone();

        tasks.push(thread::spawn(move || {
            let top = if cpu == num - 1 || (cpu as u32 + 1) * rows > ny {
                ny
            } else {
                (cpu as u32 + 1) * rows
            };

            let mut sub_image = image::ImageBuffer::new(nx, ny);

            for inverse_j in (cpu as u32 * rows)..top {
                let j = ny - inverse_j - 1;
                for i in 0..nx {
                    let mut output_colour = Vector3::<f32>::zero();

                    for _ in 0..ns {
                        let u = (i as f32 + randomutil::unit_distribution()) / nx as f32;
                        let v = (j as f32 + randomutil::unit_distribution()) / ny as f32;
                        let r = camera.get_ray(u, v);

                        output_colour += colour(&r, &*inner_world, 0)
                    }

                    output_colour /= ns as f32;
                    output_colour = Vector3::new(
                        output_colour.x.sqrt(),
                        output_colour.y.sqrt(),
                        output_colour.z.sqrt(),
                    );

                    sub_image.put_pixel(i, inverse_j - (cpu as u32 * rows), image::Rgb {
                        data: [
                            (255.99 * output_colour.x) as u8,
                            (255.99 * output_colour.y) as u8,
                            (255.99 * output_colour.z) as u8,
                        ],
                    });
                }
            }

            let mut img_data = inner_img.lock().unwrap();

            for i in 0..nx {
                for inverse_j in (cpu as u32 * rows)..top {
                    img_data.put_pixel(i, inverse_j, *sub_image.get_pixel(i, inverse_j - (cpu as u32 * rows)));
                }
            }
        }));
    }

    for task in tasks {
        let _ = task.join();
    }

    let raytrace_complete = Instant::now();
    if verbose {
        let diff = raytrace_complete.duration_since(world_created);
        info!("Raytrace complete in {:.*} seconds", 2, diff.as_secs() as f32 + diff.subsec_nanos() as f32 / 1_000_000_000f32);
    }

    let output_path = out
        .map(|output_file| path::Path::new(&output_file).to_path_buf())
        .unwrap_or_else(|| {
            let temp_path = env::temp_dir().join(path::Path::new("raytracer"));
            if !temp_path.exists() {
                fs::create_dir(&temp_path).unwrap();
            }
            temp_path.join("out.png")
        });

    let ref mut output_file = fs::File::create(&output_path).unwrap();

    if width != nx || height != ny {
        image::ImageRgb8(arc_img.lock().unwrap().clone()).resize_exact(width, height, image::FilterType::CatmullRom).save(output_file, image::PNG).unwrap();
    } else {
        image::ImageRgb8(arc_img.lock().unwrap().clone()).save(output_file, image::PNG).unwrap();
    }

    let write_complete = Instant::now();
    if verbose {
        let diff = write_complete.duration_since(raytrace_complete);
        let total_diff = write_complete.duration_since(start);
        info!("Write complete in {:.*} seconds", 2, diff.as_secs() as f32 + diff.subsec_nanos() as f32 / 1_000_000_000f32);
        info!("Program ran in {:.*} seconds", 2, total_diff.as_secs() as f32 + total_diff.subsec_nanos() as f32 / 1_000_000_000f32);
    }

    writeln!(&mut io::stdout(), "{}", output_path.into_os_string().into_string().unwrap()).unwrap();
}

fn colour(ray: &Ray3<f32>, hitable: &Hitable, depth: u32) -> Vector3<f32> {
        hitable.hit(ray, 0.001, f32::MAX).and_then(|rec| {
            if rec.material.is_some() {
                return rec.material.clone().map(|material| {
                    if depth < 50 {
                        let (attenuation, scattered, should_scatter) = material::scatter(&material, &ray, &rec);
                        if should_scatter {
                            return attenuation.mul_element_wise(colour(&scattered, hitable, depth + 1));
                        }
                    }
                    Vector3::zero()
                })
            } else {
                None
            }
        }).unwrap_or_else(|| {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        })
}
