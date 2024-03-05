use nalgebra::Vector3;

use std::f64::INFINITY;
use std::f64::NEG_INFINITY;

use crate::basic_lib::*;
use crate::hittable_list::HittableList;
use crate::{hittable::{Hittable, HittableTrait}, camera::Camera};

#[derive(Debug, Clone)]
pub struct RotateY {
    object: Hittable,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Aabb,
}

impl RotateY {
    pub fn new(object: Hittable, angle: f64) -> Self {
        let radians = Camera::degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bounding_box = object.get_bounding_box();

        let mut min = Vector3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vector3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bounding_box.x.max + (1.0 - i as f64) * bounding_box.x.min;
                    let y = j as f64 * bounding_box.y.max + (1.0 - j as f64) * bounding_box.y.min;
                    let z = k as f64 * bounding_box.z.max + (1.0 - k as f64) * bounding_box.z.min;

                    let newx =  cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vector3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bounding_box = Aabb::new_from_point(min, max);

        RotateY { object, sin_theta, cos_theta, bounding_box }
    }
}

impl HittableTrait for RotateY {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject> {
        // Change the ray from world space to object space
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin.x = self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z;
        origin.z = self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z;

        direction.x = self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z;
        direction.z = self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z;

        let rotated_ray = Ray::new(origin, direction, ray.time);

        // Determine where (if any) an intersection occurs in object space
        if let Some(mut hit_object) = self.object.hit(rotated_ray, ray_t) {
            // Change the intersection point from object space to world space
            let mut point = hit_object.point;
            point.x =  self.cos_theta * hit_object.point.x + self.sin_theta * hit_object.point.z;
            point.z = -self.sin_theta * hit_object.point.x + self.cos_theta * hit_object.point.z;

            // Change the normal from object space to world space
            let mut normal = hit_object.normal;
            normal.x =  self.cos_theta * hit_object.normal.x + self.sin_theta * hit_object.normal.z;
            normal.z = -self.sin_theta * hit_object.normal.x + self.cos_theta * hit_object.normal.z;

            hit_object.point = point;
            hit_object.normal = normal;

            return Some(hit_object);
        }

        None
    }

    fn get_bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
