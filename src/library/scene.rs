use std::sync::RwLock;

use nalgebra::Vector3;
use once_cell::sync::Lazy;

use crate::library::{camera::{Camera, CameraDefaults}, color::Color, image::Image, material::Material, perlin::Perlin};
use crate::library::{hittable_list::HittableList, quad::Quad, rotate_y::RotateY, translate::Translate, sphere::Sphere, quadbox::Quadbox};
use crate::library::material::Material::*;
use crate::library::texture::Texture::*;
use crate::library::hittable::Hittable::*;

use super::constant_medium::ConstantMedium;

pub static SCENE: Lazy<RwLock<Scene>> = Lazy::new(|| RwLock::new(Scene::new(8)));

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
            4 => Self::two_perlin_spheres(),
            5 => Self::quads(),
            6 => Self::simple_light(),
            7 => Self::cornell_box(),
            8 => Self::cornell_smoke(),
            _ => Self::random_spheres(),
        }
    }
    
    pub fn get_sphere_position(&mut self, sphere_id : Option<usize>) -> Vector3<f64> {
        let last_item_in_scene = self.hittable_list.objects.len() - 1;
        let sphere_id = sphere_id.unwrap_or(last_item_in_scene);

        let hittable = &self.hittable_list.objects[sphere_id];

        match &hittable {
            Sphere(sphere) => sphere.get_sphere_center(0.0),
            _ => Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn set_sphere_position(&mut self, position: Vector3<f64>, sphere_id : Option<usize>) {
        let last_item_in_scene = self.hittable_list.objects.len() - 1;
        let sphere_id = sphere_id.unwrap_or(last_item_in_scene);

        if let Sphere(sphere) = &mut self.hittable_list.objects[sphere_id] {
            sphere.set_sphere_center(position);
        }
    }

    fn random_spheres() -> Self {
        let mut hittable_list = HittableList::new();

        let ground_texture = Checkered(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian(ground_texture),
        )));
        
        for a in (-7..7).step_by(4) {
            for b in (-5..5).step_by(2) {
                let choose_material = Material::random_float();
                let center = Vector3::new(a as f64 + 0.9 *  Material::random_float(), 0.2, b as f64 + 0.9 *  Material::random_float());
    
                if (center - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                    let material;
    
                    if choose_material < 0.8 {
                        // diffuse
                        // let albedo = SolidColor(Color::random() * Color::random());
                        // material = Lambertian(albedo);
                        let texture = Image(Image::load_image("assets/earth_1024.jpg"));
                        let material = Lambertian(texture);
                        // let center2 = center + Vector3::new(0.0,  Material::random_float_range(0.0..0.5), 0.0);
                        // hittable_list.add(Arc::new(Sphere::new_moving(center, center2, 0.2, material)));
                        hittable_list.add(Sphere(Sphere::new_stationary(center, 0.2, material)));
                    } else if choose_material < 0.95 {
                        // metal
                        let albedo = Color::random_range(0.5..1.0);
                        let fuzz =  Material::random_float_range(0.0..0.5);
                        material = Metal(albedo, fuzz);
                        hittable_list.add(Sphere(Sphere::new_stationary(center, 0.2, material)));
                    } else {
                        // glass
                        material = Dielectric(1.5);
                        hittable_list.add(Sphere(Sphere::new_stationary(center, 0.2, material)));
                    }
                }
            }
        }

        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(2.0, 1.0, 0.0),
            1.0,
            Dielectric(1.5),
        )));

        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(-2.0, 1.0, 0.0),
            1.0,
            Lambertian(SolidColor(Color::new(0.4, 0.2, 0.1))),
        )));

        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(6.0, 1.0, 0.0),
            1.0,
            Metal(Color::new(0.4, 0.4, 0.4), 0.025),
        )));

        let earth_texture = Image(Image::load_image("assets/earth_1024.jpg"));
        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(-6.0, 1.0, 0.0),
            1.0,
            Lambertian(earth_texture),
        )));

        // let list = HittableList::new_from_list(Arc::new(BvhNode::new_from_list(hittable_list)));
        let list = hittable_list;

        Scene {
            hittable_list: list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.7, 0.8, 1.0),
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
    
        let checker = Checkered(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    
        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(0.0, -10.0, 0.0),
            10.0,
            Lambertian(checker.clone())
        )));
        
        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(0.0, 10.0, 0.0),
            10.0,
            Lambertian(checker.clone())
        )));
    
        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.7, 0.8, 1.0),
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
    
        let texture = Image(Image::load_image("assets/earth_1024.jpg"));
        let surface = Lambertian(texture);
    
        hittable_list.add(Sphere(Sphere::new_stationary(Vector3::new(0.0, 0.0, 0.0), 2.0, surface)));
    
        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.7, 0.8, 1.0),
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

    fn two_perlin_spheres() -> Self {
        let mut hittable_list = HittableList::new();
    
        let perlin_texture = Perlin(Perlin::new(), 4.0);

        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian(perlin_texture.clone())
        )));
        
        hittable_list.add(Sphere(Sphere::new_stationary(
            Vector3::new(0.0, 2.0, 0.0),
            2.0,
            Lambertian(perlin_texture.clone())
        )));

        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.7, 0.8, 1.0),
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

    fn quads() -> Self {
        let mut hittable_list = HittableList::new();
    
        // Materials
        let left_red     = Lambertian(SolidColor(Color::new(1.0, 0.2, 0.2)));
        let back_green   = Lambertian(SolidColor(Color::new(0.2, 1.0, 0.2)));
        let right_blue   = Lambertian(SolidColor(Color::new(0.2, 0.2, 1.0)));
        let upper_orange = Lambertian(SolidColor(Color::new(1.0, 0.5, 0.0)));
        let lower_teal   = Lambertian(SolidColor(Color::new(0.2, 0.8, 0.8)));
    
        // Quads
        hittable_list.add(Quad(Quad::new(Vector3::new(-3.0, -2.0, 5.0), Vector3::new(0.0, 0.0,-4.0), Vector3::new(0.0, 4.0, 0.0), left_red)));
        hittable_list.add(Quad(Quad::new(Vector3::new(-2.0, -2.0, 0.0), Vector3::new(4.0, 0.0, 0.0), Vector3::new(0.0, 4.0, 0.0), back_green)));
        hittable_list.add(Quad(Quad::new(Vector3::new( 3.0, -2.0, 1.0), Vector3::new(0.0, 0.0, 4.0), Vector3::new(0.0, 4.0, 0.0), right_blue)));
        hittable_list.add(Quad(Quad::new(Vector3::new(-2.0,  3.0, 1.0), Vector3::new(4.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 4.0), upper_orange)));
        hittable_list.add(Quad(Quad::new(Vector3::new(-2.0, -3.0, 5.0), Vector3::new(4.0, 0.0, 0.0), Vector3::new(0.0, 0.0,-4.0), lower_teal)));
        
        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.7, 0.8, 1.0),
                    vertical_fov: 80.0,
                    lookfrom: Vector3::new(0.0, 0.0, 9.0),
                    lookat : Vector3::new(0.0, 0.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }

    fn simple_light() -> Self {
        let mut hittable_list = HittableList::new();
    
        let pertext = Perlin(Perlin::new(), 4.0);
        hittable_list.add(Sphere(Sphere::new_stationary(Vector3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian(pertext.clone()))));
        hittable_list.add(Sphere(Sphere::new_stationary(Vector3::new(0.0, 2.0, 0.0), 2.0, Lambertian(pertext.clone()))));
    
        let difflight = DiffuseLight(SolidColor(Color::new(20.0, 20.0, 20.0)));
        hittable_list.add(Quad(Quad::new(Vector3::new(3.0, 1.0, -2.0), Vector3::new(2.0, 0.0, 0.0), Vector3::new(0.0, 2.0, 0.0), difflight.clone())));
        hittable_list.add(Sphere(Sphere::new_stationary(Vector3::new(0.0, 7.0, 0.0), 2.0, difflight.clone())));

        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.0, 0.0, 0.0),
                    vertical_fov: 20.0,
                    lookfrom: Vector3::new(26.0, 3.0, 6.0),
                    lookat : Vector3::new(0.0, 2.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }

    fn cornell_box() -> Self {
        let mut hittable_list = HittableList::new();
    
        let red   = Lambertian(SolidColor(Color::new(0.65, 0.05, 0.05)));
        let white = Lambertian(SolidColor(Color::new(0.73, 0.73, 0.73)));
        let green = Lambertian(SolidColor(Color::new(0.12, 0.45, 0.15)));
        let light = DiffuseLight(SolidColor(Color::new(15.0, 15.0, 15.0)));
    
        hittable_list.add(Quad(Quad::new(Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), green.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), red.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(343.0, 554.0, 332.0), Vector3::new(-130.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -105.0), light.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 555.0), white.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(555.0, 555.0, 555.0), Vector3::new(-555.0, 0.0, 0.0), Vector3::new(0.0, 0.0,-555.0), white.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 555.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), white.clone())));

        let box1 = Quadbox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(165.0, 330.0, 165.0), white.clone());
        for side in box1.objects {
            let side = RotateY::new(side, 15.0);
            let side = RotateY(Box::new(side));
            let side = Translate::new(side, Vector3::new(265.0, 0.0, 295.0));
            let side = Translate(Box::new(side));
            hittable_list.add(side)
        }

        let box2 = Quadbox::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(165.0, 165.0, 165.0), white.clone());
        for side in box2.objects {
            let side = RotateY::new(side, -18.0);
            let side = RotateY(Box::new(side));
            let side = Translate::new(side, Vector3::new(130.0, 0.0, 65.0));
            let side = Translate(Box::new(side));
            hittable_list.add(side)
        }

        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.0, 0.0, 0.0),
                    vertical_fov: 40.0,
                    lookfrom: Vector3::new(278.0, 278.0, -800.0),
                    lookat : Vector3::new(278.0, 278.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }
    
    fn cornell_smoke() -> Self {
        let mut hittable_list = HittableList::new();
    
        let red   = Lambertian(SolidColor(Color::new(0.65, 0.05, 0.05)));
        let white = Lambertian(SolidColor(Color::new(0.73, 0.73, 0.73)));
        let green = Lambertian(SolidColor(Color::new(0.12, 0.45, 0.15)));
        let light = DiffuseLight(SolidColor(Color::new(7.0, 7.0, 7.0)));
    
        hittable_list.add(Quad(Quad::new(Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), green.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), red.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(113.0, 554.0, 127.0), Vector3::new(330.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 305.0), light.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 555.0, 0.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 555.0), white.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 555.0), white.clone())));
        hittable_list.add(Quad(Quad::new(Vector3::new(0.0, 0.0, 555.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), white.clone())));

        let box1 = Translate(Box::new(
            Translate::new(RotateY(Box::new(RotateY::new(
                QuadBox(Quadbox::new(
                    Vector3::new(0.0, 0.0, 0.0),
                    Vector3::new(165.0, 330.0, 165.0),
                    white.clone()
                )),
                15.0
            ))), Vector3::new(265.0, 0.0, 295.0))
        ));

        let box2 = Translate(Box::new(
            Translate::new(RotateY(Box::new(RotateY::new(
                QuadBox(Quadbox::new(
                    Vector3::new(0.0, 0.0, 0.0),
                    Vector3::new(165.0, 165.0, 165.0),
                    white.clone()
                )), -18.0
            ))), Vector3::new(103.0, 0.0, 65.0))
        ));

        hittable_list.add(ConstantMedium(ConstantMedium::new_from_color(box1, 0.01, Color::new(0.0, 0.0, 0.0))));
        hittable_list.add(ConstantMedium(ConstantMedium::new_from_color(box2, 0.005, Color::new(1.0, 1.0, 1.0))));

        Scene {
            hittable_list,
            camera: Camera::init(
                CameraDefaults {
                    samples_per_pixel: 1,
                    max_depth: 50,
                    background: Color::new(0.0, 0.0, 0.0),
                    vertical_fov: 40.0,
                    lookfrom: Vector3::new(278.0, 278.0, -800.0),
                    lookat : Vector3::new(278.0, 278.0, 0.0),
                    vup: Vector3::new(0.0, 1.0, 0.0),
                    defocus_angle: 0.0,
                    focus_distance: 10.0,
                }
            ),
        }
    }
}
