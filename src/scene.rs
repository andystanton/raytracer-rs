use camera::Camera;
use sphere::{
    Sphere,
    MovingSphere,
};
use quad::Quad;
use pyramid::Pyramid;
use teapot::Teapot;
use hitable::HitableList;
use material::Material;
use plane::Plane;
use randomutil::seedable_unit_distribution;
use texture::{
    ConstantTexture,
    ChequeredTexture,
    NoiseTexture,
};

use cgmath::{
    Vector3,
    Point3,
    InnerSpace,
    EuclideanSpace,
    Quaternion,
    One,
    Deg,
    Rotation3,
};

use std::f32;
use std::time::{
    Instant,
    Duration,
};

pub fn random_scene(aspect: f32, motion_blur: bool) -> (HitableList, Camera) {
    let aperture_open_time = Instant::now();
    let aperture_duration = Duration::from_millis(1000);

    let ground_level = 0f32;

    let mut world = HitableList::new()
        .with_hitable(Quad::new(
            [
                Point3::new(-30.0, ground_level, -30.0),
                Point3::new(30.0, ground_level, -30.0),
                Point3::new(30.0, ground_level, 30.0),
                Point3::new(-30.0, ground_level, 30.0),
            ],
            Quaternion::from_angle_y(Deg(10f32)),
            Material::TexturedLambertian {
                texture: ChequeredTexture::new(
                    ConstantTexture::new(Vector3::new(0.2, 0.3, 0.1)),
                    ConstantTexture::new(Vector3::new(0.9, 0.9, 0.9)),
                )
            },
        ))
        .with_hitable(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Material::Dielectric { ref_idx: 1.5 }))
        .with_hitable(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Material::TexturedLambertian { texture: ConstantTexture::new(Vector3::new(0.4, 0.2, 0.1)) }))
        .with_hitable(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Material::Metal { albedo: Vector3::new(0.7, 0.6, 0.5), fuzz: 0.0 }));

    let num = 11;

    for a_offset in 0..2 * num {
        let a = (a_offset - num) as f32;
        for b_offset in 0..2 * num {
            let b = (b_offset - num) as f32;
            let choose_mat = seedable_unit_distribution();
            let center = Point3::new(a + 0.9 * seedable_unit_distribution(), 0.2, b + 0.9 * seedable_unit_distribution());
            if (center.to_vec() - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    if motion_blur {
                        world.add_hitable(MovingSphere::new(center, center + Point3::new(0f32, 0.5 * seedable_unit_distribution(), 0f32).to_vec(), aperture_open_time, aperture_duration, 0.2, Material::Lambertian { albedo: Vector3::new(seedable_unit_distribution() * seedable_unit_distribution(), seedable_unit_distribution() * seedable_unit_distribution(), seedable_unit_distribution() * seedable_unit_distribution()) }));
                    } else {
                        world.add_hitable(Sphere::new(center, 0.2, Material::Lambertian { albedo: Vector3::new(seedable_unit_distribution() * seedable_unit_distribution(), seedable_unit_distribution() * seedable_unit_distribution(), seedable_unit_distribution() * seedable_unit_distribution()) }));
                    }
                } else if choose_mat < 0.95 {
                    if motion_blur {
                        world.add_hitable(MovingSphere::new(center, center + Point3::new(0f32, 0.5 * seedable_unit_distribution(), 0f32).to_vec(), aperture_open_time, aperture_duration, 0.2, Material::Metal { albedo: Vector3::new(0.5 * (1.0 + seedable_unit_distribution()), 0.5 * (1.0 + seedable_unit_distribution()), 0.5 * (1.0 + seedable_unit_distribution())), fuzz: 0.5 * seedable_unit_distribution() }));
                    } else {
                        world.add_hitable(Sphere::new(center, 0.2, Material::Metal { albedo: Vector3::new(0.5 * (1.0 + seedable_unit_distribution()), 0.5 * (1.0 + seedable_unit_distribution()), 0.5 * (1.0 + seedable_unit_distribution())), fuzz: 0.5 * seedable_unit_distribution() }));
                    }
                } else {
                    if motion_blur {
                        world.add_hitable(MovingSphere::new(center, center + Point3::new(0f32, 0.5 * seedable_unit_distribution(), 0f32).to_vec(), aperture_open_time, aperture_duration, 0.2, Material::Dielectric { ref_idx: 1.5 }));
                    } else {
                        world.add_hitable(Sphere::new(center, 0.2, Material::Dielectric { ref_idx: 1.5 }));
                    }
                }
            }
        }
    }

    let from = Point3::new(24.0, 2.0, 6.0);
    let at = Vector3::unit_y();
    let dist_to_focus = (from.to_vec() - at).magnitude();
    let aperture = 0.001;
    let camera = Camera::new(from, at, Vector3::unit_y(), 15.0, aspect, aperture, dist_to_focus, aperture_open_time, aperture_duration);

    (world, camera)
}

