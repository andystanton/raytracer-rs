use hitable::{
    Hitable,
    HitRecord,
};
use ray::Ray3;
use material::Material;
use timeutil::TimeUtil;
use aabb::AABB;

use cgmath::{
    Point3,
    Vector3,
    InnerSpace,
};

use std::time::{
    Duration,
    Instant,
};

pub struct Sphere {
    center: Point3<f32>,
    radius: f32,
    material: Material,
}

pub struct MovingSphere {
    center1: Point3<f32>,
    center0: Point3<f32>,
    movement_start: Instant,
    movement_duration: Duration,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3<f32>, radius: f32, material: Material) -> Self {
        Sphere { center, radius, material }
    }
}

impl MovingSphere {
    pub fn new(center0: Point3<f32>,
               center1: Point3<f32>,
               movement_start: Instant,
               movement_duration: Duration,
               radius: f32,
               material: Material) -> Self {
        MovingSphere {
            center0,
            center1,
            movement_start,
            movement_duration,
            radius,
            material
        }
    }

    pub fn center_at(&self, time: Instant) -> Point3<f32> {
        let since_start = (time - self.movement_start).as_millis() as f32;
        let movement_millis = self.movement_duration.as_millis() as f32;
        self.center0 + (since_start / movement_millis) * (self.center1 - self.center0)
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray3<f32>, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0f32 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord::new(
                    temp,
                    p,
                    (p - self.center) / self.radius,
                    Some(self.material.clone())
                ));
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord::new(
                    temp,
                    p,
                    (p - self.center) / self.radius,
                    Some(self.material.clone())
                ));
            }
        }
        None
    }

    fn bounding_box(&self, _: Instant, _: Instant) -> Option<AABB> {
        let radius_cubed = Vector3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(self.center + -radius_cubed, self.center + radius_cubed))
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray3<f32>, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center_at(r.time);
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0f32 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord::new(
                    temp,
                    p,
                    (p - self.center_at(r.time)) / self.radius,
                    Some(self.material.clone())
                ));
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord::new(
                    temp,
                    p,
                    (p - self.center_at(r.time)) / self.radius,
                    Some(self.material.clone())
                ));
            }
        }
        None
    }

    fn bounding_box(&self, _: Instant, _: Instant) -> Option<AABB> {
        let radius_cubed = Vector3::new(self.radius, self.radius, self.radius);
        let box0 = AABB::new(self.center0 + -radius_cubed, self.center0 + radius_cubed);
        let box1 = AABB::new(self.center1 + -radius_cubed, self.center1 + radius_cubed);
        Some(box0.surrounding_box(&box1))
    }
}
