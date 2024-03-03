use crate::{ray::Ray, interval::Interval, hit_object::HitObject, aabb::Aabb};

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject>;

    fn get_bounding_box(&self) -> Aabb;
}