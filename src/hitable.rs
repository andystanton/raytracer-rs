use ray::Ray3;
use aabb::AABB;
use material::Material;

use cgmath::{
    Vector3,
    Point3,
};

use std::time::Instant;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Point3<f32>,
    pub normal: Vector3<f32>,
    pub material: Option<Material>,
}

impl HitRecord {
    pub fn new(t: f32, p: Point3<f32>, normal: Vector3<f32>, material: Option<Material>) -> Self {
        HitRecord { t, p, normal, material }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray3<f32>, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: Instant, t1: Instant) -> Option<AABB>;
}

pub struct HitableList {
    list: Vec<Box<Hitable + Sync + Send>>,
}

impl HitableList {
    pub fn new() -> Self {
        HitableList { list: Vec::new() }
    }

    pub fn with_hitable<H>(mut self, hitable: H) -> Self where H: Hitable + Sync + Send + 'static {
        self.list.push(Box::new(hitable));
        self
    }

    pub fn add_hitable<H>(&mut self, hitable: H) where H: Hitable + Sync + Send + 'static {
        self.list.push(Box::new(hitable))
    }
}

impl Hitable for HitableList {
    // base performance: original
    fn hit(&self, r: &Ray3<f32>, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec = None;

        for hitable in &self.list {
            if let Some(temp_record) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_record.t;
                rec = Some(temp_record);
            }
        }
        rec
    }

    // worse performance: using fold but mutable closest_so_far
    //    fn hit(&self, r: &Ray3<f32>, t_min: f32, t_max: f32) -> Option<HitRecord> {
    //        let mut closest_so_far = t_max;
    //        self.list.iter().fold(None, |acc, next| {
    //            if let Some(temp_record) = next.hit(r, t_min, closest_so_far) {
    //                closest_so_far = temp_record.t;
    //                Some(temp_record)
    //            } else {
    //                acc
    //            }
    //        })
    //    }

    // even worse performance: using fold and option operations
    //    fn hit(&self, r: &Ray3<f32>, t_min: f32, t_max: f32) -> Option<HitRecord> {
    //        self.list.iter().fold(None, |acc, next| {
    //            acc.and_then(
    //                |prev| next.hit(r, t_min, prev.t).or_else(
    //                    || Some(prev)
    //                )
    //            ).or_else(
    //                || next.hit(r, t_min, t_max)
    //            )
    //        })
    //    }

    fn bounding_box(&self, t0: Instant, t1: Instant) -> Option<AABB> {
        if self.list.is_empty() {
            None
        } else {
            let temp_box = self.list[0].bounding_box(t0, t1);
            if temp_box.is_none() {
                None
            } else {
                self.list[1..].iter().fold(temp_box, |acc, next| {
                    acc.and_then(|b| next.bounding_box(t0, t1).map(|n| b.surrounding_box(&n)))
                })
            }
        }
    }
}