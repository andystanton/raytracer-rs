use cgmath::{
    Point3,
    Vector3,
    BaseNum,
    EuclideanSpace,
};
use std::time::Instant;

pub struct Ray3<T> where T: BaseNum {
    pub origin: Point3<T>,
    pub direction: Vector3<T>,
    pub time: Instant,
}

impl<T> Ray3<T> where T: BaseNum {
    pub fn new(origin: Point3<T>, direction: Vector3<T>, time: Instant) -> Self {
        Ray3 { origin, direction, time }
    }

    pub fn point_at_parameter(&self, t: T) -> Point3<T> {
        Point3::from_vec(self.origin.to_vec() + self.direction * t)
    }
}