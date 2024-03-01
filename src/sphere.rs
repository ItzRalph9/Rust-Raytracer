use nalgebra::Vector3;

use crate::material::Material;
use crate::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Material
}

impl Sphere {
    pub fn calculate_normal(&self, hit_position: Vector3<f64>) -> Vector3<f64>{
        (hit_position - self.center) / self.radius
    }

    pub fn intersect_ray_sphere(ray: Ray, center: Vector3<f64>, radius: f64) -> Option<f64> {
        let oc = ray.origin - center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - radius * radius;
        
        let discriminant = b * b - 4.0 * a * c;
        if discriminant <= 0.0 {
            return None;
        }
    
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t < 0.0 {
          return None;  
        } 
    
        Some(t)
    }
}
