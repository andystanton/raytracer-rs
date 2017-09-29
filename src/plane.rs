use hitable::{
    Hitable,
    HitRecord,
};
use ray::Ray3;
use aabb::AABB;
use material::Material;

use cgmath::{
    Vector3,
    Point3,
    InnerSpace,
};

use std::time::Instant;

pub struct Plane {
    center: Point3<f32>,
    surface_normal: Vector3<f32>,
    material: Material,
}

impl Plane {
    pub fn new(center: Point3<f32>, surface_normal: Vector3<f32>, material: Material) -> Self {
        Self {
            center,
            surface_normal,
            material,
        }
    }
}

impl Hitable for Plane {
    fn hit(&self, r: &Ray3<f32>, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let dot = self.surface_normal.dot(r.direction);
        if dot < 0.00001 {
            let p0r0 = self.center - r.origin;
            let t = p0r0.dot(self.surface_normal) / dot;
            if t > 0.00001 && t < t_max && t > t_min {
                return Some(HitRecord::new(
                    t,
                    r.point_at_parameter(t),
                    self.surface_normal,
                    Some(self.material.clone())
                ));
            }
        }
        None
    }

    fn bounding_box(&self, _: Instant, _: Instant) -> Option<AABB> {
        unimplemented!()
    }
}