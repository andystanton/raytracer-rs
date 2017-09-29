use hitable::{
    Hitable,
    HitRecord,
    HitableList,
};
use ray::Ray3;
use aabb::AABB;
use material::Material;
use triangle::Triangle;

use cgmath::{
    Point3,
    Quaternion,
    Rotation,
};

use std::time::Instant;

pub struct Quad {
    hitable_list: HitableList,
}

impl Quad {
    pub fn new(vertices: [Point3<f32>; 4], rotation: Quaternion<f32>, material: Material) -> Self {
        Self {
            hitable_list: HitableList::new()
                .with_hitable(Triangle::new([rotation.rotate_point(vertices[0]), rotation.rotate_point(vertices[1]), rotation.rotate_point(vertices[2])], material.clone()))
                .with_hitable(Triangle::new([rotation.rotate_point(vertices[0]), rotation.rotate_point(vertices[2]), rotation.rotate_point(vertices[3])], material.clone()))
        }
    }
}

impl Hitable for Quad {
    fn hit(&self, r: &Ray3<f32>, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hitable_list.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t0: Instant, t1: Instant) -> Option<AABB> {
        self.hitable_list.bounding_box(t0, t1)
    }
}