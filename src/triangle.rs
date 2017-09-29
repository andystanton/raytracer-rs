use hitable::{
    Hitable,
    HitRecord,
};
use ray::Ray3;
use aabb::AABB;
use material::Material;

use cgmath::{
    Point3,
    Vector3,
    InnerSpace,
};

use std::time::Instant;

pub struct Triangle {
    vertices: [Point3<f32>; 3],
    surface_normal: Vector3<f32>,
    material: Material,
}

impl Triangle {
    pub fn new(vertices: [Point3<f32>; 3], material: Material) -> Self {
        let e1 = vertices[1] - vertices[0];
        let e2 = vertices[2] - vertices[0];
        let surface_normal = e2.cross(e1).normalize();
        Self { vertices, surface_normal, material }
    }
}

impl Hitable for Triangle {
    fn hit(&self, r: &Ray3<f32>, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let e1 = self.vertices[1] - self.vertices[0];
        let e2 = self.vertices[2] - self.vertices[0];
        let dir = r.direction;

        let pvec = dir.cross(e2);
        let det = e1.dot(pvec);

        if det > -0.00001 && det < 0.00001 {
            return None
        }

        let inv_det = 1.0 / det;
        let tvec = r.origin - self.vertices[0];
        let u = tvec.dot(pvec) * inv_det;

        if u < 0.0 || u > 1.0 {
            return None
        }

        let qvec = tvec.cross(e1);
        let v = dir.dot(qvec) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None
        }

        let t = e2.dot(qvec) * inv_det;

        if t > 0.00001 && t < t_max && t > t_min {
            return Some(HitRecord::new(
                t,
                r.point_at_parameter(t),
                self.surface_normal,
                Some(self.material.clone()),
            ));
        }

        None
    }

    fn bounding_box(&self, _: Instant, _: Instant) -> Option<AABB> {
        let mut min = Point3::new(self.vertices[0].x.min(self.vertices[1].x), self.vertices[0].y.min(self.vertices[1].y), self.vertices[0].z.min(self.vertices[1].z));
        min = Point3::new(min.x.min(self.vertices[2].x), min.y.min(self.vertices[2].y), min.z.min(self.vertices[2].z));

        let mut max = Point3::new(self.vertices[0].x.max(self.vertices[1].x), self.vertices[0].y.max(self.vertices[1].y), self.vertices[0].z.max(self.vertices[1].z));
        max = Point3::new(max.x.max(self.vertices[2].x), max.y.max(self.vertices[2].y), max.z.max(self.vertices[2].z));

        Some(AABB::new(min, max))
    }
}

pub struct NormalTriangle {
    vertices: [Point3<f32>; 3],
    normals: [Vector3<f32>; 3],
    material: Material,
}

impl NormalTriangle {
    pub fn new(vertices: [Point3<f32>; 3], normals: [Vector3<f32>; 3], material: Material) -> Self {
        Self { vertices, normals, material }
    }
}

impl Hitable for NormalTriangle {
    fn hit(&self, r: &Ray3<f32>, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let e1 = self.vertices[1] - self.vertices[0];
        let e2 = self.vertices[2] - self.vertices[0];
        let dir = r.direction;

        let pvec = dir.cross(e2);
        let det = e1.dot(pvec);

        if det > -0.00001 && det < 0.00001 {
            return None
        }

        let inv_det = 1.0 / det;
        let tvec = r.origin - self.vertices[0];
        let u = tvec.dot(pvec) * inv_det;

        if u < 0.0 || u > 1.0 {
            return None
        }

        let qvec = tvec.cross(e1);
        let v = dir.dot(qvec) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return None
        }

        let t = e2.dot(qvec) * inv_det;

        let p = r.point_at_parameter(t);
        if t > 0.00001 && t < t_max && t > t_min {
            let normal = u * self.normals[1] + v * self.normals[2] + (1f32 - u - v) * self.normals[0];
            Some(HitRecord::new(
                t,
                p,
                normal,
                Some(self.material.clone()),
            ))
        } else {
            None
        }
    }

    fn bounding_box(&self, _: Instant, _: Instant) -> Option<AABB> {
        let mut min = Point3::new(self.vertices[0].x.min(self.vertices[1].x), self.vertices[0].y.min(self.vertices[1].y), self.vertices[0].z.min(self.vertices[1].z));
        min = Point3::new(min.x.min(self.vertices[2].x), min.y.min(self.vertices[2].y), min.z.min(self.vertices[2].z));

        let mut max = Point3::new(self.vertices[0].x.max(self.vertices[1].x), self.vertices[0].y.max(self.vertices[1].y), self.vertices[0].z.max(self.vertices[1].z));
        max = Point3::new(max.x.max(self.vertices[2].x), max.y.max(self.vertices[2].y), max.z.max(self.vertices[2].z));

        Some(AABB::new(min, max))
    }
}