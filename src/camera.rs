use ray::Ray3;
use randomutil::{
    seedable_unit_distribution,
    random_in_unit_disk,
};

use cgmath::{
    Point3,
    Vector3,
    EuclideanSpace,
    InnerSpace,
};
use std::f32;
use std::time::{
    Duration,
    Instant,
};
use timeutil::TimeUtil;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Point3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    lens_radius: f32,
    u: Vector3<f32>,
    v: Vector3<f32>,
    aperture_open_time: Instant,
    aperture_duration: Duration,
}

impl Camera {
    pub fn new(look_from: Point3<f32>,
               look_at: Vector3<f32>,
               up: Vector3<f32>,
               vfov: f32,
               aspect: f32,
               aperture: f32,
               focus_dist: f32,
               aperture_open_time: Instant,
               aperture_duration: Duration) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from.to_vec() - look_at).normalize();
        let u = up.cross(w);
        let v = w.cross(u);

        Camera {
            lower_left_corner: look_from.to_vec() - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: look_from,
            lens_radius,
            u,
            v,
            aperture_open_time,
            aperture_duration,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray3<f32> {
        let rd: Vector3<f32> = self.lens_radius * random_in_unit_disk();
        let offset: Vector3<f32> = self.u * rd.x + self.v * rd.y;
        let time = self.aperture_open_time + self.aperture_duration.mul_decimal(seedable_unit_distribution());
        Ray3::new(self.origin + offset,
                  self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin.to_vec() - offset,
                  time)
    }
}