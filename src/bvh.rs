use hitable::{
    Hitable,
    HitRecord,
};
use ray::Ray3;
use aabb::AABB;
use randomutil::seedable_unit_distribution;

use cgmath::Point3;

use std::time::Instant;
use std::sync::Arc;
use std::cmp::Ordering;

pub struct BVHNode {
    left: Arc<Hitable + Send + Sync>,
    right: Arc<Hitable + Send + Sync>,
    bounding_box: AABB,
}

type ArcHitable = Arc<Hitable + Send + Sync>;

fn box_axis_compare(axis: usize) -> Box<Fn(&ArcHitable, &ArcHitable) -> Ordering> {
    return Box::new(move |a: &ArcHitable, b: &ArcHitable| {
        a.bounding_box(Instant::now(), Instant::now()).and_then(|box_left| {
            b.bounding_box(Instant::now(), Instant::now()).map(|box_right| {
                if box_left.min[axis] - box_right.min[axis] < 0f32 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
        }).unwrap_or_else(|| {
            panic!("no bounding box in bvh node constructor");
        })
    });
}

impl BVHNode {
    pub fn new(mut l: Vec<ArcHitable>, t0: Instant, t1: Instant) -> Self {
        let axis = (3f32 * seedable_unit_distribution()) as usize;
        if axis > 2 {
            panic!("WTF")
        }
        l.sort_by(&*box_axis_compare(axis));

        let (left, right): (ArcHitable, ArcHitable) = if l.len() == 1 {
            (l[0].clone(), l[0].clone())
        } else if l.len() == 2 {
            (l[0].clone(), l[1].clone())
        } else {
            // TODO: this is possibly bullshit
            (
                Arc::new(BVHNode::new(l[0..l.len() / 2].to_vec(), t0, t1)),
                Arc::new(BVHNode::new(l[l.len() / 2 + 1..].to_vec(), t0, t1))
            )
        };

        Self {
            left: left,
            right: right,
            bounding_box: AABB::new(Point3::new(0f32, 0f32, 0f32), Point3::new(1f32, 1f32, 1f32))
        }
    }
}

impl Hitable for BVHNode {
    fn hit(&self, r: &Ray3<f32>, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bounding_box.hit(r, t_min, t_max) {
            let hit_left = self.left.hit(r, t_min, t_max);
            let hit_right = self.right.hit(r, t_min, t_max);

            if hit_left.is_some() && hit_right.is_some() {
                let left_rec = hit_left.unwrap();
                let right_rec = hit_right.unwrap();
                if left_rec.t < right_rec.t {
                    Some(left_rec)
                } else {
                    Some(right_rec)
                }
            } else if hit_left.is_some() {
                hit_left
            } else if hit_right.is_some() {
                hit_right
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _: Instant, _: Instant) -> Option<AABB> {
        unimplemented!()
    }
}