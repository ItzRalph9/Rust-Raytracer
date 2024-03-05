use nalgebra::Vector3;

use crate::library::basic_lib::*;
use crate::library::hittable::{Hittable, HittableTrait};

#[derive(Debug, Clone)]
pub struct Translate {
    object: Hittable,
    offset: Vector3<f64>,
    bounding_box: Aabb,
}

impl Translate {
    pub fn new(object: Hittable, offset: Vector3<f64>) -> Self {
        let bounding_box = object.get_bounding_box() + offset;

        Translate { object, offset, bounding_box }
    }
}

impl HittableTrait for Translate {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject> {
        // Move the ray backwards by the offset
        let offset_r = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        // Determine where (if any) an intersection occurs along the offset ray
        if let Some(mut hit_object) = self.object.hit(offset_r, ray_t) {
           // Move the intersection point forwards by the offset
           hit_object.point += self.offset;
           
           return Some(hit_object);
        }
           
        None
    }

    fn get_bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
