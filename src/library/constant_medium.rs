use nalgebra::Vector3;

use crate::library::basic_lib::*;
use crate::library::{hittable::{Hittable, HittableTrait}, material::Material, texture::Texture, color::Color};

#[derive(Debug, Clone)]
pub struct ConstantMedium {
    boundary: Box<Hittable>,
    neg_inv_density: f64,
    phase_function: Material,
}

impl ConstantMedium {
    pub fn new(boundary: Hittable, density: f64, phase_function: Texture) -> Self {
        ConstantMedium {
            boundary: Box::new(boundary),
            neg_inv_density: -1.0 / density,
            phase_function: Material::IsoTropic(phase_function),
        }
    }

    pub fn new_from_color(boundary: Hittable, density: f64, color: Color) -> Self {
        Self::new(boundary, density, Texture::SolidColor(color))
    }
}

impl HittableTrait for ConstantMedium {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject> {
        if let Some(mut hit1) = self.boundary.hit(ray, Interval::universe()) {
            if let Some(mut hit2) = self.boundary.hit(ray, Interval::new(hit1.t + 0.0001, f64::INFINITY)) {
                if hit1.t < ray_t.min { hit1.t = ray_t.min; }
                if hit2.t > ray_t.max { hit2.t = ray_t.max; }

                if hit1.t >= hit2.t {
                    return None;
                }

                if hit1.t < 0.0 {
                    hit1.t = 0.0;
                }

                let ray_length = ray.direction.magnitude();
                let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
                let hit_distance = self.neg_inv_density * Material::random_float().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = hit1.t + hit_distance / ray_length;
                let point = ray.calculate_hit_position(t);

                let outward_normal = Vector3::new(1.0, 0.0, 0.0);  // arbitrary
                let material = &self.phase_function;

                return Some(HitObject::new(point, ray, outward_normal, material.clone(), t, 0.0, 0.0))
            }
        }

        None
    }

    fn get_bounding_box(&self) -> Aabb {
        self.boundary.get_bounding_box()
    }
}
