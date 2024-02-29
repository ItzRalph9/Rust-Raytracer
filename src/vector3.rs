use nalgebra::Vector3;

pub trait Vector3Extensions {
    fn near_zero(&self) -> bool;
    fn length_squared(&self) -> f64;
}

impl Vector3Extensions for Vector3<f64> {
    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}