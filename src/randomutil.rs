use rand::{
    Rng,
    SeedableRng,
    StdRng,
    thread_rng,
};
use rand::distributions::range::SampleRange;
use cgmath::{
    Vector3,
    InnerSpace,
};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct ThreadSeedableRng {
    rng: Rc<RefCell<StdRng>>
}

impl ThreadSeedableRng {
    fn gen_range<T: PartialOrd + SampleRange>(&mut self, low: T, high: T) -> T where Self: Sized {
        self.rng.borrow_mut().gen_range(low, high)
    }
}

thread_local! {
    static SEEDABLE_RNG: ThreadSeedableRng = ThreadSeedableRng {rng: Rc::new(RefCell::new(StdRng::new().unwrap()))};
}

pub fn thread_seedable_rng() -> ThreadSeedableRng {
    SEEDABLE_RNG.with(|rng| rng.clone())
}

pub fn thread_seedable_reseed(seed: usize) {
    SEEDABLE_RNG.with(|rng| {
        let seed_values: &[_] = &[seed];
        rng.rng.borrow_mut().reseed(seed_values);
    });
}

pub fn seedable_unit_distribution() -> f32 {
    thread_seedable_rng().gen_range(0.0, 1.0)
}

pub fn unit_distribution() -> f32 {
    thread_rng().gen_range(0.0, 1.0)
}

pub fn random_in_unit_sphere() -> Vector3<f32> {
    let mut p: Vector3<f32>;
    loop {
        p = 2.0 * Vector3::new(unit_distribution(), unit_distribution(), unit_distribution()) - Vector3::new(1.0, 1.0, 1.0);
        if p.magnitude2() >= 1.0 { break }
    }
    p
}

pub fn random_in_unit_disk() -> Vector3<f32> {
    let mut p: Vector3<f32>;
    loop {
        p = 2.0 * Vector3::new(unit_distribution(), unit_distribution(), 0.0) - Vector3::new(1.0, 1.0, 0.0);
        if p.dot(p) >= 1.0 { break };
    }
    p
}
