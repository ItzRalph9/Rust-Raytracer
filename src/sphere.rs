use nalgebra::Vector3;

use crate::{hit_object::HitObject, material::Material, ray::Ray, interval::Interval};
use crate::vector3::Vector3Extensions;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center1: Vector3<f64>,
    pub radius: f64,
    pub material: Material,
    pub is_moving: bool,
    pub center_vec: Vector3<f64>,
}

impl Sphere {
    pub fn new_stationary(center: Vector3<f64>, radius: f64, material: Material) -> Self {
        Sphere {
            center1: center,
            radius,
            material,
            is_moving: false,
            center_vec: center,
        }
    }
    
    pub fn new_moving(center1: Vector3<f64>, center2: Vector3<f64>, radius: f64, material: Material) -> Self {
        Sphere {
            center1,
            radius,
            material,
            is_moving: true,
            center_vec: center2 - center1,
        }
    }
    
    pub fn calculate_normal(&self, hit_position: Vector3<f64>) -> Vector3<f64>{
        (hit_position - self.center1) / self.radius
    }

    pub fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject> {
        let mut center = self.center1;
        if self.is_moving {
            center = self.get_sphere_center(ray.time);
        }

        let oc = ray.origin - center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
    
        let sqrt = discriminant.sqrt();
        let mut root = (-half_b - sqrt) / a;
        if root <= ray_t.min || ray_t.max <= root {
            root = (-half_b + sqrt) / a;
            if root <= ray_t.min || ray_t.max <= root {
                return None;
            }
        }

        let t = root;
        let hit_point = ray.calculate_hit_position(t);
    
        Some(HitObject::new(hit_point, ray, self.calculate_normal(hit_point), self.material, t))
    }

    pub fn get_sphere_center(&self, time: f64) -> Vector3<f64> {
        self.center1 + time * self.center_vec
    }
}
