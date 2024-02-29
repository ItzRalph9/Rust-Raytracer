use nalgebra::Vector3;
use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::{hit_object::HitObject, ray::Ray, sphere::Sphere, material::Material, color::Color};
use crate::constants::{CAMERA_POSITION, CAMERA_FOCAL_LENGTH};

pub struct Scene {
    pub hittable_list: Vec<Sphere>,
    pub camera: Camera,
}

impl Scene {
    pub fn hit(&self, ray: Ray) -> Option<HitObject> {
        Ray::cast_ray(ray, &self.hittable_list)
    }

    pub fn new() -> Self {
        Scene {
            hittable_list: vec![
                Sphere{
                    center: Vector3::new(0.0, -100.5, -1.0),
                    radius: 100.0,
                    material: Material::Lambertian(Color::new(0.8, 0.8, 0.0)),
                },
                Sphere {
                    center: Vector3::new(0.0, 0.0, -1.0),
                    radius: 0.5,
                    material: Material::Lambertian(Color::new(0.7, 0.3, 0.3)),
                },
                Sphere{
                    center: Vector3::new(-1.0, 0.0, -1.0),
                    radius: 0.5,
                    material: Material::Metal(Color::new(0.8, 0.8, 0.8), 0.3),
                },
                Sphere{
                    center: Vector3::new(1.0, 0.0, -1.0),
                    radius: 0.5,
                    material: Material::Metal(Color::new(0.8, 0.6, 0.2), 1.0),
                },
            ],
            camera: Camera {
                position: CAMERA_POSITION,
                focal_lenght: CAMERA_FOCAL_LENGTH,  
            },
        }
    }
    
    pub fn get_sphere_position(&mut self) -> Vector3<f64> {
        self.hittable_list[0].center
    }

    pub fn set_sphere_position(&mut self, position: Vector3<f64>) {
        self.hittable_list[0].center = position;
    }
}

pub static SCENE: Lazy<RwLock<Scene>> = Lazy::new(|| RwLock::new(Scene::new()));

pub struct Camera {
    pub position: Vector3<f64>,
    pub focal_lenght: f64,
}
