use nalgebra::Vector3;

use crate::basic_lib::*;
use crate::{hittable::HittableTrait, hittable_list::HittableList, hittable::Hittable, material::Material};

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

    // Returns the 3D box (six sides) that contains the two opposite vertices a & b.
    pub fn create_box(a: Vector3<f64>, b: Vector3<f64>, material: Material) -> HittableList {
        let mut sides = HittableList::new();
    
        // Construct the two opposite vertices with the minimum and maximum coordinates.
        let min = Vector3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Vector3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));
    
        let dx = Vector3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vector3::new(0.0, max.y - min.y, 0.0);
        let dz = Vector3::new(0.0, 0.0, max.z - min.z);
    
        sides.add(Hittable::Quad(Quad::new(Vector3::new(min.x, min.y, max.z),  dx,  dy, material.clone()))); // front
        sides.add(Hittable::Quad(Quad::new(Vector3::new(max.x, min.y, max.z), -dz,  dy, material.clone()))); // right
        sides.add(Hittable::Quad(Quad::new(Vector3::new(max.x, min.y, min.z), -dx,  dy, material.clone()))); // back
        sides.add(Hittable::Quad(Quad::new(Vector3::new(min.x, min.y, min.z),  dz,  dy, material.clone()))); // left
        sides.add(Hittable::Quad(Quad::new(Vector3::new(min.x, max.y, max.z),  dx, -dz, material.clone()))); // top
        sides.add(Hittable::Quad(Quad::new(Vector3::new(min.x, min.y, min.z),  dx,  dz, material.clone()))); // bottom
    
        sides
    }
}

impl HittableTrait for Quad {
    fn hit(&self, ray: crate::ray::Ray, ray_t: Interval) -> Option<HitObject> {
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
