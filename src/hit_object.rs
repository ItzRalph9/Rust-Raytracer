use nalgebra::Vector3;
use crate::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct HitObject {
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: Material,
    pub t: f64,
    pub _front_face: bool,
}

impl HitObject {
    pub fn new(point: Vector3<f64>, normal: Vector3<f64>, material: Material, t: f64, _front_face: bool) -> Self {
        HitObject { point, normal, material, t, _front_face }
    }
}