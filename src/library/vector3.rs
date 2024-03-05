use nalgebra::Vector3;
use rand::prelude::*;

pub trait Vector3Extensions {
    fn near_zero(&self) -> bool;
    fn length_squared(&self) -> f64;
    fn random_float_range(range: std::ops::Range<f64>) -> Vector3<f64>;
}

impl Vector3Extensions for Vector3<f64> {
    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    fn random_float_range(range: std::ops::Range<f64>) -> Vector3<f64> {
        let mut rng = rand::thread_rng();

        let x: f64 = rng.gen_range(range.clone());
        let y: f64 = rng.gen_range(range.clone());
        let z: f64 = rng.gen_range(range.clone());
        Vector3::new(x, y, z)
    }
}