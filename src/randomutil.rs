use rand::{Rng, SeedableRng, rng};
use rand::rngs::StdRng;
use cgmath::{Vector3, InnerSpace};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct ThreadSeedableRng {
    rng: Rc<RefCell<StdRng>>,
}

impl ThreadSeedableRng {
    fn random_range<T, R>(&mut self, range: R) -> T
    where
        T: rand::distr::uniform::SampleUniform,
        R: rand::distr::uniform::SampleRange<T>,
    {
        self.rng.borrow_mut().random_range(range)
    }
}

thread_local! {
    static SEEDABLE_RNG: RefCell<ThreadSeedableRng> = RefCell::new(ThreadSeedableRng {
        rng: Rc::new(RefCell::new(StdRng::from_rng(&mut rng())))
    });
}

pub fn thread_seedable_rng() -> ThreadSeedableRng {
    SEEDABLE_RNG.with(|rng| rng.borrow().clone())
}

pub fn thread_seedable_reseed(seed: u64) {
    SEEDABLE_RNG.with(|rng| {
        *rng.borrow_mut() = ThreadSeedableRng {
            rng: Rc::new(RefCell::new(StdRng::seed_from_u64(seed))),
        };
    });
}

pub fn seedable_unit_distribution() -> f32 {
    thread_seedable_rng().random_range(0.0..1.0)
}

pub fn unit_distribution() -> f32 {
    rng().random_range(0.0..1.0)
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
