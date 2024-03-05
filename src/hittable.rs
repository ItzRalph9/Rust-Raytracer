use crate::basic_lib::*;
use crate::{sphere::Sphere, quad::Quad};

pub trait HittableTrait: Sync + Send {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject>;

    fn get_bounding_box(&self) -> Aabb;
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Hittable {
    Sphere(Sphere),
    Quad(Quad),
    // Bvh(Box<BvhNode>)
}

impl Hittable {
    pub fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, ray_t),
            Hittable::Quad(quad) => quad.hit(ray, ray_t),
            // Hittable::Bvh(bvh) => bvh.hit(ray, ray_t),
        }
    }

    pub fn get_bounding_box(&self) -> Aabb {
        match self {
            Hittable::Sphere(sphere) => sphere.get_bounding_box(),
            Hittable::Quad(quad) => quad.get_bounding_box(),
            // Hittable::Bvh(bvh) => bvh.get_bounding_box(),
        }
    }
}