pub fn default_scene(aspect: f32) -> (HitableList, Camera) {
    let ground_level = -0.5;
    let world = HitableList::new()
        .with_hitable(Plane::new(Point3::new(0.0, ground_level, 0.0), Vector3::unit_y(), Material::Lambertian { albedo: Vector3::new(0.8, 0.8, 0.0) }))
        .with_hitable(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Material::Lambertian { albedo: Vector3::new(0.1, 0.2, 0.5) }))
        .with_hitable(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Material::Metal { albedo: Vector3::new(0.8, 0.6, 0.2), fuzz: 0.3 }))
        .with_hitable(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Material::Dielectric { ref_idx: 1.5 }));

    let from = Point3::new(6.0, 1.0, 2.0);
    let at = Vector3::new(0.0, 0.0, -1.1);
    let dist_to_focus = (from.to_vec() - at).magnitude();
    let aperture = 0.001;
    let camera = Camera::new(from, at, Vector3::unit_y(), 15.0, aspect, aperture, dist_to_focus, Instant::now(), Duration::from_millis(1000));

    (world, camera)
}

pub fn teapot_scene(aspect: f32) -> (HitableList, Camera) {
    let ground_level = 0.0;
    let teapot_scale = 2.5;

    let mut world = HitableList::new()
        .with_hitable(Quad::new(
            [
                Point3::new(-30.0, ground_level, -30.0),
                Point3::new(30.0, ground_level, -30.0),
                Point3::new(30.0, ground_level, 30.0),
                Point3::new(-30.0, ground_level, 30.0),
            ],
            Quaternion::from_angle_y(Deg(90f32)),
            Material::Lambertian { albedo: Vector3::new(0.5, 0.5, 0.5) }
        ))
        .with_hitable(Teapot::new(Point3::new(0.0, ground_level + teapot_scale / 2.0, 4.0), teapot_scale, Quaternion::one(), Material::Lambertian { albedo: Vector3::new(0.1, 0.2, 0.5) }))
        .with_hitable(Teapot::new(Point3::new(0.0, ground_level + teapot_scale / 2.0, -4.0), teapot_scale, Quaternion::one(), Material::Metal { albedo: Vector3::new(0.7, 0.6, 0.5), fuzz: 0.0 }))
        .with_hitable(Teapot::new(Point3::new(0.0, ground_level + teapot_scale / 2.0, 0.0), teapot_scale, Quaternion::one(), Material::Dielectric { ref_idx: 1.5 }));

    let num = 11;

    for a_offset in 0..2 * num {
        let a = (a_offset - num) as f32;
        for b_offset in 0..2 * num {
            let b = (b_offset - num) as f32;
            let choose_mat = seedable_unit_distribution();
            let center = Point3::new(a + 0.9 * seedable_unit_distribution(), 0.2 + ground_level, b + 0.9 * seedable_unit_distribution());
            if (center.to_vec() - Vector3::new(4.0, 0.2 + ground_level, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    world.add_hitable(Sphere::new(center, 0.2, Material::Lambertian { albedo: Vector3::new(seedable_unit_distribution() * seedable_unit_distribution(), seedable_unit_distribution() * seedable_unit_distribution(), seedable_unit_distribution() * seedable_unit_distribution()) }));
                } else if choose_mat < 0.95 {
                    world.add_hitable(Sphere::new(center, 0.2, Material::Metal { albedo: Vector3::new(0.5 * (1.0 + seedable_unit_distribution()), 0.5 * (1.0 + seedable_unit_distribution()), 0.5 * (1.0 + seedable_unit_distribution())), fuzz: 0.5 * seedable_unit_distribution() }));
                } else {
                    world.add_hitable(Sphere::new(center, 0.2, Material::Dielectric { ref_idx: 1.5 }));
                }
            }
        }
    }

    let from = Point3::new(14.0, 1.5, -22.0);
    let at = Vector3::new(0.0, 1.0, -0.5);
    let dist_to_focus = (from.to_vec() - at).magnitude();
    let aperture = 0.001;
    let camera = Camera::new(from, at, Vector3::unit_y(), 15.0, aspect, aperture, dist_to_focus, Instant::now(), Duration::from_millis(1000));

    (world, camera)
}

pub fn two_spheres(aspect: f32) -> (HitableList, Camera) {
    let texture = Material::TexturedLambertian {
        texture: ChequeredTexture::new(
            ConstantTexture::new(Vector3::new(0.2, 0.3, 0.1)),
            ConstantTexture::new(Vector3::new(0.9, 0.9, 0.9)),
        )
    };

    let world = HitableList::new()
        .with_hitable(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10f32, texture.clone()))
        .with_hitable(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10f32, texture.clone()));

    let from = Point3::new(13.0, 2.0, 3.0);
    let at = Vector3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(from, at, Vector3::unit_y(), 15.0, aspect, aperture, dist_to_focus, Instant::now(), Duration::from_millis(1000));

    (world, camera)
}

