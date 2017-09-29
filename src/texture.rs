use cgmath::{
    Point3,
    Vector3,
};
use noise::{
    NoiseFn,
    Perlin,
    Turbulence,
};

use std::clone::Clone;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Point3<f32>) -> Vector3<f32>;
    fn box_clone(&self) -> Box<Texture + Send + Sync>;
}

impl Clone for Box<Texture + Send + Sync> {
    fn clone(&self) -> Box<Texture + Send + Sync> {
        self.box_clone()
    }
}

// Constant Texture

#[derive(Clone)]
pub struct ConstantTexture {
    pub colour: Vector3<f32>
}

impl ConstantTexture {
    pub fn new(colour: Vector3<f32>) -> Box<Self> {
        Box::new(Self { colour })
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _: f32, _: f32, _: Point3<f32>) -> Vector3<f32> {
        self.colour
    }
    fn box_clone(&self) -> Box<Texture + Send + Sync> {
        Box::new((*self).clone())
    }
}

// Chequered Texture

#[derive(Clone)]
pub struct ChequeredTexture {
    pub odd: Box<Texture + Send + Sync>,
    pub even: Box<Texture + Send + Sync>,
}

impl ChequeredTexture {
    pub fn new(odd: Box<Texture + Send + Sync>, even: Box<Texture + Send + Sync>) -> Box<Self> {
        Box::new(Self { odd, even })
    }
}

impl Texture for ChequeredTexture {
    fn value(&self, u: f32, v: f32, p: Point3<f32>) -> Vector3<f32> {
        let chequer_frequency = 10f32;
        let sines = if p.y == 0f32 {
            (chequer_frequency * p.x).sin() * (chequer_frequency * p.z).sin()
        } else {
            (chequer_frequency * p.x).sin() * (chequer_frequency * p.y).sin() * (chequer_frequency * p.z).sin()
        };

        if sines < 0f32 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
    fn box_clone(&self) -> Box<Texture + Send + Sync> {
        Box::new((*self).clone())
    }
}

// Perlin Noise Texture

#[derive(Clone)]
pub struct NoiseTexture {
    perlin: Turbulence<Perlin>,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Box<Self> {
        Box::new(Self {
            perlin: Turbulence::new(Perlin::new()),
            scale,
        })
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f32, _: f32, p: Point3<f32>) -> Vector3<f32> {
        Vector3::new(1f32, 1f32, 1f32) * 0.5 * (1f32 + (self.scale * p.z + 10f32 * self.perlin.get([p.x as f64, p.y as f64, p.z as f64]) as f32).sin())
    }

    fn box_clone(&self) -> Box<Texture + Send + Sync> {
        Box::new((*self).clone())
    }
}