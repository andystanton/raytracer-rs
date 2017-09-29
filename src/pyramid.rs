use hitable::{
    Hitable,
    HitRecord,
    HitableList,
};
use ray::Ray3;
use aabb::AABB;
use material::Material;
use triangle::Triangle;
use quad::Quad;

use cgmath::{
    Point3,
    Vector3,
    Quaternion,
    Rotation,
    One,
};

use std::time::Instant;

pub struct Pyramid {
    hitable_list: HitableList,
}

impl Pyramid {
    pub fn new(position: Point3<f32>, base_length: f32, height: f32, rotation: Quaternion<f32>, material: Material) -> Self {
        let base_vertices = [
            position + rotation.rotate_vector(Vector3::new(-0.5, 0.0, 0.5) * base_length),
            position + rotation.rotate_vector(Vector3::new(-0.5, 0.0, -0.5) * base_length),
            position + rotation.rotate_vector(Vector3::new(0.5, 0.0, -0.5) * base_length),
            position + rotation.rotate_vector(Vector3::new(0.5, 0.0, 0.5) * base_length),
        ];
        let zenith = position + Vector3::new(0f32, height, 0f32);

        Self {
            hitable_list: HitableList::new()
                .with_hitable(Triangle::new([base_vertices[0], zenith, base_vertices[3]], material.clone()))
                .with_hitable(Triangle::new([base_vertices[3], zenith, base_vertices[2]], material.clone()))
                .with_hitable(Triangle::new([base_vertices[2], zenith, base_vertices[1]], material.clone()))
                .with_hitable(Triangle::new([base_vertices[1], zenith, base_vertices[0]], material.clone()))
                .with_hitable(Quad::new([base_vertices[0], base_vertices[3], base_vertices[2], base_vertices[1], ], Quaternion::one(), material.clone()))
        }
    }
}

impl Hitable for Pyramid {
    fn hit(&self, r: &Ray3<f32>, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hitable_list.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t0: Instant, t1: Instant) -> Option<AABB> {
        self.hitable_list.bounding_box(t0, t1)
    }
}