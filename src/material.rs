use ray::Ray3;
use hitable::HitRecord;
use randomutil;
use texture::Texture;

use cgmath::{
    Vector3,
    InnerSpace,
    Zero,
};
use cgmath::num_traits::pow;

pub fn scatter(material: &Material, r_in: &Ray3<f32>, hit_rec: &HitRecord) -> (Vector3<f32>, Ray3<f32>, bool) {
    match *material {
        Material::Metal { albedo, fuzz } => {
            let reflected = reflect(r_in.direction.normalize(), hit_rec.normal);
            let scattered = Ray3::new(hit_rec.p, reflected + fuzz * randomutil::random_in_unit_sphere(), r_in.time);
            let should_scatter = scattered.direction.dot(hit_rec.normal) > 0.0;
            (albedo, scattered, should_scatter)
        }
        Material::Lambertian { albedo } => {
            let target = hit_rec.p + hit_rec.normal + randomutil::random_in_unit_sphere();
            (albedo, Ray3::new(hit_rec.p, target - hit_rec.p, r_in.time), true)
        }
        Material::TexturedLambertian { ref texture } => {
            let target = hit_rec.p + hit_rec.normal + randomutil::random_in_unit_sphere();
            (texture.value(0f32, 0f32, hit_rec.p), Ray3::new(hit_rec.p, target - hit_rec.p, r_in.time), true)
        }
        Material::Dielectric { ref_idx } => {
            let reflected = reflect(r_in.direction, hit_rec.normal);
            let attenuation = Vector3::new(1.0, 1.0, 1.0);
            let direction_dot_normal = r_in.direction.dot(hit_rec.normal);
            let (outward_normal, ni_over_nt, cosine) = if direction_dot_normal > 0.0 {
                (-hit_rec.normal, ref_idx, ref_idx * direction_dot_normal / r_in.direction.magnitude())
            } else {
                (hit_rec.normal, 1.0 / ref_idx, -direction_dot_normal / r_in.direction.magnitude())
            };

            let (refracted, should_refract) = refract(r_in.direction, outward_normal, ni_over_nt);

            let reflect_prob = if should_refract {
                schlick(cosine, ref_idx)
            } else {
                1.0
            };

            if randomutil::unit_distribution() < reflect_prob {
                (attenuation, Ray3::new(hit_rec.p, reflected, r_in.time), true)
            } else {
                (attenuation, Ray3::new(hit_rec.p, refracted, r_in.time), true)
            }
        }
    }
}

#[derive(Clone)]
pub enum Material {
    Metal {
        albedo: Vector3<f32>,
        fuzz: f32,
    },
    Lambertian {
        albedo: Vector3<f32>,
    },
    TexturedLambertian {
        texture: Box<Texture + Send + Sync>,
    },
    Dielectric {
        ref_idx: f32,
    },
}

fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(n) * n
}

fn refract(v: Vector3<f32>, n: Vector3<f32>, ni_over_nt: f32) -> (Vector3<f32>, bool) {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        (ni_over_nt * (uv - n * dt) - n * discriminant.sqrt(), true)
    } else {
        (Vector3::zero(), false)
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * pow((1.0 - cosine), 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_clones() {
        let option = Some(Material::Lambertian { albedo: Vector3::new(1f32, 2f32, 3f32) });
        match option {
            Some(Material::Lambertian { albedo }) => assert_eq!(albedo, Vector3::new(1f32, 2f32, 3f32)),
            _ => assert!(false),
        }
    }
}