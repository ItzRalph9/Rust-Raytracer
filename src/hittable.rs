use crate::{aabb::Aabb, hit_object::HitObject, interval::Interval, ray::Ray, sphere::Sphere};

pub trait HittableTrait: Sync + Send {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject>;

    fn get_bounding_box(&self) -> Aabb;
}

#[derive(Debug, Clone)]
pub enum Hittable {
    Sphere(Box<Sphere>),
    // Bvh(Box<BvhNode>)
}

impl Hittable {
    pub fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, ray_t),
            // Hittable::Bvh(bvh) => bvh.hit(ray, ray_t),
        }
    }

    pub fn get_bounding_box(&self) -> Aabb {
        match self {
            Hittable::Sphere(sphere) => sphere.get_bounding_box(),
            // Hittable::Bvh(bvh) => bvh.get_bounding_box(),
        }
    }
}