pub fn two_perlin_spheres(aspect: f32) -> (HitableList, Camera) {
    let material = Material::TexturedLambertian {
        texture: NoiseTexture::new(0.01),
    };

    let world = HitableList::new()
        .with_hitable(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000f32, material.clone()))
        .with_hitable(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2f32, material.clone()));

    let from = Point3::new(13.0, 2.0, 3.0);
    let at = Vector3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let camera = Camera::new(from, at, Vector3::unit_y(), 15.0, aspect, aperture, dist_to_focus, Instant::now(), Duration::from_millis(1000));

    (world, camera)
}

pub fn test_scene(aspect: f32) -> (HitableList, Camera) {
    let ground_level = -0.5;
    let pyramid_base = 250f32;
    let pyramid_height = 100f32;
    let shiny_teapot_scale = 7.0;
    let glass_teapot_scale = 1.2;
    let sphere_radius = 1.0;

    let mut world = HitableList::new()
        .with_hitable(Pyramid::new(
            Point3::new(-400.0, ground_level, -1200.0),
            pyramid_base,
            pyramid_height,
            Quaternion::from_angle_y(Deg(45f32)),
            Material::Lambertian { albedo: Vector3::new(0.4, 0.2, 0.1) }
        ))
        .with_hitable(Plane::new(Point3::new(0.0, ground_level, 0.0), Vector3::unit_y(), Material::Lambertian { albedo: Vector3::new(0.8, 0.5, 0.2) }))
        .with_hitable(Teapot::new(Point3::new(-25.0, ground_level + shiny_teapot_scale / 2.0, -40.0), shiny_teapot_scale, Quaternion::one(), Material::Metal { albedo: Vector3::new(0.7, 0.6, 0.5), fuzz: 0.0 }))
        .with_hitable(Teapot::new(Point3::new(8.5, ground_level + glass_teapot_scale / 2.0, 15.0), glass_teapot_scale, Quaternion::from_angle_y(Deg(240f32)), Material::Dielectric { ref_idx: 1.5 }))
        .with_hitable(Sphere::new(Point3::new(-10.0, sphere_radius + ground_level, -25.0), sphere_radius, Material::Dielectric { ref_idx: 1.5 }))
        .with_hitable(Sphere::new(Point3::new(-4.0, sphere_radius + ground_level, -20.0), sphere_radius, Material::Lambertian { albedo: Vector3::new(0.1, 0.2, 0.5) }))
        .with_hitable(Sphere::new(Point3::new(-21.0, sphere_radius + ground_level, -60.0), sphere_radius, Material::Metal { albedo: Vector3::new(0.7, 0.6, 0.5), fuzz: 0.0 }));

    let num = 11;

    for a_offset in 0..2 * num {
        let a = (a_offset - num) as f32;
        for b_offset in 0..2 * num {
            let b = (b_offset - num) as f32;
            let choose_mat = seedable_unit_distribution();
            let center = Point3::new(a + 0.9 * seedable_unit_distribution(), 0.2 + ground_level, b + 0.9 * seedable_unit_distribution());
            if (center.to_vec() - Vector3::new(4.0, 0.2 + ground_level, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    world.add_hitable(Sphere::new(center, 0.2, Material::Lambertian { albedo: Vector3::new(seedable_unit_distribution() * seedable_unit_distribution(), seedable_unit_distribution() * seedable_unit_distribution(), seedable_unit_distribution() * seedable_unit_distribution()) }));
                } else if choose_mat < 0.95 {
                    world.add_hitable(Sphere::new(center, 0.2, Material::Metal { albedo: Vector3::new(0.5 * (1.0 + seedable_unit_distribution()), 0.5 * (1.0 + seedable_unit_distribution()), 0.5 * (1.0 + seedable_unit_distribution())), fuzz: 0.5 * seedable_unit_distribution() }));
                } else {
                    world.add_hitable(Sphere::new(center, 0.2, Material::Dielectric { ref_idx: 1.5 }));
                }
            }
        }
    }

    let from = Point3::new(11.4, 1.0, 22.8);
    let at = Vector3::new(0.75, 0.0, 0.5);
    let dist_to_focus = (from.to_vec() - at).magnitude();
    let aperture = 0.001;
    let camera = Camera::new(from, at, Vector3::unit_y(), 15.0, aspect, aperture, dist_to_focus, Instant::now(), Duration::from_millis(1000));

    (world, camera)
}
