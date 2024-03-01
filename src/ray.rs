use nalgebra::Vector3;
use rand::prelude::*;

use crate::{sphere::Sphere, hit_object::HitObject, vector3::Vector3Extensions};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Ray { origin, direction }
    }
    
    pub fn calculate_hit_position(&self, distance: f64) -> Vector3<f64> {
        self.origin + distance * self.direction
    }

    pub fn cast_ray(ray: Ray, spheres: &[Sphere]) -> Option<HitObject> {
        let mut hit_object: Option<HitObject> = None;
    
        let offset = 0.001; // This is used so that the new ray won't hit the same sphere again
    
        for sphere in spheres {
            let intersection_distance = Sphere::intersect_ray_sphere(ray, sphere.center, sphere.radius);
            
            if let Some(distance) = intersection_distance {
                if hit_object.clone().map_or(true, |hit| distance < hit.t) {
                    if distance < offset {
                        continue;
                    }
    
                    let hit_position = ray.calculate_hit_position(distance);
                    let outward_normal = sphere.calculate_normal(hit_position);
                    let material = sphere.material;
                    
                    hit_object = Some(HitObject::new(hit_position, ray, outward_normal, material, distance));
                }
            }
        }
    
        hit_object
    }

    pub fn random_unit_vector() -> Vector3<f64> {
        Self::random_in_unit_sphere().normalize()
    }

    // pub fn random_on_hemisphere(normal: Vector3<f64>) -> Vector3<f64> {
    //     let on_unit_sphere = Self::random_unit_vector();

    //     let same_hemisphere_as_normal = on_unit_sphere.dot(&normal) > 0.0;
    //     if same_hemisphere_as_normal {
    //         on_unit_sphere
    //     } else {
    //         -on_unit_sphere
    //     }
    // }

    pub fn random_in_unit_sphere() -> Vector3<f64> {
        let mut rng = rand::thread_rng();

        loop {
            let x = rng.gen_range(-1.0..1.0);
            let y = rng.gen_range(-1.0..1.0);
            let z = rng.gen_range(-1.0..1.0);
        
            let p = Vector3::new(x, y, z);
            if p.length_squared() < 1.0 {
                break p;
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
