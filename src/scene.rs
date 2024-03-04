use std::sync::Arc;

use nalgebra::Vector3;
use once_cell::sync::Lazy;

use crate::{bvh::BvhNode, camera::{Camera, CameraDefaults}, color::Color, hittable_list::HittableList, image::Image, material::Material, sphere::Sphere, texture::Texture};

pub struct Scene {
    pub hittable_list: HittableList,
    pub camera: Camera,
}

impl Scene {
    pub fn new(scene: usize) -> Self {
        match scene {
            1 => Self::random_spheres(),
            2 => Self::two_spheres(),
            3 => Self::earth(),
            _ => Self::random_spheres(),
        }
    }
    
    pub fn _get_sphere_position(&mut self, _sphere_id : Option<usize>) -> Vector3<f64> {
        // let sphere_id = match sphere_id {
        //     Some(id) => id,
        //     None => 1,
        // };

        // self.hittable_list.spheres[sphere_id].get_sphere_center(0.0)
        Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn _set_sphere_position(&mut self, _position: Vector3<f64>, _sphere_id : Option<usize>) {
        // let sphere_id = match sphere_id {
        //     Some(id) => id,
        //     None => 1,
        // };

        // self.hittable_list.spheres[sphere_id]._set_sphere_center(position);
    }

    fn random_spheres() -> Self {
        let mut hittable_list = HittableList::new();

        let texture = Texture::Checkered(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

        hittable_list.add(Arc::new(Sphere::new_stationary(
            Vector3::new(0.0, -1000.0, 0.0),
            1000.0,
            Material::Lambertian(texture),
        )));
        
        for a in (-5..5).step_by(3) {
            for b in (-5..5).step_by(3) {
                let choose_material = Material::random_float();
                let center = Vector3::new(a as f64 + 0.9 * Material::random_float(), 0.2, b as f64 + 0.9 * Material::random_float());
    
                if (center - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                    let material;
    
                    if choose_material < 0.8 {
                        // diffuse
                        let albedo = Texture::SolidColor(Color::random() * Color::random());
                        material = Material::Lambertian(albedo);
                        let center2 = center + Vector3::new(0.0, Material::random_float_range(0.0..0.5), 0.0);
                        hittable_list.add(Arc::new(Sphere::new_moving(center, center2, 0.2, material)));
                        // hittable_list.add(Arc::new(Sphere::new_stationary(center, 0.2, material)));
                    } else if choose_material < 0.95 {
                        // metal
                        let albedo = Color::random_range(0.5..1.0);
                        let fuzz = Material::random_float_range(0.0..0.5);
                        material = Material::Metal(albedo, fuzz);
                        hittable_list.add(Arc::new(Sphere::new_stationary(center, 0.2, material)));
                    } else {
                        // glass
                        material = Material::Dielectric(1.5);
                        hittable_list.add(Arc::new(Sphere::new_stationary(center, 0.2, material)));
                    }
                }
            }
        }

        hittable_list.add(Arc::new(Sphere::new_stationary(
            Vector3::new(0.0, 1.0, 0.0),
            1.0,
            Material::Dielectric(1.5),
        )));

        hittable_list.add(Arc::new(Sphere::new_stationary(
            Vector3::new(-4.0, 1.0, 0.0),
            1.0,
            Material::Lambertian(Texture::SolidColor(Color::new(0.4, 0.2, 0.1))),
        )));

        hittable_list.add(Arc::new(Sphere::new_stationary(
            Vector3::new(4.0, 1.0, 0.0),
            1.0,
            Material::Metal(Color::new(0.4, 0.4, 0.4), 0.025),
        )));

        // let list = HittableList::new_from_list(Arc::new(BvhNode::new_from_list(hittable_list)));
        let list = hittable_list;

        Scene {
            hittable_list: list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    vertical_fov: 20.0,
                    lookfrom: Vector3::new(13.0, 2.0, 3.0),
                    lookat : Vector3::new(0.0, 0.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.15,
                    focus_distance: 10.0,
                }
            ),
        }
    }

    fn two_spheres() -> Self {
        let mut hittable_list = HittableList::new();
    
        let checker = Texture::Checkered(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    
        hittable_list.add(Arc::new(Sphere::new_stationary(Vector3::new(0.0, -10.0, 0.0), 10.0, Material::Lambertian(checker.clone()))));
        hittable_list.add(Arc::new(Sphere::new_stationary(Vector3::new(0.0, 10.0, 0.0), 10.0, Material::Lambertian(checker.clone()))));
    
        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    vertical_fov: 20.0,
                    lookfrom: Vector3::new(13.0, 2.0, 3.0),
                    lookat : Vector3::new(0.0, 0.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }

    fn earth() -> Self {
        let mut hittable_list = HittableList::new();
    
        let texture = Texture::Image(Image::load_image("images/earth_true_color.png"));
        let surface = Material::Lambertian(texture);
    
        hittable_list.add(Arc::new(Sphere::new_stationary(Vector3::new(0.0, 0.0, 0.0), 2.0, surface)));
    
        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    vertical_fov: 20.0,
                    lookfrom: Vector3::new(0.0, 0.0, 12.0),
                    lookat : Vector3::new(0.0, 0.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }
}

pub static SCENE: Lazy<Scene> = Lazy::new(|| Scene::new(3));
