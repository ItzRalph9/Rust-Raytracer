use crate::library::basic_lib::*;
use crate::library::rotate_y::RotateY;
use crate::library::{sphere::Sphere, quad::Quad, translate::Translate, quadbox::Quadbox};

use super::constant_medium::ConstantMedium;
use super::triangle::Triangle;

pub trait HittableTrait: Sync + Send {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject>;

    fn get_bounding_box(&self) -> Aabb;
}

#[derive(Debug, Clone)]
pub enum Hittable {
    Sphere(Sphere),
    Quad(Quad),
    QuadBox(Quadbox),
    Translate(Box<Translate>),
    RotateY(Box<RotateY>),
    ConstantMedium(ConstantMedium),
    Triangle(Triangle),
    // Bvh(Box<BvhNode>)
}

impl Hittable {
    pub fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitObject> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, ray_t),
            Hittable::Quad(quad) => quad.hit(ray, ray_t),
            Hittable::QuadBox(quad_box) => quad_box.hit(ray, ray_t),
            Hittable::Translate(translate) => translate.hit(ray, ray_t),
            Hittable::RotateY(rotate_y) => rotate_y.hit(ray, ray_t),
            Hittable::ConstantMedium(medium) => medium.hit(ray, ray_t),
            Hittable::Triangle(triangle) => triangle.hit(ray, ray_t),
            // Hittable::Bvh(bvh) => bvh.hit(ray, ray_t),
        }
    }

    pub fn get_bounding_box(&self) -> Aabb {
        match self {
            Hittable::Sphere(sphere) => sphere.get_bounding_box(),
            Hittable::Quad(quad) => quad.get_bounding_box(),
            Hittable::QuadBox(quad_box) => quad_box.get_bounding_box(),
            Hittable::Translate(translate) => translate.get_bounding_box(),
            Hittable::RotateY(rotate_y) => rotate_y.get_bounding_box(),
            Hittable::ConstantMedium(medium) => medium.get_bounding_box(),
            Hittable::Triangle(triangle) => triangle.get_bounding_box(),
            // Hittable::Bvh(bvh) => bvh.get_bounding_box(),
        }
    }
}