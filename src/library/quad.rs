use nalgebra::Vector3;

use crate::library::basic_lib::*;
use crate::library::{hittable::HittableTrait, material::Material};

#[derive(Debug, Clone)]
pub struct Quad {
    q: Vector3<f64>, // Lower-left corner
    u: Vector3<f64>, // first side
    v: Vector3<f64>, // second side
    material: Material,
    bounding_box: Aabb,
    normal: Vector3<f64>,
    d: f64,          // a constant
    w: Vector3<f64>,
}

impl Quad {
    pub fn new(q: Vector3<f64>, u: Vector3<f64>, v: Vector3<f64>, material: Material) -> Self {
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = normal.dot(&q);
        let w = n / n.dot(&n);

        Quad {
            q, u, v,
            material,
            bounding_box: Aabb::new_from_point(q, q + u + v).pad(),
            normal, d, w,
        }
    }

    fn is_interior(a: f64, b: f64) -> Option<(f64, f64)> {
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        if (a < 0.0) || (1.0 < a) || (b < 0.0) || (1.0 < b) {
            return None;
        }

        Some((a, b))
    }
}

impl HittableTrait for Quad {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject> {
        let denomenator = self.normal.dot(&ray.direction);

        // No hit if the ray is parallel to the plane.
        if denomenator.abs() < 1e-8 {
            return None;
        }

        // Return none if the hit point parameter t is outside the ray interval.
        let t = (self.d - self.normal.dot(&ray.origin)) / denomenator;
        if !ray_t.contains(t) {
            return None;
        }

        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.calculate_hit_position(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        if let Some((u, v)) = Self::is_interior(alpha, beta) {
            // Ray hits the 2D shape; set the rest of the hit record and return true.
            let hit_point = ray.calculate_hit_position(t);
            let hit_object = HitObject::new(hit_point, ray, self.normal, self.material.clone(), t, u, v);

            Some(hit_object)
        } else {
            None
        }
    }

    fn get_bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
