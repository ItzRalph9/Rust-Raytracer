use nalgebra::Vector3;
use rand::prelude::*;

use crate::vector3::Vector3Extensions;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>, time: f64) -> Self {
        Ray { origin, direction, time }
    }
    
    pub fn calculate_hit_position(&self, distance: f64) -> Vector3<f64> {
        self.origin + distance * self.direction
    }

    pub fn random_unit_vector() -> Vector3<f64> {
        Self::random_in_unit_sphere().normalize()
    }

    pub fn random_in_unit_sphere() -> Vector3<f64> {
        loop {
            let p = Vector3::random_float_range(-1.0..1.0);
            if p.length_squared() < 1.0 {
                break p;
            }
        }
    }

    pub fn random_in_unit_disk() -> Vector3<f64> {
        let mut rng = rand::thread_rng();

        loop {
            let p = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
        v - 2.0 * v.dot(&n) * n
    }

    pub fn refract(uv: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
        let cos_theta = -uv.dot(&n).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;
        
        r_out_perp + r_out_parallel
    }
}
