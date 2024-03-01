use nalgebra::Vector3;
use crate::{material::Material, ray::Ray};

#[derive(Debug, Clone, Copy)]
pub struct HitObject {
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitObject {
    pub fn new(point: Vector3<f64>, ray: Ray, outward_normal: Vector3<f64>, material: Material, t: f64) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;

        let mut normal = -outward_normal;
        if front_face {
            normal = outward_normal;
        }

        HitObject { point, normal, material, t, front_face }
    }
}