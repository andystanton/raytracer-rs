use ray::Ray3;

use cgmath::Point3;

use std::mem::swap;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl AABB {
    pub fn new(min: Point3<f32>, max: Point3<f32>) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn surrounding_box(&self, other: &AABB) -> AABB {
        Self::new(
            Point3::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y), self.min.z.min(other.min.z)),
            Point3::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y), self.max.z.max(other.max.z)),
        )
    }

    pub fn hit(&self, r: &Ray3<f32>, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1f32 / r.direction[a];
            let mut t0 = (self.min[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin[a]) * inv_d;

            if inv_d < 0f32 {
                swap(&mut t0, &mut t1)
            }

            let t_min_2 = if t0 > t_min {
                t0
            } else {
                t_min
            };

            let t_max_2 = if t1 < t_max {
                t1
            } else {
                t_max
            };

            if t_max_2 <= t_min_2 {
                return false;
            }
        }
        true
    }
}