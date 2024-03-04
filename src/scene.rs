use std::sync::RwLock;

use nalgebra::Vector3;
use once_cell::sync::Lazy;

use crate::{camera::{Camera, CameraDefaults}, color::Color, hittable::Hittable, hittable_list::HittableList, image::Image, material::Material, sphere::Sphere, texture::Texture};

pub static SCENE: Lazy<RwLock<Scene>> = Lazy::new(|| RwLock::new(Scene::new(1)));

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
    
    pub fn get_sphere_position(&mut self, sphere_id : Option<usize>) -> Result<Vector3<f64>, ()> {
        let sphere_id = match sphere_id {
            Some(id) => id,
            None => self.hittable_list.objects.len() - 1,
        };

        let hittable = &self.hittable_list.objects[sphere_id];

        match &hittable {
            Hittable::Sphere(sphere) => Ok(sphere.get_sphere_center(0.0)),
            _ => panic!()
        }
    }

    pub fn set_sphere_position(&mut self, position: Vector3<f64>, sphere_id : Option<usize>) {
        let sphere_id = match sphere_id {
            Some(id) => id,
            None => self.hittable_list.objects.len() - 1, // last item in the scene
        };

        if let Hittable::Sphere(sphere) = &mut self.hittable_list.objects[sphere_id] {
            // Call set_sphere_center on the mutable reference to sphere
            sphere.set_sphere_center(position);
        }
    }

    fn random_spheres() -> Self {
        let mut hittable_list = HittableList::new();

        let ground_texture = Texture::Checkered(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

        hittable_list.add(Hittable::Sphere(Box::new(Sphere::new_stationary(
            Vector3::new(0.0, -1000.0, 0.0),
            1000.0,
            Material::Lambertian(ground_texture),
        ))));
        
        for a in (-7..7).step_by(4) {
            for b in (-5..5).step_by(2) {
                let choose_material = Material::random_float();
                let center = Vector3::new(a as f64 + 0.9 * Material::random_float(), 0.2, b as f64 + 0.9 * Material::random_float());
    
                if (center - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                    let material;
    
                    if choose_material < 0.8 {
                        // diffuse
                        // let albedo = Texture::SolidColor(Color::random() * Color::random());
                        // material = Material::Lambertian(albedo);
                        let texture = Texture::Image(Image::load_image("assets/earth_1024.jpg"));
                        let material = Material::Lambertian(texture);
                        // let center2 = center + Vector3::new(0.0, Material::random_float_range(0.0..0.5), 0.0);
                        // hittable_list.add(Arc::new(Sphere::new_moving(center, center2, 0.2, material)));
                        hittable_list.add(Hittable::Sphere(Box::new(Sphere::new_stationary(center, 0.2, material))));
                    } else if choose_material < 0.95 {
                        // metal
                        let albedo = Color::random_range(0.5..1.0);
                        let fuzz = Material::random_float_range(0.0..0.5);
                        material = Material::Metal(albedo, fuzz);
                        hittable_list.add(Hittable::Sphere(Box::new(Sphere::new_stationary(center, 0.2, material))));
                    } else {
                        // glass
                        material = Material::Dielectric(1.5);
                        hittable_list.add(Hittable::Sphere(Box::new(Sphere::new_stationary(center, 0.2, material))));
                    }
                }
            }
        }

        hittable_list.add(Hittable::Sphere(Box::new(Sphere::new_stationary(
            Vector3::new(2.0, 1.0, 0.0),
            1.0,
            Material::Dielectric(1.5),
        ))));

        hittable_list.add(Hittable::Sphere(Box::new(Sphere::new_stationary(
            Vector3::new(-2.0, 1.0, 0.0),
            1.0,
            Material::Lambertian(Texture::SolidColor(Color::new(0.4, 0.2, 0.1))),
        ))));

        hittable_list.add(Hittable::Sphere(Box::new(Sphere::new_stationary(
            Vector3::new(6.0, 1.0, 0.0),
            1.0,
            Material::Metal(Color::new(0.4, 0.4, 0.4), 0.025),
        ))));

        let earth_texture = Texture::Image(Image::load_image("assets/earth_1024.jpg"));
        hittable_list.add(Hittable::Sphere(Box::new(Sphere::new_stationary(
            Vector3::new(-6.0, 1.0, 0.0),
            1.0,
            Material::Lambertian(earth_texture),
        ))));

        // let list = HittableList::new_from_list(Arc::new(BvhNode::new_from_list(hittable_list)));
        let list = hittable_list;

        Scene {
            hittable_list: list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    vertical_fov: 20.0,
                    lookfrom: Vector3::new(15.0, 2.5, 8.0),
                    lookat : Vector3::new(0.0, 0.0, -1.0),
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
    
        hittable_list.add(Hittable::Sphere(Box::new(Sphere::new_stationary(Vector3::new(0.0, -10.0, 0.0), 10.0, Material::Lambertian(checker.clone())))));
        hittable_list.add(Hittable::Sphere(Box::new(Sphere::new_stationary(Vector3::new(0.0, 10.0, 0.0), 10.0, Material::Lambertian(checker.clone())))));
    
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
    
        let texture = Texture::Image(Image::load_image("assets/earth_1024.jpg"));
        let surface = Material::Lambertian(texture);
    
        hittable_list.add(Hittable::Sphere(Box::new(Sphere::new_stationary(Vector3::new(0.0, 0.0, 0.0), 2.0, surface))));
    
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
