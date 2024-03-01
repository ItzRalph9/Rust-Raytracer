use nalgebra::Vector3;
use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::{hit_object::HitObject, ray::Ray, sphere::Sphere, material::Material, color::Color};
use crate::constants::{WIDTH, HEIGHT};

pub struct Scene {
    pub hittable_list: Vec<Sphere>,
    pub camera: Camera,
    pub viewport: Viewport,
}

impl Scene {
    pub fn hit(&self, ray: Ray) -> Option<HitObject> {
        Ray::cast_ray(ray, &self.hittable_list)
    }

    pub fn new() -> Self {
        let default_camera = Camera {
            position:  Vector3::new(0.0, 0.0, 0.0),
            focal_lenght: 1.0,  
        };

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
                    material: Material::Lambertian(Color::new(0.1, 0.2, 0.5)),
                },
                Sphere{
                    center: Vector3::new(-1.0, 0.0, -1.0),
                    radius: 0.5,
                    material: Material::Dielectric(1.5),
                },
                Sphere{
                    center: Vector3::new(-1.0, 0.0, -1.0),
                    radius: -0.4,
                    material: Material::Dielectric(1.5),
                },
                Sphere{
                    center: Vector3::new(1.0, 0.0, -1.0),
                    radius: 0.5,
                    material: Material::Metal(Color::new(0.8, 0.6, 0.2), 0.0),
                },
            ],
            camera: default_camera,
            viewport: Viewport::init(default_camera),
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

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Vector3<f64>,
    pub focal_lenght: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub viewport_u: Vector3<f64>,
    pub viewport_v: Vector3<f64>,

    pub pixel_delta_u: Vector3<f64>,
    pub pixel_delta_v: Vector3<f64>,
    pub pixel00_loc: Vector3<f64>,
}

impl Viewport {
    pub fn init(camera: Camera) -> Self {
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (WIDTH as f64 / HEIGHT as f64);

        let viewport_u: Vector3<f64> = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vector3<f64> = Vector3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / WIDTH as f64;
        let pixel_delta_v = viewport_v / HEIGHT as f64;

        let viewport_upper_left = camera.position - Vector3::new(0.0, 0.0, camera.focal_lenght)
                - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Viewport {
            viewport_u, viewport_v,
            pixel_delta_u, pixel_delta_v,
            pixel00_loc
        }
    }
}
