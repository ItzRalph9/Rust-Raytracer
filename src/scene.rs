use nalgebra::Vector3;
use once_cell::sync::Lazy;
use std::sync::RwLock;

use crate::{camera::{Camera, CameraDefaults}, color::Color, hit_object::HitObject, material::Material, ray::Ray, sphere::Sphere};

pub struct Scene {
    pub hittable_list: Vec<Sphere>,
    pub camera: Camera,
}

impl Scene {
    pub fn hit(&self, ray: Ray) -> Option<HitObject> {
        Ray::cast_ray(ray, &self.hittable_list)
    }

    pub fn new() -> Self {
        let mut hittable_list: Vec<Sphere> = Vec::new();

        hittable_list.push(Sphere {
            center: Vector3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Material::Lambertian(Color::new(0.5, 0.5, 0.5)),
        });
        
        for a in -11..11 {
            for b in -11..11 {
                let choose_material = Material::random_float();
                let center = Vector3::new(a as f64 + 0.9 * Material::random_float(), 0.2, b as f64 + 0.9 * Material::random_float());
    
                if (center - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                    let material;
    
                    if choose_material < 0.8 {
                        // diffuse
                        let albedo = Color::random() * Color::random();
                        material = Material::Lambertian(albedo);
                        hittable_list.push(Sphere {center, radius: 0.2, material});
                    } else if choose_material < 0.95 {
                        // metal
                        let albedo = Color::random_range(0.5..1.0);
                        let fuzz = Material::random_float_range(0.0..0.5);
                        material = Material::Metal(albedo, fuzz);
                        hittable_list.push(Sphere {center, radius: 0.2, material});
                    } else {
                        // glass
                        material = Material::Dielectric(1.5);
                        hittable_list.push(Sphere {center, radius: 0.2, material});
                    }
                }
            }
        }

        hittable_list.push(Sphere {
            center: Vector3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Dielectric(1.5),
        });

        hittable_list.push(Sphere {
            center: Vector3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Lambertian(Color::new(0.4, 0.2, 0.1)),
        });

        hittable_list.push(Sphere {
            center: Vector3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Material::Metal(Color::new(0.7, 0.6, 0.5), 0.0),
        });

        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 500,
                    max_depth: 50,
                    vertical_fov: 20.0,
                    lookfrom: Vector3::new(13.0, 2.0, 3.0),
                    lookat : Vector3::new(0.0, 0.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.6,
                    focus_distance: 10.0,
                }
            ),
        }
    }
    
    pub fn get_sphere_position(&mut self, sphere_id : Option<usize>) -> Vector3<f64> {
        let sphere_id = match sphere_id {
            Some(id) => id,
            None => 1,
        };

        self.hittable_list[sphere_id].center
    }

    pub fn set_sphere_position(&mut self, position: Vector3<f64>, sphere_id : Option<usize>) {
        let sphere_id = match sphere_id {
            Some(id) => id,
            None => 1,
        };

        self.hittable_list[sphere_id].center = position;
    }
}

pub static SCENE: Lazy<RwLock<Scene>> = Lazy::new(|| RwLock::new(Scene::new()));
